pub mod contract;
mod domain;
pub mod error;
pub mod gateway;
mod handlers;
pub mod msg;
mod prolog;
mod queries;
mod replies;
pub mod state;

pub use error::AxoneGovError;

/// The version of your app
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use contract::interface::AxoneGovInterface;

pub const AXONE_NAMESPACE: &str = "axone";
pub const AXONE_GOV_NAME: &str = "axone-gov";
pub const AXONE_GOV_ID: &str = const_format::concatcp!(AXONE_NAMESPACE, ":", AXONE_GOV_NAME);

// Governance domain constants
pub const GOV_INTENT_REVISE_CONSTITUTION: &str = "gov:revise_constitution";
pub const GOV_VERDICT_PERMITTED: &str = "gov:permitted";
pub const GOV_VERDICT_FORBIDDEN: &str = "gov:forbidden";
pub const GOV_CTX_MODULE: &str = "gov:module";
pub const GOV_CTX_COSMWASM: &str = "gov:cosmwasm";

// Response constants
pub const RESPONSE_KEY_CONSTITUTION_REVISION: &str = "constitution_revision";
pub const RESPONSE_KEY_CONSTITUTION_HASH: &str = "constitution_hash";
pub const RESPONSE_KEY_CONSTITUTION_REVISER: &str = "constitution_reviser";
