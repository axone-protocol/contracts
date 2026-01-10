use crate::{
    contract::{AxoneGov, AxoneGovResult},
    msg::AxoneGovExecuteMsg,
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo};

#[allow(clippy::unnecessary_wraps)]
pub fn execute_handler(
    _deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    module: AxoneGov,
    msg: AxoneGovExecuteMsg,
) -> AxoneGovResult {
    match msg {
        AxoneGovExecuteMsg::NoOp {} => Ok(module.response("noop")),
    }
}
