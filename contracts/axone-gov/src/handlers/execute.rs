use crate::{
    contract::{AxoneGov, AxoneGovResult},
    domain::{Case, Constitution},
    error::AxoneGovError,
    gateway::logic::{query_service_ask, AxoneLogicQuery, QueryServiceAskRequest},
    msg::AxoneGovExecuteMsg,
    queries::decision::{build_decide_query, build_decide_query_with_motivation},
    services::decision::{
        build_governance_case, decide_case_with_motivation, module_term, tx_term,
    },
    state::{load_constitution, load_constitution_status, save_revised_constitution},
    GOV_CTX_MODULE, GOV_CTX_TX, GOV_INTENT_ESTABLISH, GOV_INTENT_REVISE_CONSTITUTION,
    GOV_VERDICT_PERMITTED, RESPONSE_KEY_CASE_HASH, RESPONSE_KEY_CONSTITUTION_HASH,
    RESPONSE_KEY_CONSTITUTION_REVISER, RESPONSE_KEY_CONSTITUTION_REVISION,
    RESPONSE_KEY_DECISION_ID, RESPONSE_KEY_MOTIVATION_HASH, RESPONSE_KEY_VERDICT,
    RESPONSE_KEY_VERDICT_HASH,
};

use crate::domain::Decision;
use crate::prolog::term as t;
use crate::state::record_decision;
use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{Binary, DepsMut, Env, MessageInfo, QuerierWrapper};

#[allow(clippy::unnecessary_wraps)]
pub fn execute_handler(
    deps: DepsMut<'_>,
    env: Env,
    info: MessageInfo,
    module: AxoneGov,
    msg: AxoneGovExecuteMsg,
) -> AxoneGovResult {
    match msg {
        AxoneGovExecuteMsg::ReviseConstitution { constitution, case } => {
            execute_revise_constitution(deps, env, info, module, constitution, case)
        }
        AxoneGovExecuteMsg::RecordDecision { case, motivated } => {
            execute_record_decision(deps, env, info, module, case, motivated.unwrap_or(false))
        }
    }
}

fn execute_revise_constitution(
    deps: DepsMut<'_>,
    env: Env,
    info: MessageInfo,
    module: AxoneGov,
    constitution_bytes: Binary,
    case_input: Option<String>,
) -> AxoneGovResult {
    let querier = QuerierWrapper::<AxoneLogicQuery>::new(&*deps.querier);
    let revised_constitution = Constitution::try_new(constitution_bytes, &querier)?;
    let current_constitution = load_constitution(deps.storage)?;
    let current_status = load_constitution_status(deps.storage)?;

    let current_case = build_governance_case(
        case_input.as_deref(),
        GOV_INTENT_REVISE_CONSTITUTION,
        &revised_constitution,
        Some(&current_status),
        &module,
        &env,
        &info,
    )?;
    let current_decision =
        decide_case_with_motivation(&querier, current_constitution.source(), &current_case)?;

    if current_decision.verdict != t::atom(GOV_VERDICT_PERMITTED) {
        return Err(AxoneGovError::DecisionRefused {
            intent: GOV_INTENT_REVISE_CONSTITUTION.to_string(),
            verdict: current_decision.verdict.to_string(),
            motivation: current_decision.motivation.to_string(),
        });
    }

    let proposed_case = build_governance_case(
        case_input.as_deref(),
        GOV_INTENT_ESTABLISH,
        &revised_constitution,
        Some(&current_status),
        &module,
        &env,
        &info,
    )?;
    let proposed_decision =
        decide_case_with_motivation(&querier, revised_constitution.source(), &proposed_case)?;

    if proposed_decision.verdict != t::atom(GOV_VERDICT_PERMITTED) {
        return Err(AxoneGovError::DecisionRefused {
            intent: GOV_INTENT_ESTABLISH.to_string(),
            verdict: proposed_decision.verdict.to_string(),
            motivation: proposed_decision.motivation.to_string(),
        });
    }

    let status = save_revised_constitution(deps.storage, &revised_constitution)?;

    Ok(module.custom_response(
        "revise_constitution",
        vec![
            (
                RESPONSE_KEY_CONSTITUTION_REVISION.to_string(),
                status.constitution_revision().to_string(),
            ),
            (
                RESPONSE_KEY_CONSTITUTION_HASH.to_string(),
                status.constitution_hash_hex(),
            ),
            (
                RESPONSE_KEY_CONSTITUTION_REVISER.to_string(),
                info.sender.to_string(),
            ),
        ],
    ))
}

fn execute_record_decision(
    deps: DepsMut<'_>,
    env: Env,
    info: MessageInfo,
    module: AxoneGov,
    case_input: String,
    motivated: bool,
) -> AxoneGovResult {
    let mut case = Case::new(&case_input)?;
    let enrichment_term = t::dict(
        "ctx",
        vec![
            t::kv(GOV_CTX_MODULE, module_term(&module)),
            t::kv(GOV_CTX_TX, tx_term(&env, &info)),
        ],
    );

    let enrichment = Case::try_from(enrichment_term)?;
    case.merge(&enrichment);

    let case_term = case.to_string();

    let constitution = load_constitution(deps.storage)?;
    let status = load_constitution_status(deps.storage)?;
    let program = constitution.source();
    let query = if motivated {
        build_decide_query_with_motivation(&case)
    } else {
        build_decide_query(&case)
    };

    let request = QueryServiceAskRequest::one(program, query);
    let response = query_service_ask(
        &QuerierWrapper::<AxoneLogicQuery>::new(&*deps.querier),
        request,
    )?;
    let answer = response.answer.ok_or(AxoneGovError::PrologEngineNoAnswer)?;

    if let Some(error) = answer
        .results
        .iter()
        .find_map(|result| result.error.as_deref())
    {
        return Err(AxoneGovError::DecisionFailed(error.to_string()));
    }

    let result = answer
        .results
        .first()
        .ok_or(AxoneGovError::DecisionNoResult)?;
    let verdict =
        find_substitution(result, "Verdict").ok_or(AxoneGovError::DecisionMissingVerdict)?;
    let motivation = if motivated {
        Some(
            find_substitution(result, "Motivation")
                .ok_or(AxoneGovError::DecisionMissingMotivation)?,
        )
    } else {
        None
    };

    let decision = Decision::new(
        &status,
        case_term,
        verdict,
        motivation,
        info.sender,
        env.block.height,
        env.block.time.seconds(),
    );

    let decision_record = record_decision(deps.storage, decision)?;
    let mut attrs = vec![
        (
            RESPONSE_KEY_DECISION_ID.to_string(),
            decision_record.id().to_string(),
        ),
        (
            RESPONSE_KEY_CONSTITUTION_REVISION.to_string(),
            decision_record.constitution_revision().to_string(),
        ),
        (
            RESPONSE_KEY_CONSTITUTION_HASH.to_string(),
            decision_record.constitution_hash_hex(),
        ),
        (
            RESPONSE_KEY_CASE_HASH.to_string(),
            decision_record.case_hash_hex(),
        ),
        (
            RESPONSE_KEY_VERDICT.to_string(),
            decision_record.verdict().clone(),
        ),
        (
            RESPONSE_KEY_VERDICT_HASH.to_string(),
            decision_record.verdict_hash_hex(),
        ),
    ];

    if let Some(h) = decision_record.motivation_hash_hex() {
        attrs.push((RESPONSE_KEY_MOTIVATION_HASH.to_string(), h));
    }

    Ok(module.custom_response("record_decision", attrs))
}

fn find_substitution(result: &crate::gateway::logic::Result, variable: &str) -> Option<String> {
    result
        .substitutions
        .iter()
        .find(|sub| sub.variable == variable)
        .map(|sub| sub.expression.clone())
}
