use crate::error::LogicAskResponseError;
use crate::ContractError;
use cosmwasm_std::{Event, StdError, StdResult};
use itertools::Itertools;
use okp4_logic_bindings::error::CosmwasmUriError;
use okp4_logic_bindings::uri::CosmwasmUri;
use okp4_logic_bindings::{AskResponse, Substitution, TermValue};
use okp4_objectarium_client::ObjectRef;
use std::any::type_name;

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
        .flat_map(|r: &okp4_logic_bindings::Result| r.substitutions.clone())
        .filter(|s| s.variable == variable)
        .map(|s: Substitution| {
            s.parse_expression()
                .map_err(|e| ContractError::LogicAskResponse(LogicAskResponseError::Parse(e)))
                .and_then(term_as_vec)
        })
        .flatten_ok()
        .map(|res: Result<String, ContractError>| match res {
            Ok(raw) => CosmwasmUri::try_from(raw)
                .and_then(ObjectRef::try_from)
                .map_err(ContractError::ParseCosmwasmUri),
            Err(e) => Err(e),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use okp4_logic_bindings::error::TermParseError;
    use okp4_logic_bindings::Answer;

    #[test]
    fn logic_to_objects() {
        let cases = vec![
            ("[]".to_string(), Ok(vec![])),
            ("['cosmwasm:okp4-objectarium:okp41ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pqrteqt3?query=%7B%22object_data%22%3A%7B%22id%22%3A%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D']".to_string(), Ok(vec![ObjectRef{
                object_id: "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05".to_string(),
                storage_address: "okp41ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pqrteqt3".to_string(),
            }])),
            ("['cosmwasm:okp4-objectarium:okp41ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pqrteqt3?query=%7B%22object_data%22%3A%7B%22id%22%3A%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D','cosmwasm:okp4-objectarium:okp41cxmx7su8h5pvqca85cxdylz86uj9x9gu5xuqv34kw87q5x0hexds8w44jg?query=%7B%22object_data%22%3A%7B%22id%22%3A%221485133dd3ab4b1c4b8085e7265585f91ae3cca0996a39e0377a1059296f6aa7%22%7D%7D']".to_string(), Ok(vec![ObjectRef{
                object_id: "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05".to_string(),
                storage_address: "okp41ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pqrteqt3".to_string(),
            },ObjectRef{
                object_id: "1485133dd3ab4b1c4b8085e7265585f91ae3cca0996a39e0377a1059296f6aa7".to_string(),
                storage_address: "okp41cxmx7su8h5pvqca85cxdylz86uj9x9gu5xuqv34kw87q5x0hexds8w44jg".to_string(),
            }])),
            ("[,]".to_string(), Err(ContractError::LogicAskResponse(LogicAskResponseError::Parse(TermParseError::EmptyValue)))),
            ("(1,2)".to_string(), Err(ContractError::LogicAskResponse(LogicAskResponseError::UnexpectedTerm))),
            ("[[]]".to_string(), Err(ContractError::LogicAskResponse(LogicAskResponseError::UnexpectedTerm))),
            ("[[]]".to_string(), Err(ContractError::LogicAskResponse(LogicAskResponseError::UnexpectedTerm))),
            ("['nawak']".to_string(), Err(ContractError::ParseCosmwasmUri(CosmwasmUriError::ParseURI(url::ParseError::RelativeUrlWithoutBase)))),
            ("['cosmwasm:addr?query=%7B%22object%22%3A%7B%22id%22%3A%221485133dd3ab4b1c4b8085e7265585f91ae3cca0996a39e0377a1059296f6aa7%22%7D%7D']".to_string(), Err(ContractError::ParseCosmwasmUri(CosmwasmUriError::Malformed("wrong query content".to_string())))),
        ];

        for case in cases {
            assert_eq!(
                ask_response_to_objects(
                    AskResponse {
                        answer: Some(Answer {
                            results: vec![okp4_logic_bindings::Result {
                                substitutions: vec![Substitution {
                                    variable: "X".to_string(),
                                    expression: case.0,
                                }]
                            }],
                            has_more: false,
                            success: true,
                            variables: vec![],
                            error: None,
                        }),
                        height: 1,
                        gas_used: 1,
                    },
                    "X".to_string()
                ),
                case.1
            );
        }
    }
}
