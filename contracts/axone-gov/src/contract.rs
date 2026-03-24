use crate::{
    error::AxoneGovError,
    handlers,
    msg::{AxoneGovExecuteMsg, AxoneGovInstantiateMsg, AxoneGovMigrateMsg, AxoneGovQueryMsg},
    replies::{self, INSTANTIATE_REPLY_ID},
    APP_VERSION, AXONE_GOV_ID, AXONE_GOV_NAME,
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

const APP_METADATA_URL: Option<&str> = Some(const_format::concatcp!(
    "https://raw.githubusercontent.com/axone-protocol/contracts/refs/tags/",
    AXONE_GOV_NAME,
    "-v",
    APP_VERSION,
    "/contracts/",
    AXONE_GOV_NAME,
    "/metadata.json"
));

const APP: AxoneGov = AxoneGov::new(AXONE_GOV_ID, APP_VERSION, APP_METADATA_URL)
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

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: cw_orch::environment::CwEnv> abstract_interface::DependencyCreation
    for crate::AxoneGovInterface<Chain>
{
    type DependenciesConfig = cosmwasm_std::Empty;
}
