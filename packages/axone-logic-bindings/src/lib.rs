pub mod error;
mod query;
mod term_parser;

pub use query::{Answer, AskResponse, LogicCustomQuery, Result, Substitution};
pub use term_parser::TermValue;

// Exposed for testing only
// Both unit tests and integration tests are compiled to native code, so everything in here does not need to compile to Wasm.
#[cfg(not(target_arch = "wasm32"))]
pub mod testing;
