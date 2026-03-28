use crate::{
    contract::{AxoneVc, AxoneVcResult},
    msg::{AxoneVcQueryMsg, FooResponse},
    state::FOO,
};

use cosmwasm_std::{to_json_binary, Binary, Deps, Env};

pub fn query_handler(
    deps: Deps<'_>,
    _env: Env,
    _module: &AxoneVc,
    msg: AxoneVcQueryMsg,
) -> AxoneVcResult<Binary> {
    match msg {
        AxoneVcQueryMsg::Foo {} => to_json_binary(&query_foo(deps)?),
    }
    .map_err(Into::into)
}

fn query_foo(deps: Deps<'_>) -> AxoneVcResult<FooResponse> {
    Ok(FooResponse {
        value: FOO.load(deps.storage)?,
    })
}
