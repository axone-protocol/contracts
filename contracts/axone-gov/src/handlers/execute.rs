use crate::{
    contract::{AxoneGov, AxoneGovResult},
    msg::AxoneGovExecuteMsg,
    state::CONFIG,
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo};

pub fn execute_handler(
    deps: DepsMut<'_>,
    env: Env,
    info: MessageInfo,
    module: AxoneGov,
    msg: AxoneGovExecuteMsg,
) -> AxoneGovResult {
    match msg {
        AxoneGovExecuteMsg::UpdateConfig {} => update_config(deps, env, info, module),
    }
}

/// Update the configuration of the app
fn update_config(
    deps: DepsMut<'_>,
    env: Env,
    msg_info: MessageInfo,
    module: AxoneGov,
) -> AxoneGovResult {
    // Only the admin should be able to call this
    module
        .admin
        .assert_admin(deps.as_ref(), &env, &msg_info.sender)?;
    let mut _config = CONFIG.load(deps.storage)?;

    Ok(module.response("update_config"))
}
