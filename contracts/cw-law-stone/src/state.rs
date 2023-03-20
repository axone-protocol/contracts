use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Law {
    /// The `cw-storage` object link to the Prolog program carrying law rules and facts.
    pub program: Object,

    /// The list of all `cw-storage` dependencies of the law program.
    pub dependencies: Vec<String>,
}

/// Represent a link to an Object stored in the `cw-storage` contract.
pub struct Object {
    /// The object id in the `cw-storage` contract.
    pub object_id: String,

    /// The `cw-storage` contract address on which the object is stored.
    pub storage_address: String,
}

pub const LAW: Item<'_, Law> = Item::new("law");
