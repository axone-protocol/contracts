use crate::contract::AxoneGovResult;
use crate::gateway::logic::{query_service_ask, AxoneLogicQuery, QueryServiceAskRequest};
use crate::queries::validation::build_required_predicates_query;
use crate::AxoneGovError;
use cosmwasm_std::{Binary, Querier, QuerierWrapper};

/// The required predicates that must be present in the constitution.
/// If any are missing, the constitution is considered invalid.
const REQUIRED_PREDICATES: [&str; 2] = ["decide/2", "decide/3"];

pub fn constitution(querier: &dyn Querier, constitution: &Binary) -> AxoneGovResult<()> {
    let program = std::str::from_utf8(constitution.as_slice())
        .map(ToString::to_string)
        .map_err(|err| AxoneGovError::ConstitutionUtf8(err.to_string()))?;
    let query = build_required_predicates_query(&REQUIRED_PREDICATES);
    let request = QueryServiceAskRequest::new(program, query, Some(1));
    let response = query_service_ask(&QuerierWrapper::<AxoneLogicQuery>::new(querier), request)
        .map_err(|err| AxoneGovError::PrologEngineError(err.to_string()))?;

    let answer = response
        .answer
        .as_ref()
        .ok_or(AxoneGovError::PrologEngineNoAnswer)?;

    if answer.results.is_empty() {
        let predicates = REQUIRED_PREDICATES.join(", ");
        return Err(AxoneGovError::ConstitutionInvalid(format!(
            "constitution is missing required predicates ({predicates})"
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
