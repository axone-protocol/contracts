use crate::{
    error::AxoneVcError,
    handlers,
    msg::{AxoneVcExecuteMsg, AxoneVcInstantiateMsg, AxoneVcMigrateMsg, AxoneVcQueryMsg},
    APP_VERSION, AXONE_VC_ID,
};

use abstract_app::AppContract;
use cosmwasm_std::Response;

/// The type of the result returned by the app entry points.
pub type AxoneVcResult<T = Response> = Result<T, AxoneVcError>;

/// The type of the app used to build the contract and access Abstract SDK features.
pub type AxoneVc = AppContract<
    AxoneVcError,
    AxoneVcInstantiateMsg,
    AxoneVcExecuteMsg,
    AxoneVcQueryMsg,
    AxoneVcMigrateMsg,
>;

const APP: AxoneVc = AxoneVc::new(AXONE_VC_ID, APP_VERSION, None)
    .with_instantiate(handlers::instantiate_handler)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler)
    .with_migrate(handlers::migrate_handler)
    .with_dependencies(&[]);

#[cfg(feature = "export")]
abstract_app::export_endpoints!(APP, AxoneVc);

abstract_app::cw_orch_interface!(APP, AxoneVc, AxoneVcInterface);

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: cw_orch::environment::CwEnv> abstract_interface::DependencyCreation
    for crate::AxoneVcInterface<Chain>
{
    type DependenciesConfig = cosmwasm_std::Empty;
}
