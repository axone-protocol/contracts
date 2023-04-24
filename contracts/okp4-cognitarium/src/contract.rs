#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Store, STORE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:storage";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    STORE.save(
        deps.storage,
        &Store {
            owner: info.sender,
            limits: msg.limits.into(),
        },
    )?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Err(ContractError::NotImplemented)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Err(StdError::generic_err("Not implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::StoreLimitsInput;
    use crate::state;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::Uint128;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            limits: StoreLimitsInput {
                max_triple_count: Some(Uint128::from(1u128)),
                max_byte_size: Some(Uint128::from(2u128)),
                max_triple_byte_size: Some(Uint128::from(3u128)),
                max_query_limit: Some(4),
                max_query_variable_count: Some(5),
                max_insert_data_byte_size: Some(Uint128::from(6u128)),
                max_insert_data_triple_count: Some(Uint128::from(7u128)),
            },
        };

        let info = mock_info("owner", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        let store = STORE.load(&deps.storage).unwrap();
        assert_eq!(store.owner, info.sender);
        assert_eq!(
            store.limits,
            state::StoreLimits {
                max_triple_count: Uint128::from(1u128),
                max_byte_size: Uint128::from(2u128),
                max_triple_byte_size: Uint128::from(3u128),
                max_query_limit: 4,
                max_query_variable_count: 5,
                max_insert_data_byte_size: Uint128::from(6u128),
                max_insert_data_triple_count: Uint128::from(7u128),
            }
        );
    }
}
