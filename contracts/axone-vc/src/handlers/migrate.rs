use crate::{
    contract::{AxoneVc, AxoneVcResult},
    msg::AxoneVcMigrateMsg,
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env};

#[allow(clippy::unnecessary_wraps)]
pub fn migrate_handler(
    _deps: DepsMut<'_>,
    _env: Env,
    module: AxoneVc,
    _msg: AxoneVcMigrateMsg,
) -> AxoneVcResult {
    Ok(module.response("migrate"))
}
