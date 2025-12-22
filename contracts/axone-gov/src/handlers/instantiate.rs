use crate::{
    contract::{AxoneGov, AxoneGovResult},
    msg::AxoneGovInstantiateMsg,
    state::{Config, CONFIG, COUNT},
};
use abstract_app::sdk::AbstractResponse;

use cosmwasm_std::{DepsMut, Env, MessageInfo};

pub fn instantiate_handler(
    deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    module: AxoneGov,
    msg: AxoneGovInstantiateMsg,
) -> AxoneGovResult {
    let config: Config = Config {};

    CONFIG.save(deps.storage, &config)?;
    COUNT.save(deps.storage, &msg.count)?;
    Ok(module.response("instantiate"))
}
