#![forbid(unsafe_code)]
#![deny(
    warnings,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_lifetimes,
    unused_import_braces,
    unused_qualifications,
    unused_qualifications
)]

pub mod contract;
mod error;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;
