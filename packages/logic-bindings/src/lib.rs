#![forbid(unsafe_code)]
#![deny(
    warnings,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_lifetimes,
    unused_import_braces
)]

pub mod error;
mod query;
pub mod uri;

pub use query::{Answer, AskResponse, LogicCustomQuery, Result, Substitution, Term};

// Exposed for testing only
// Both unit tests and integration tests are compiled to native code, so everything in here does not need to compile to Wasm.
#[cfg(not(target_arch = "wasm32"))]
pub mod testing;
