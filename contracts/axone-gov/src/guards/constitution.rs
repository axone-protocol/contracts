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

#[derive(Clone, Debug, Eq, PartialEq)]
enum ConstitutionValidationError {
    InvalidUtf8(String),
    NoAnswer,
    MissingRequiredPredicates,
    Engine(String),
}

impl ConstitutionValidationError {
    fn to_message(&self) -> String {
        match self {
            Self::InvalidUtf8(err) => {
                format!("constitution must be valid UTF-8: {err}")
            }
            Self::NoAnswer => {
                format!(
                    "constitution validation failed while checking required predicates ({}, {}): no answer returned",
                    REQUIRED_PREDICATES[0],
                    REQUIRED_PREDICATES[1]
                )
            }
            Self::MissingRequiredPredicates => {
                format!(
                    "constitution is missing required predicates ({}, {})",
                    REQUIRED_PREDICATES[0], REQUIRED_PREDICATES[1]
                )
            }
            Self::Engine(err) => {
                format!(
                    "constitution validation failed while checking required predicates ({}, {}): {err}",
                    REQUIRED_PREDICATES[0],
                    REQUIRED_PREDICATES[1]
                )
            }
        }
    }
}

pub fn constitution(querier: &dyn Querier, constitution: &Binary) -> AxoneGovResult<()> {
    let program = std::str::from_utf8(constitution.as_slice())
        .map(|s| s.to_string())
        .map_err(|err| {
            AxoneGovError::InvalidConstitution(
                ConstitutionValidationError::InvalidUtf8(err.to_string()).to_message(),
            )
        })?;
    let query = required_predicates_query();
    let request = QueryServiceAskRequest::new(program, query, Some(1));
    let response = query_service_ask(&QuerierWrapper::<AxoneLogicQuery>::new(querier), request)
        .map_err(|err| {
            AxoneGovError::InvalidConstitution(
                ConstitutionValidationError::Engine(err.to_string()).to_message(),
            )
        })?;

    let answer = response.answer.as_ref().ok_or_else(|| {
        AxoneGovError::InvalidConstitution(ConstitutionValidationError::NoAnswer.to_message())
    })?;

    if answer.results.is_empty() {
        return Err(AxoneGovError::InvalidConstitution(
            ConstitutionValidationError::MissingRequiredPredicates.to_message(),
        ));
    }

    if let Some(error) = answer
        .results
        .iter()
        .filter_map(|result| result.error.as_deref())
        .next()
    {
        return Err(AxoneGovError::InvalidConstitution(
            ConstitutionValidationError::Engine(error.to_string()).to_message(),
        ));
    }

    Ok(())
}
