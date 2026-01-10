use crate::{contract::AxoneGovResult, msg::AxoneGovExecuteMsg};

use cosmwasm_std::{DepsMut, Env, MessageInfo};

pub fn execute_handler(
    _deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    _module: crate::contract::AxoneGov,
    msg: AxoneGovExecuteMsg,
) -> AxoneGovResult {
    match msg {}
}
