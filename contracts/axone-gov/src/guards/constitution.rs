use crate::contract::AxoneGovResult;
use crate::gateway::logic::{query_service_ask, AxoneLogicQuery, QueryServiceAskRequest};
use crate::AxoneGovError;
use cosmwasm_std::{Binary, Querier, QuerierWrapper};

/// The required predicates that must be present in the constitution.
/// If any are missing, the constitution is considered invalid.
const REQUIRED_PREDICATES: [&str; 2] = ["decide/2", "decide/3"];

fn required_predicates_query() -> String {
    format!(
        "current_predicate({}), current_predicate({}).",
        REQUIRED_PREDICATES[0], REQUIRED_PREDICATES[1],
    )
}

pub fn constitution(querier: &dyn Querier, constitution: &Binary) -> AxoneGovResult<()> {
    let program = std::str::from_utf8(constitution.as_slice())
        .map(ToString::to_string)
        .map_err(|err| AxoneGovError::ConstitutionUtf8(err.to_string()))?;
    let query = required_predicates_query();
    let request = QueryServiceAskRequest::new(program, query, Some(1));
    let response = query_service_ask(&QuerierWrapper::<AxoneLogicQuery>::new(querier), request)
        .map_err(|err| AxoneGovError::PrologEngineError(err.to_string()))?;

    let answer = response
        .answer
        .as_ref()
        .ok_or(AxoneGovError::PrologEngineNoAnswer)?;

    if answer.results.is_empty() {
        return Err(AxoneGovError::ConstitutionInvalid(format!(
            "constitution is missing required predicates ({}, {})",
            REQUIRED_PREDICATES[0], REQUIRED_PREDICATES[1]
        )));
    }

    if let Some(error) = answer
        .results
        .iter()
        .filter_map(|result| result.error.as_deref())
        .next()
    {
        return Err(AxoneGovError::ConstitutionInvalid(format!(
            "predicate validation failed: {error}"
        )));
    }

    Ok(())
}
