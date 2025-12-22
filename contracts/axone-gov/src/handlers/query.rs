use crate::{
    contract::{AxoneGov, AxoneGovResult},
    msg::{AxoneGovQueryMsg, ConfigResponse, CountResponse},
    state::{CONFIG, COUNT},
};

use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

pub fn query_handler(
    deps: Deps<'_>,
    _env: Env,
    _module: &AxoneGov,
    msg: AxoneGovQueryMsg,
) -> AxoneGovResult<Binary> {
    match msg {
        AxoneGovQueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        AxoneGovQueryMsg::Count {} => to_json_binary(&query_count(deps)?),
    }
    .map_err(Into::into)
}

fn query_config(deps: Deps<'_>) -> StdResult<ConfigResponse> {
    let _config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {})
}

fn query_count(deps: Deps<'_>) -> StdResult<CountResponse> {
    let count = COUNT.load(deps.storage)?;
    Ok(CountResponse { count })
}
