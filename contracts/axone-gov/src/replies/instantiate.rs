use crate::contract::{AxoneGov, AxoneGovResult};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, Reply};

#[allow(clippy::unnecessary_wraps)]
pub fn instantiate_reply(
    _deps: DepsMut<'_>,
    _env: Env,
    module: AxoneGov,
    _reply: Reply,
) -> AxoneGovResult {
    Ok(module.response("instantiate_reply"))
}
