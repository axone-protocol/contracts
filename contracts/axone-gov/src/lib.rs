pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
mod replies;
pub mod state;

pub use error::AxoneGovError;

/// The version of your app
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use contract::interface::AxoneGovInterface;

pub const ABSTRACT_SCAFFOLD_NAMESPACE: &str = "abstract-scaffold";
pub const AXONE_GOV_NAME: &str = "axone-gov";
pub const AXONE_GOV_ID: &str =
    const_format::concatcp!(ABSTRACT_SCAFFOLD_NAMESPACE, ":", AXONE_GOV_NAME);
