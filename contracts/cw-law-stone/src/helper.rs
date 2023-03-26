use crate::ContractError;
use cosmwasm_std::{Event, StdError, StdResult};
use logic_bindings::error::CosmwasmUriError;
use logic_bindings::uri::CosmwasmUri;
use logic_bindings::{AskResponse, Substitution};
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

/// Files terms is List atom, List is represented as String in prolog, filter to remove
/// all paterm to represent the list and return the result as Vec<String>.
fn filter_source_files(substitution: Substitution) -> Vec<String> {
    substitution
        .term
        .name
        .split(',')
        .into_iter()
        .map(|s| s.replace(['\'', '[', ']'], ""))
        .collect::<Vec<String>>()
}

pub fn ask_response_to_objects(
    res: AskResponse,
    variable: String,
) -> Result<Vec<ObjectRef>, ContractError> {
    let uris = res
        .answer
        .map(|a| a.results)
        .unwrap_or_default()
        .iter()
        .flat_map(|result| result.substitutions.clone())
        .filter(|s| s.variable == variable)
        .flat_map(filter_source_files)
        .collect::<Vec<String>>();

    let mut objects = vec![];
    for str_uri in uris {
        objects.push(
            CosmwasmUri::try_from(str_uri.clone())
                .and_then(ObjectRef::try_from)
                .map_err(|e| ContractError::ParseCosmwasmUri {
                    error: e,
                    uri: str_uri,
                })?,
        );
    }
    Ok(objects)
}
