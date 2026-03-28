pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
pub mod state;

pub use contract::interface::AxoneVcInterface;
pub use error::AxoneVcError;

/// The version of your app.
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const AXONE_NAMESPACE: &str = "axone";
pub const AXONE_VC_NAME: &str = "axone-vc";
pub const AXONE_VC_ID: &str = const_format::concatcp!(AXONE_NAMESPACE, ":", AXONE_VC_NAME);
