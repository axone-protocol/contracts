use crate::{
    contract::{AxoneGov, AxoneGovResult},
    msg::{AxoneGovQueryMsg, ConstitutionResponse},
    state::CONSTITUTION,
};

use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

pub fn query_handler(
    deps: Deps<'_>,
    _env: Env,
    _module: &AxoneGov,
    msg: AxoneGovQueryMsg,
) -> AxoneGovResult<Binary> {
    match msg {
        AxoneGovQueryMsg::Constitution {} => to_json_binary(&query_constitution(deps)?),
    }
    .map_err(Into::into)
}

fn query_constitution(deps: Deps<'_>) -> StdResult<ConstitutionResponse> {
    let constitution = CONSTITUTION.load(deps.storage)?;
    Ok(ConstitutionResponse {
        governance: constitution,
    })
}
