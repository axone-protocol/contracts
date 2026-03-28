use crate::{
    contract::{AxoneVc, AxoneVcResult},
    msg::AxoneVcExecuteMsg,
    state::FOO,
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo};

pub fn execute_handler(
    deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    module: AxoneVc,
    msg: AxoneVcExecuteMsg,
) -> AxoneVcResult {
    match msg {
        AxoneVcExecuteMsg::Foo { value } => execute_foo(deps, module, value),
    }
}

fn execute_foo(deps: DepsMut<'_>, module: AxoneVc, value: String) -> AxoneVcResult {
    FOO.save(deps.storage, &value)?;

    Ok(module.custom_response("foo", vec![("foo".to_string(), value)]))
}
