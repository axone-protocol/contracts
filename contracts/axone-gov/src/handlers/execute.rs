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

    let enrichment = Case::try_from(Term::Dict(
        "ctx".to_string(),
        vec![
            (
                "intent".to_string(),
                Term::Atom("gov:revise_constitution".to_string()),
            ),
            ("cosmwasm".to_string(), build_cosmwasm_term(&env, &info)?),
        ],
    ))?;
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

    let authorized = verdict_term == Term::Atom("gov:permitted".to_string());

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
            Ok(Term::Compound(
                "-".to_string(),
                vec![Term::Integer(amount), Term::Atom(c.denom.clone())],
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let height = Int64::try_from(Uint64::from(env.block.height)).map_err(StdError::from)?;
    let mut block_entries = vec![
        ("height".to_string(), Term::Integer(height)),
        ("time".to_string(), Term::Atom(env.block.time.to_string())),
    ];
    if let Some(tx) = &env.transaction {
        block_entries.push(("tx_index".to_string(), Term::Integer(Int64::from(tx.index))));
    }

    Ok(Term::Dict(
        "cosmwasm".to_string(),
        vec![
            (
                "message".to_string(),
                Term::Dict(
                    "message".to_string(),
                    vec![
                        ("sender".to_string(), Term::Atom(info.sender.to_string())),
                        ("funds".to_string(), Term::List(funds, None)),
                    ],
                ),
            ),
            (
                "block".to_string(),
                Term::Dict("block".to_string(), block_entries),
            ),
        ],
    ))
}
