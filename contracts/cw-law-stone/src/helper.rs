use crate::error::LogicAskResponseError;
use crate::ContractError;
use cosmwasm_std::{Event, StdError, StdResult};
use itertools::Itertools;
use logic_bindings::error::CosmwasmUriError;
use logic_bindings::uri::CosmwasmUri;
use logic_bindings::{AskResponse, Substitution, TermValue};
use std::any::type_name;
use storage::ObjectRef;

pub fn object_ref_to_uri(object: ObjectRef) -> StdResult<CosmwasmUri> {
    CosmwasmUri::try_from(object).map_err(|e: CosmwasmUriError| {
        StdError::parse_err(type_name::<CosmwasmUri>(), e.to_string())
    })
}

pub fn get_reply_event_attribute(events: Vec<Event>, key: String) -> Option<String> {
    return events
        .iter()
        .flat_map(|e| e.attributes.clone())
        .filter(|a| a.key == key)
        .map(|a| a.value)
        .next();
}

fn term_as_vec(term: TermValue) -> Result<Vec<String>, ContractError> {
    match term {
        TermValue::Array(values) => values
            .iter()
            .map(|v| -> Result<String, ContractError> {
                match v {
                    TermValue::Value(str) => Ok(str.clone()),
                    _ => Err(ContractError::LogicAskResponse(
                        LogicAskResponseError::UnexpectedTerm,
                    )),
                }
            })
            .collect(),
        _ => Err(ContractError::LogicAskResponse(
            LogicAskResponseError::UnexpectedTerm,
        )),
    }
}

pub fn ask_response_to_objects(
    res: AskResponse,
    variable: String,
) -> Result<Vec<ObjectRef>, ContractError> {
    res.answer
        .map(|a| a.results)
        .unwrap_or_default()
        .iter()
        .flat_map(|r: &logic_bindings::Result| r.substitutions.clone())
        .filter(|s| s.variable == variable)
        .map(|s: Substitution| {
            s.term
                .parse()
                .map_err(|e| ContractError::LogicAskResponse(LogicAskResponseError::Parse(e)))
                .and_then(term_as_vec)
        })
        .flatten_ok()
        .map(|res: Result<String, ContractError>| match res {
            Ok(raw) => CosmwasmUri::try_from(raw)
                .and_then(|uri| ObjectRef::try_from(uri))
                .map_err(|e| ContractError::ParseCosmwasmUri(e)),
            Err(e) => Err(e),
        })
        .collect()
}
