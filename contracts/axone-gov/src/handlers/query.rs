use crate::{
    contract::{AxoneGov, AxoneGovResult},
    error::AxoneGovError,
    gateway::logic::{
        build_decide_query, build_decide_query_with_motivation, query_service_ask, AxoneLogicQuery,
        QueryServiceAskRequest,
    },
    guards,
    msg::{AxoneGovQueryMsg, ConstitutionResponse, ConstitutionStatusResponse, DecideResponse},
    state::{load_constitution_as_string, CONSTITUTION, CONSTITUTION_STATUS},
};

use cosmwasm_std::{to_json_binary, Binary, Deps, Env, QuerierWrapper, StdResult};

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

fn query_constitution(deps: Deps<'_>) -> StdResult<ConstitutionResponse> {
    let constitution = CONSTITUTION.load(deps.storage)?;
    Ok(ConstitutionResponse {
        governance: constitution,
    })
}

fn query_constitution_status(deps: Deps<'_>) -> StdResult<ConstitutionStatusResponse> {
    let status = CONSTITUTION_STATUS.load(deps.storage)?;
    Ok(ConstitutionStatusResponse::from(&status))
}

fn query_decide(deps: Deps<'_>, case: &str, motivated: bool) -> AxoneGovResult<DecideResponse> {
    guards::case(case)?;

    let program = load_constitution_as_string(deps.storage)?;
    let query = if motivated {
        build_decide_query_with_motivation(case)
    } else {
        build_decide_query(case)
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
        .filter_map(|result| result.error.as_deref())
        .next()
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
