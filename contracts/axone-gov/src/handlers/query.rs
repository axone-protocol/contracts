use crate::{
    contract::{AxoneGov, AxoneGovResult},
    domain::Case,
    error::AxoneGovError,
    gateway::logic::{query_service_ask, AxoneLogicQuery, QueryServiceAskRequest},
    msg::{AxoneGovQueryMsg, ConstitutionResponse, ConstitutionStatusResponse, DecideResponse},
    queries::decision::{build_decide_query, build_decide_query_with_motivation},
    state::load_constitution,
};

use crate::state::load_constitution_status;
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, QuerierWrapper};

pub fn query_handler(
    deps: Deps<'_>,
    _env: Env,
    _module: &AxoneGov,
    msg: AxoneGovQueryMsg,
) -> AxoneGovResult<Binary> {
    match msg {
        AxoneGovQueryMsg::Constitution {} => to_json_binary(&query_constitution(deps)?),
        AxoneGovQueryMsg::ConstitutionStatus {} => {
            to_json_binary(&query_constitution_status(deps)?)
        }
        AxoneGovQueryMsg::Decide { case, motivated } => {
            to_json_binary(&query_decide(deps, &case, motivated)?)
        }
    }
    .map_err(Into::into)
}

fn query_constitution(deps: Deps<'_>) -> AxoneGovResult<ConstitutionResponse> {
    Ok(ConstitutionResponse::from(&load_constitution(
        deps.storage,
    )?))
}

fn query_constitution_status(deps: Deps<'_>) -> AxoneGovResult<ConstitutionStatusResponse> {
    Ok(ConstitutionStatusResponse::from(&load_constitution_status(
        deps.storage,
    )?))
}

fn query_decide(deps: Deps<'_>, case: &str, motivated: bool) -> AxoneGovResult<DecideResponse> {
    let case = Case::new(case)?;

    let constitution = load_constitution(deps.storage)?;
    let program = constitution.source();
    let query = if motivated {
        build_decide_query_with_motivation(&case)
    } else {
        build_decide_query(&case)
    };

    let request = QueryServiceAskRequest::new(program, query, Some(1));
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

    Ok(DecideResponse {
        verdict,
        motivation,
    })
}

fn find_substitution(result: &crate::gateway::logic::Result, variable: &str) -> Option<String> {
    result
        .substitutions
        .iter()
        .find(|sub| sub.variable == variable)
        .map(|sub| sub.expression.clone())
}
