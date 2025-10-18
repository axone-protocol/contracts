use crate::{
    contract::{AxoneGov, AxoneGovResult},
    msg::AxoneGovInstantiateMsg,
    state::{Config, CONFIG, COUNT},
};

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn instantiate_handler(
    deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    _module: AxoneGov,
    msg: AxoneGovInstantiateMsg,
) -> AxoneGovResult {
    let config: Config = Config {};

    CONFIG.save(deps.storage, &config)?;
    COUNT.save(deps.storage, &msg.count)?;
    Ok(Response::new())
}
