use crate::state::Object;
use crate::ContractError;
use cosmwasm_std::Event;
use logic_bindings::{AskResponse, Substitution};
use url::Url;

pub fn get_reply_event_attribute(events: Vec<Event>, key: String) -> Option<String> {
    let r = events
        .iter()
        .flat_map(|e| e.attributes.clone())
        .filter(|a| a.key == key)
        .map(|a| a.value)
        .collect::<Vec<String>>();

    if r.len() > 0 {
        Some(r[0].clone())
    } else {
        None
    }
}

/// Files terms is List atom, List is represented as String in prolog, filter to remove
/// all paterm to represent the list and return the result as Vec<String>.
fn filter_source_files(substitution: Substitution) -> Vec<String> {
    substitution
        .term
        .name
        .split(",")
        .into_iter()
        .map(|s| s.replace(&['\'', '[', ']'], ""))
        .collect::<Vec<String>>()
}

pub fn ask_response_to_objects(
    res: AskResponse,
    variable: String,
) -> Result<Vec<Object>, ContractError> {
    let uris = res
        .answer
        .map(|a| a.results)
        .unwrap_or(vec![])
        .iter()
        .flat_map(|result| result.substitutions.clone())
        .filter(|s| s.variable == variable)
        .flat_map(|s| filter_source_files(s))
        .collect::<Vec<String>>();

    let mut objects = vec![];
    for uri in uris {
        let url = Url::parse(uri.as_str())
            .map_err(|e| ContractError::dependency_uri(e.into(), uri.clone()))?;
        let object = Object::try_from(url).map_err(|e| ContractError::dependency_uri(e, uri))?;

        objects.push(object)
    }
    return Ok(objects);
}
