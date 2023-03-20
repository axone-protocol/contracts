use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

/// Represent a link to an Object stored in the `cw-storage` contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Object {
    /// The object id in the `cw-storage` contract.
    pub object_id: String,

    /// The `cw-storage` contract address on which the object is stored.
    pub storage_address: String,
}

pub const PROGRAM: Item<'_, Object> = Item::new("program");

pub const DEPENDENCIES: Map<'_, &str, Object> = Map::new("dependencies");
