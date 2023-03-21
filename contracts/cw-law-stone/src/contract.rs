use crate::ContractError::NotImplemented;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
    SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw_storage::msg::{ExecuteMsg as StorageMsg, ObjectResponse};
use cw_utils::parse_reply_execute_data;
use logic_bindings::LogicCustomQuery;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:law-stone";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const STORE_PROGRAM_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<'_, LogicCustomQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let store_msg = StorageMsg::StoreObject {
        data: msg.program,
        pin: true,
    };

    let store_program_msg = WasmMsg::Execute {
        contract_addr: msg.storage_address,
        msg: to_binary(&store_msg)?,
        funds: vec![],
    };

    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        store_program_msg,
        STORE_PROGRAM_REPLY_ID,
    )))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Err(NotImplemented {})
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps<'_>, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Err(StdError::generic_err("Not implemented"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut<'_, LogicCustomQuery>,
    env: Env,
    msg: Reply,
) -> Result<Response, ContractError> {
    match msg.id {
        STORE_PROGRAM_REPLY_ID => reply::store_program_reply(deps, env, msg),
        _ => Err(StdError::generic_err("Not implemented").into()),
    }
}

pub mod reply {
    use super::*;

    pub fn store_program_reply(
        _deps: DepsMut<'_, LogicCustomQuery>,
        _env: Env,
        msg: Reply,
    ) -> Result<Response, ContractError> {
        let _ = parse_reply_execute_data(msg)?;
        Err(StdError::generic_err("Not implemented").into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{DEPENDENCIES, PROGRAM};
    use cosmwasm_std::testing::{mock_env, mock_info, MockQuerierCustomHandlerResult};
    use cosmwasm_std::{
        from_binary, to_binary, CosmosMsg, Order, SubMsgResponse, SubMsgResult, SystemError,
        SystemResult,
    };
    use logic_bindings::testing::mock::mock_dependencies_with_logic_handler;
    use logic_bindings::{
        Answer, AskResponse, LogicCustomQuery, Result as LogicResult, Substitution, Term,
    };

    fn custom_logic_handler_with_dependencies(
        dependencies: Vec<String>,
        request: &LogicCustomQuery,
    ) -> MockQuerierCustomHandlerResult {
        let deps_name = format!("[{}]", &dependencies.join(","));
        match request {
            LogicCustomQuery::Ask { query, .. } if query == "source_files(Files)." => {
                SystemResult::Ok(
                    to_binary(&AskResponse {
                        height: 1,
                        gas_used: 1000,
                        answer: Some(Answer {
                            success: true,
                            has_more: false,
                            variables: vec!["Files".to_string()],
                            results: vec![LogicResult {
                                substitutions: vec![Substitution {
                                    variable: "Files".to_string(),
                                    term: Term {
                                        name: deps_name,
                                        arguments: vec![],
                                    },
                                }],
                            }],
                        }),
                    })
                    .into(),
                )
            }
            _ => SystemResult::Err(SystemError::InvalidRequest {
                error: "Ask `souces_files(Files).` predicate not called".to_string(),
                request: Default::default(),
            }),
        }
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_logic_handler(|request| {
            custom_logic_handler_with_dependencies(vec![], request)
        });
        let program = to_binary("foo(_) :- true.").unwrap();

        let msg = InstantiateMsg {
            program: program.clone(),
            storage_address: "okp41ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pqrteqt3"
                .to_string(),
        };
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Check if a message is send to the cw-storage to store the logic program.
        assert_eq!(1, res.messages.len());
        let sub_msg = res.messages.first().unwrap();
        assert_eq!(STORE_PROGRAM_REPLY_ID, sub_msg.id);
        match &sub_msg.msg {
            CosmosMsg::Wasm(wasm_msg) => match wasm_msg {
                WasmMsg::Execute { msg, .. } => {
                    let result: StorageMsg = from_binary(msg).unwrap();
                    match result {
                        StorageMsg::StoreObject { data, pin } => {
                            assert_eq!(data, program);
                            assert!(pin, "the main program should be pinned");
                        }
                        _ => assert!(false, "storage message should be a StoreObject message"),
                    }
                }
                _ => assert!(false, "wasm message should be a Storage message"),
            },
            _ => assert!(false, "cosmos sub message should be a Wasm message execute"),
        }
    }

    struct StoreTestCase {
        dependencies: Vec<(String, String, String)>, // URI, contract address, object id
        object_id: String,
    }

    #[test]
    fn store_program_reply() {
        let cases = vec![
            StoreTestCase {
                dependencies: vec![
                    (
                        "cosmwasm:okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s?query=%7B%22object_data%22%3A%7B%22id%22%3A%20%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D".to_string(),
                        "okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s".to_string(),
                        "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05".to_string()
                    ),
                ],
                object_id: "0689c526187c6785dfcce28f8df19138da292598dc19548a852de1792062f271"
                    .to_string(),
            },
            StoreTestCase {
                dependencies: vec![],
                object_id: "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05"
                    .to_string(),
            },
            StoreTestCase {
                dependencies: vec![
                    (
                        "cosmwasm:okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s?query=%7B%22object_data%22%3A%7B%22id%22%3A%20%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D".to_string(),
                        "okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s".to_string(), // contract addr
                        "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05".to_string() // object id
                    ),
                    (
                        "cosmwasm:okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s?query=%7B%22object_data%22%3A%7B%22id%22%3A%20%220689c526187c6785dfcce28f8df19138da292598dc19548a852de1792062f271%22%7D%7D".to_string(),
                        "okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s".to_string(), // contract addr
                        "0689c526187c6785dfcce28f8df19138da292598dc19548a852de1792062f271".to_string() // object id
                    ),
                ],
                object_id: "0689c526187c6785dfcce28f8df19138da292598dc19548a852de1792062f271"
                    .to_string(),
            },
        ];

        for case in cases {
            let uris = Box::new(
                case.dependencies
                    .clone()
                    .into_iter()
                    .map(|(uri, _, _)| uri)
                    .collect::<Vec<String>>(),
            );
            let mut deps = mock_dependencies_with_logic_handler(move |request| {
                custom_logic_handler_with_dependencies(uris.to_vec(), request)
            });

            let object = ObjectResponse {
                id: case.object_id.clone(),
                owner: "creator".to_string(),
                is_pinned: true,
                size: Default::default(),
            };
            let reply = Reply {
                id: STORE_PROGRAM_REPLY_ID,
                result: SubMsgResult::Ok(SubMsgResponse {
                    events: vec![],
                    data: Some(to_binary(&object).unwrap()),
                }),
            };
            let res = reply::store_program_reply(deps.as_mut(), mock_env(), reply).unwrap();

            let program = PROGRAM.load(&deps.storage).unwrap();
            assert_eq!(case.object_id.clone(), program.object_id);

            let deps_len_requirement = case.dependencies.len();

            if deps_len_requirement > 0 {
                assert_eq!(
                    deps_len_requirement,
                    DEPENDENCIES
                        .keys_raw(&deps.storage, None, None, Order::Ascending)
                        .count()
                );
                for (_, contract_addr, object_id) in case.dependencies {
                    let o = DEPENDENCIES.load(&deps.storage, object_id.as_str());
                    assert!(
                        o.is_ok(),
                        "dependencies should contains each object id dependencies as key"
                    );
                    let o = o.unwrap();
                    assert_eq!(
                        o.object_id, object_id,
                        "dependencies should contains each object id dependencies as key"
                    );
                    assert_eq!(
                        o.storage_address, contract_addr,
                        "dependencies should contains each object id dependencies as key"
                    );
                }
            }

            assert_eq!(
                deps_len_requirement,
                res.messages.len(),
                "response should contains any sub message as dependencies"
            );

            let objects_pinned: Vec<String> = res
                .messages
                .into_iter()
                .flat_map(|sub_msg| -> Option<String> {
                    match &sub_msg.msg {
                        CosmosMsg::Wasm(wasm_msg) => match wasm_msg {
                            WasmMsg::Execute { msg, .. } => {
                                let result: StorageMsg = from_binary(msg).unwrap();
                                match result {
                                    StorageMsg::PinObject { id } => Some(id),
                                    _ => {
                                        assert!(false, "should contains only PinObject message(s)");
                                        None
                                    }
                                }
                            }
                            _ => {
                                assert!(false, "wasm message should be a Storage message");
                                None
                            }
                        },
                        _ => {
                            assert!(false, "cosmos sub message should be a Wasm message execute");
                            None
                        }
                    }
                })
                .collect();

            for object in objects_pinned {
                assert!(
                    DEPENDENCIES.has(&deps.storage, object.as_str()),
                    "each dependencies should be pinned by a PinObject message"
                )
            }
        }
    }
}
