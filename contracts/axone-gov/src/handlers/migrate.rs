use crate::{
    contract::{AxoneGov, AxoneGovResult},
    msg::AxoneGovMigrateMsg,
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env};

/// Handle the app migrate msg
/// The top-level Abstract app does version checking and dispatches to this handler
#[allow(clippy::unnecessary_wraps)]
pub fn migrate_handler(
    _deps: DepsMut<'_>,
    _env: Env,
    module: AxoneGov,
    _msg: AxoneGovMigrateMsg,
) -> AxoneGovResult {
    Ok(module.response("migrate"))
}
