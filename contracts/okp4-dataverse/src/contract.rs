#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    instantiate2_address, to_binary, Binary, CodeInfoResponse, Deps, DepsMut, Env, MessageInfo,
    Response, StdError, StdResult, WasmMsg,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Dataverse, DATAVERSE};

// version info for migration info
const CONTRACT_NAME: &str = concat!("crates.io:", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<'_>,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let creator = deps.api.addr_canonicalize(env.contract.address.as_str())?;
    let CodeInfoResponse { checksum, .. } = deps
        .querier
        .query_wasm_code_info(msg.triplestore_config.code_id.u64())?;
    let salt = Binary::from(msg.name.as_bytes());

    let triplestore_address = deps
        .api
        .addr_humanize(&instantiate2_address(&checksum, &creator, &salt)?)?;

    DATAVERSE.save(
        deps.storage,
        &Dataverse {
            name: msg.name.clone(),
            triplestore_address: triplestore_address.to_string(),
        },
    )?;

    Ok(Response::new()
        .add_attribute("triplestore_address", triplestore_address)
        .add_message(WasmMsg::Instantiate2 {
            admin: Some(env.contract.address.to_string()),
            code_id: msg.triplestore_config.code_id.u64(),
            label: format!("{}_triplestore", msg.name),
            msg: to_binary(&okp4_cognitarium::msg::InstantiateMsg {
                limits: msg.triplestore_config.limits,
            })?,
            funds: vec![],
            salt,
        }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Err(StdError::generic_err("Not implemented").into())
}

pub mod execute {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps<'_>, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Err(StdError::generic_err("Not implemented"))
}

pub mod query {}

#[cfg(test)]
mod tests {}
