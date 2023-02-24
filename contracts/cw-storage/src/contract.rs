#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, to_binary};
use cw2::set_contract_version;
use ContractError::NotImplemented;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Bucket, BUCKET};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:storage";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let bucket = Bucket { name: msg.bucket, limits: msg.limits };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    BUCKET.save(deps.storage, &bucket)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Err(NotImplemented {})
}

pub mod execute {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Bucket {} => to_binary(&query::bucket(deps)?),
        _ => Err(StdError::generic_err("Not implemented"))
    }

}

pub mod query {
    use crate::msg::BucketResponse;
    use super::*;

    pub fn bucket(deps: Deps) -> StdResult<BucketResponse> {
        let bucket = BUCKET.load(deps.storage)?;

        Ok(BucketResponse { name: bucket.name, limits: bucket.limits })
    }
}
