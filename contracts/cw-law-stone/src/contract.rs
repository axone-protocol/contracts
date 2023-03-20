use crate::ContractError::NotImplemented;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, SubMsg,
    WasmMsg,
};
use cw2::set_contract_version;
use cw_storage::msg::ExecuteMsg as StorageMsg;
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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_env, mock_info, MockQuerierCustomHandlerResult};
    use cosmwasm_std::{from_binary, to_binary, CosmosMsg, SystemError, SystemResult};
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
            custom_logic_handler_with_dependencies(vec!["file1".to_string()], request)
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
                            assert!(pin);
                        }
                        _ => assert!(false, "storage message should be a StoreObject message"),
                    }
                }
                _ => assert!(false, "wasm message should be a Storage message"),
            },
            _ => assert!(false, "cosmos sub message should be a Wasm message execute"),
        }
    }
}
