use crate::{
    contract::{AxoneVc, AxoneVcResult},
    msg::AxoneVcInstantiateMsg,
    state::{DEFAULT_FOO_VALUE, FOO},
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo};

#[allow(clippy::unnecessary_wraps)]
pub fn instantiate_handler(
    deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    module: AxoneVc,
    _msg: AxoneVcInstantiateMsg,
) -> AxoneVcResult {
    FOO.save(deps.storage, &DEFAULT_FOO_VALUE.to_string())?;

    Ok(module.custom_response(
        "instantiate",
        vec![("foo".to_string(), DEFAULT_FOO_VALUE.to_string())],
    ))
}
