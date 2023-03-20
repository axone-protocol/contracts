use crate::ContractError::NotImplemented;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw2::set_contract_version;
use logic_bindings::LogicCustomQuery;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:law-stone";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<'_, LogicCustomQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Err(NotImplemented {})
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
    use cosmwasm_std::{ to_binary, SystemResult, SystemError};
    use logic_bindings::{Answer, AskResponse, LogicCustomQuery, Substitution, Term, Result as LogicResult};
    use logic_bindings::testing::mock::mock_dependencies_with_logic_handler;

    fn custom_logic_handler_with_dependencies(dependencies: Vec<String>, request: &LogicCustomQuery) -> MockQuerierCustomHandlerResult {
        let deps_name = format!("[{}]", &dependencies.join(","));
        match request {
            LogicCustomQuery::Ask {
                query,
                ..
            } if query == "source_files(Files)." => SystemResult::Ok(to_binary(&AskResponse {
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
            }).into()),
            _ => SystemResult::Err(SystemError::InvalidRequest { error: "Ask `souces_files(Files).` predicate not called".to_string(), request: Default::default() })
        }
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_logic_handler(|request| {
            custom_logic_handler_with_dependencies(vec!["file1".to_string()], request)
        });

        let msg = InstantiateMsg {
            program: Default::default(),
            storage_address: "".to_string(),
        };
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}
