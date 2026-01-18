use crate::{
    contract::{AxoneGov, AxoneGovResult},
    domain::{Case, Constitution},
    error::AxoneGovError,
    gateway::logic::{query_service_ask, AxoneLogicQuery, QueryServiceAskRequest},
    msg::AxoneGovExecuteMsg,
    prolog::ast::Term,
    queries::decision::build_decide_query_with_motivation,
    state::{load_constitution, save_revised_constitution},
};

use crate::prolog::term as t;
use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{Binary, DepsMut, Env, Int64, MessageInfo, QuerierWrapper, StdError, Uint64};

#[allow(clippy::unnecessary_wraps)]
pub fn execute_handler(
    deps: DepsMut<'_>,
    env: Env,
    info: MessageInfo,
    module: AxoneGov,
    msg: AxoneGovExecuteMsg,
) -> AxoneGovResult {
    match msg {
        AxoneGovExecuteMsg::NoOp {} => Ok(module.response("noop")),
        AxoneGovExecuteMsg::ReviseConstitution { constitution, case } => {
            revise_constitution(deps, env, info, module, constitution, case)
        }
    }
}

fn revise_constitution(
    deps: DepsMut<'_>,
    env: Env,
    info: MessageInfo,
    module: AxoneGov,
    constitution_bytes: Binary,
    case_input: Option<String>,
) -> AxoneGovResult {
    let querier = QuerierWrapper::<AxoneLogicQuery>::new(&*deps.querier);
    let revised_constitution = Constitution::try_new(constitution_bytes, &querier)?;

    let mut case = if let Some(input) = case_input {
        Case::new(&input)?
    } else {
        Case::default()
    };

    let enrichment_term = t::dict(
        "ctx",
        vec![
            t::kv("intent", t::atom("gov:revise_constitution")),
            t::kv("cosmwasm", build_cosmwasm_term(&env, &info)?),
        ],
    );

    let enrichment = Case::try_from(enrichment_term)?;
    case.merge(&enrichment);
    let current_constitution = load_constitution(deps.storage)?;
    let program = current_constitution.source();
    let query = build_decide_query_with_motivation(&case);

    let request = QueryServiceAskRequest::one(program, query);
    let response = query_service_ask(&querier, request)?;
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

    let verdict_substitution = result
        .substitutions
        .iter()
        .find(|sub| sub.variable == "Verdict")
        .ok_or(AxoneGovError::DecisionMissingVerdict)?;
    let verdict = verdict_substitution.expression.clone();
    let verdict_term = verdict_substitution.expression_term().map_err(|err| {
        AxoneGovError::PrologEngineError(format!(
            "invalid verdict term at offset {}: {}",
            err.at, err.msg
        ))
    })?;

    let motivation = result
        .substitutions
        .iter()
        .find(|sub| sub.variable == "Motivation")
        .map(|sub| sub.expression.clone())
        .ok_or(AxoneGovError::DecisionMissingMotivation)?;

    let authorized = verdict_term == t::atom("gov:permitted");

    if !authorized {
        return Err(AxoneGovError::RevisionRefused {
            verdict,
            motivation,
        });
    }

    let status = save_revised_constitution(deps.storage, &revised_constitution)?;

    Ok(module.custom_response(
        "revise_constitution",
        vec![
            (
                "constitution_revision".to_string(),
                status.constitution_revision().to_string(),
            ),
            (
                "constitution_hash".to_string(),
                status.constitution_hash_hex(),
            ),
            ("constitution_reviser".to_string(), info.sender.to_string()),
        ],
    ))
}
fn build_cosmwasm_term(env: &Env, info: &MessageInfo) -> AxoneGovResult<Term> {
    let funds = info
        .funds
        .iter()
        .map(|c| -> AxoneGovResult<Term> {
            let amount = Int64::try_from(c.amount).map_err(StdError::from)?;
            Ok(t::compound2("coin", amount.into(), t::atom(c.denom.clone())))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let height = Int64::try_from(Uint64::from(env.block.height)).map_err(StdError::from)?;

    let mut block_entries = vec![
        t::kv("height", height.into()),
        t::kv("time", t::atom(env.block.time.to_string())),
    ];
    if let Some(tx) = &env.transaction {
        block_entries.push(t::kv("tx_index", Int64::from(tx.index).into()));
    }

    Ok(t::dict(
        "cosmwasm",
        vec![
            t::kv(
                "message",
                t::dict(
                    "message",
                    vec![
                        t::kv("sender", t::atom(info.sender.to_string())),
                        t::kv("funds", t::list(funds)),
                    ],
                ),
            ),
            t::kv("block", t::dict("block", block_entries)),
        ],
    ))
}
