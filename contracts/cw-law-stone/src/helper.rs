use std::env::var;
use cosmwasm_std::{Attribute, Event, SubMsg, to_binary, WasmMsg};
use url::Url;
use logic_bindings::{AskResponse, Substitution};
use crate::ContractError;
use crate::state::Object;
use cw_storage::msg::ExecuteMsg as StorageMsg;
use crate::ContractError::NotImplemented;
use crate::uri::uri_to_object;

pub fn get_reply_event_attribute(events: Vec<Event>, key: String) -> Option<String> {
    let r = events.iter()
        .flat_map(|e| e.attributes.clone())
        .filter(|a| a.key == key)
        .map(|a| a.value)
        .collect::<Vec<String>>();

    if r.len() > 0 { Some(r[0].clone())} else { None }
}

/// Files terms is List atom, List is represented as String in prolog, filter to remove
/// all paterm to represent the list and return the result as Vec<String>.
fn filter_source_files(substitution: Substitution) -> Vec<String> {
    substitution.term.name.split(",")
        .into_iter()
        .map(|s| s.replace(&['\'', '[', ']'], ""))
        .collect::<Vec<String>>()
}

pub fn ask_response_to_submsg(res: AskResponse, storage_addr: String, variable: String) -> Result<Vec<SubMsg>, ContractError> {
    let uris = res.answer
        .map(|a| a.results)
        .unwrap_or(vec![])
        .iter()
        .flat_map(|result| result.substitutions.clone())
        .filter(|s| s.variable == variable)
        .flat_map(|s| filter_source_files(s))
        .collect::<Vec<String>>();

    let mut msgs = vec![];
    for uri in uris {
        let object = uri_to_object(uri)?;
        let msg = WasmMsg::Execute {
            contract_addr: storage_addr.to_string(),
            msg: to_binary(&StorageMsg::PinObject {
                id: object.object_id,
            })?,
            funds: vec![],
        };
        msgs.push(SubMsg::new(msg))
    }
    return Ok(msgs)
}
