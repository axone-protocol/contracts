#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::msg::{BarResponse, ExecuteMsg, InstantiateMsg, QueryMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new()
        .add_attribute("contract", "axone-dummy")
        .add_attribute("greeting", axone_dummy_lib::greeting()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Foo {} => Ok(Response::new()
            .add_attribute("action", "foo")
            .add_attribute("greeting", axone_dummy_lib::greeting())),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps<'_>, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    to_json_binary(&BarResponse {
        msg: axone_dummy_lib::greeting().to_string(),
    })
}
