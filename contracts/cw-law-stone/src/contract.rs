use crate::ContractError::NotImplemented;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
    SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw_storage::msg::ExecuteMsg as StorageMsg;
use logic_bindings::LogicCustomQuery;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::INSTANTIATE_CONTEXT;

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
        data: msg.program.clone(),
        pin: true,
    };

    let store_program_msg = WasmMsg::Execute {
        contract_addr: msg.storage_address.clone(),
        msg: to_binary(&store_msg)?,
        funds: vec![],
    };

    INSTANTIATE_CONTEXT.save(deps.storage, &(msg.storage_address, msg.program))?;

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
    use crate::helper::{ask_response_to_objects, get_reply_event_attribute};
    use crate::state::{Object, DEPENDENCIES, PROGRAM};
    use url::Url;

    pub fn store_program_reply(
        deps: DepsMut<'_, LogicCustomQuery>,
        _env: Env,
        msg: Reply,
    ) -> Result<Response, ContractError> {
        let context = INSTANTIATE_CONTEXT.load(deps.storage)?;

        msg.result
            .into_result()
            .map_err(|_| {
                ContractError::InvalidReplyMsg(StdError::generic_err("no message in reply"))
            })
            .and_then(|e| {
                get_reply_event_attribute(e.events, "id".to_string()).ok_or(
                    ContractError::InvalidReplyMsg(StdError::generic_err(
                        "reply event doesn't contains object id",
                    )),
                )
            })
            .map(|obj_id| Object {
                object_id: obj_id,
                storage_address: context.0.clone(),
            })
            .and_then(|program| -> Result<Vec<SubMsg>, ContractError> {
                PROGRAM
                    .save(deps.storage, &program)
                    .map_err(ContractError::from)?;

                // Clean instantiate context
                INSTANTIATE_CONTEXT.remove(deps.storage);

                let req = build_source_files_query(program.clone())?.into();
                let res = deps.querier.query(&req).map_err(ContractError::from)?;

                let objects = ask_response_to_objects(res, "Files".to_string())?;
                let mut msgs = Vec::with_capacity(objects.len());
                for obj in objects {
                    if obj.object_id == program.object_id {
                        continue;
                    }
                    DEPENDENCIES.save(deps.storage, obj.object_id.as_str(), &obj)?;

                    msgs.push(SubMsg::new(WasmMsg::Execute {
                        msg: to_binary(&StorageMsg::PinObject {
                            id: obj.clone().object_id,
                        })?,
                        contract_addr: obj.clone().storage_address,
                        funds: vec![],
                    }));
                }

                Ok(msgs)
            })
            .map(|msg| Response::new().add_submessages(msg))
    }

    pub fn build_source_files_query(program: Object) -> Result<LogicCustomQuery, ContractError> {
        let program_uri: Url = program.try_into()?;

        Ok(LogicCustomQuery::Ask {
            program: "source_files(Files) :- bagof(File, source_file(File), Files).".to_string(),
            query: [
                "consult('",
                program_uri.as_str(),
                "'), source_files(Files).",
            ]
            .join(""),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Object, DEPENDENCIES, PROGRAM};
    use cosmwasm_std::testing::{mock_env, mock_info, MockQuerierCustomHandlerResult};
    use cosmwasm_std::{
        from_binary, to_binary, CosmosMsg, Event, Order, SubMsgResponse, SubMsgResult, SystemError,
        SystemResult,
    };
    use logic_bindings::testing::mock::mock_dependencies_with_logic_handler;
    use logic_bindings::{
        Answer, AskResponse, LogicCustomQuery, Result as LogicResult, Substitution, Term,
    };
    use url::Url;

    fn custom_logic_handler_with_dependencies(
        dependencies: Vec<String>,
        program: Object,
        request: &LogicCustomQuery,
    ) -> MockQuerierCustomHandlerResult {
        let program_uri: Url = program.clone().try_into().unwrap();
        let mut updated_deps = dependencies;
        updated_deps.push(program_uri.to_string());
        let deps_name = format!("[{}]", &updated_deps.join(","));
        let LogicCustomQuery::Ask {
            program: exp_program,
            query: exp_query,
            ..
        } = reply::build_source_files_query(program).unwrap();
        match request {
            LogicCustomQuery::Ask { program, query }
                if *query == exp_query && *program == exp_program =>
            {
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
        let mut deps =
            mock_dependencies_with_logic_handler(|_| SystemResult::Err(SystemError::Unknown {}));
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
                        _ => panic!("storage message should be a StoreObject message"),
                    }
                }
                _ => panic!("wasm message should be a Storage message"),
            },
            _ => panic!("cosmos sub message should be a Wasm message execute"),
        }
        assert_eq!(
            "okp41ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pqrteqt3".to_string(),
            INSTANTIATE_CONTEXT.load(&deps.storage).unwrap().0
        );
        assert_eq!(program, INSTANTIATE_CONTEXT.load(&deps.storage).unwrap().1)
    }

    #[derive(Clone)]
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
                object_id: "1cc6de7672c97db145a3940df2264140ea893c6688fa5ca55b73cb8b68e0574d"
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
            let program_object_id = case.clone().object_id;
            let mut deps = mock_dependencies_with_logic_handler(move |request| {
                custom_logic_handler_with_dependencies(
                    uris.to_vec(),
                    Object {
                        object_id: program_object_id.clone(),
                        storage_address:
                            "okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s"
                                .to_string(),
                    },
                    request,
                )
            });

            let reply = Reply {
                id: STORE_PROGRAM_REPLY_ID,
                result: SubMsgResult::Ok(SubMsgResponse {
                    events: vec![Event::new("e".to_string())
                        .add_attribute("id".to_string(), case.clone().object_id)],
                    data: None,
                }),
            };

            // Configure the instantiate context
            let program = Binary::from_base64("Zm9vKF8pIDotIHRydWUu").unwrap();
            INSTANTIATE_CONTEXT
                .save(
                    deps.as_mut().storage,
                    &(
                        "okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s"
                            .to_string(),
                        program,
                    ),
                )
                .unwrap();

            let response = reply::store_program_reply(deps.as_mut(), mock_env(), reply);
            let res = response.unwrap();

            let program = PROGRAM.load(&deps.storage).unwrap();
            assert_eq!(case.clone().object_id, program.object_id);

            let deps_len_requirement = case.clone().dependencies.len();

            if deps_len_requirement > 0 {
                assert_eq!(
                    deps_len_requirement,
                    DEPENDENCIES
                        .keys_raw(&deps.storage, None, None, Order::Ascending)
                        .count()
                );
                for (_, contract_addr, object_id) in case.clone().dependencies {
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
                                    _ => panic!("should contains only PinObject message(s)"),
                                }
                            }
                            _ => panic!("wasm message should be a Storage message"),
                        },
                        _ => panic!("cosmos sub message should be a Wasm message execute"),
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

    #[test]
    fn build_source_files_query() {
        let result = reply::build_source_files_query(Object {
            object_id: "1cc6de7672c97db145a3940df2264140ea893c6688fa5ca55b73cb8b68e0574d"
                .to_string(),
            storage_address: "okp41ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pqrteqt3"
                .to_string(),
        });

        match result {
            Ok(LogicCustomQuery::Ask { program, query }) => {
                assert_eq!(
                    program,
                    "source_files(Files) :- bagof(File, source_file(File), Files)."
                );
                assert_eq!(query, "consult('cosmwasm:cw-storage:okp41ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pqrteqt3?query=%7B%22object_data%22%3A%7B%22id%22%3A%221cc6de7672c97db145a3940df2264140ea893c6688fa5ca55b73cb8b68e0574d%22%7D%7D'), source_files(Files).")
            }
            _ => panic!("Expected Ok(LogicCustomQuery)."),
        }
    }
}
