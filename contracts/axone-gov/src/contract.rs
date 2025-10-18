use crate::{
    error::AxoneGovError,
    handlers,
    msg::{AxoneGovExecuteMsg, AxoneGovInstantiateMsg, AxoneGovMigrateMsg, AxoneGovQueryMsg},
    replies::{self, INSTANTIATE_REPLY_ID},
    APP_VERSION, AXONE_GOV_ID,
};

use abstract_app::AppContract;
use cosmwasm_std::Response;

/// The type of the result returned by your app's entry points.
pub type AxoneGovResult<T = Response> = Result<T, AxoneGovError>;

/// The type of the app that is used to build your app and access the Abstract SDK features.
pub type AxoneGov = AppContract<
    AxoneGovError,
    AxoneGovInstantiateMsg,
    AxoneGovExecuteMsg,
    AxoneGovQueryMsg,
    AxoneGovMigrateMsg,
>;

const APP: AxoneGov = AxoneGov::new(AXONE_GOV_ID, APP_VERSION, None)
    .with_instantiate(handlers::instantiate_handler)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler)
    .with_migrate(handlers::migrate_handler)
    .with_replies(&[(INSTANTIATE_REPLY_ID, replies::instantiate_reply)])
    .with_dependencies(&[]);

// Export handlers
#[cfg(feature = "export")]
abstract_app::export_endpoints!(APP, AxoneGov);

abstract_app::cw_orch_interface!(APP, AxoneGov, AxoneGovInterface);

// TODO: add to docmuentation
// https://linear.app/abstract-sdk/issue/ABS-414/add-documentation-on-dependencycreation-trait
#[cfg(not(target_arch = "wasm32"))]
impl<Chain: cw_orch::environment::CwEnv> abstract_interface::DependencyCreation
    for crate::AxoneGovInterface<Chain>
{
    type DependenciesConfig = cosmwasm_std::Empty;
}
