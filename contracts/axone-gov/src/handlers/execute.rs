use crate::{
    contract::{AxoneGov, AxoneGovResult},
    msg::AxoneGovExecuteMsg,
    state::{CONFIG, COUNT},
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
        AxoneGovExecuteMsg::Increment {} => increment(deps, module),
        AxoneGovExecuteMsg::Reset { count } => reset(deps, env, info, count, module),
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

fn increment(deps: DepsMut<'_>, module: AxoneGov) -> AxoneGovResult {
    COUNT.update(deps.storage, |count| AxoneGovResult::Ok(count + 1))?;

    Ok(module.response("increment"))
}

fn reset(
    deps: DepsMut<'_>,
    env: Env,
    info: MessageInfo,
    count: i32,
    module: AxoneGov,
) -> AxoneGovResult {
    module
        .admin
        .assert_admin(deps.as_ref(), &env, &info.sender)?;
    COUNT.save(deps.storage, &count)?;

    Ok(module.response("reset"))
}
