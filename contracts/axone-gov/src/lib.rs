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
