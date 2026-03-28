use crate::{
    contract::{AxoneVc, AxoneVcResult},
    msg::{AuthorityResponse, AxoneVcQueryMsg},
    state::AUTHORITY,
};

use cosmwasm_std::{to_json_binary, Binary, Deps, Env};

pub fn query_handler(
    deps: Deps<'_>,
    _env: Env,
    _module: &AxoneVc,
    msg: AxoneVcQueryMsg,
) -> AxoneVcResult<Binary> {
    match msg {
        AxoneVcQueryMsg::Authority {} => to_json_binary(&query_authority(deps)?),
    }
    .map_err(Into::into)
}

fn query_authority(deps: Deps<'_>) -> AxoneVcResult<AuthorityResponse> {
    let authority = AUTHORITY.load(deps.storage)?;
    Ok(AuthorityResponse {
        did: authority.did().to_string(),
    })
}
