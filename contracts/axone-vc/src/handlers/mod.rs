pub mod execute;
pub mod instantiate;
pub mod migrate;
pub mod query;

pub use self::{
    execute::execute_handler, instantiate::instantiate_handler, migrate::migrate_handler,
    query::query_handler,
};
