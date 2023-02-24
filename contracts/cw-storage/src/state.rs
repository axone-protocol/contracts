use crate::error::BucketError;
use crate::error::BucketError::EmptyName;
use crate::msg::BucketLimits;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const DATA: Map<String, Vec<u8>> = Map::new("DATA");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Bucket {
    /// The name of the bucket.
    pub name: String,
    /// The limits of the bucket.
    pub limits: Limits,
}

impl Bucket {
    pub fn new(name: String, limits: Limits) -> Result<Self, BucketError> {
        let n: String = name.split_whitespace().collect();
        if n.is_empty() {
            return Err(EmptyName);
        }

        Ok(Self { name: n, limits })
    }
}

/// Limits is the type of the limits of a bucket.
///
/// The limits are optional and if not set, there is no limit.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Limits {
    /// The maximum total size of the objects in the bucket.
    pub max_total_size: Option<Uint128>,
    /// The maximum number of objects in the bucket.
    pub max_objects: Option<Uint128>,
    /// The maximum size of the objects in the bucket.
    pub max_object_size: Option<Uint128>,
    /// The maximum number of pins in the bucket for an object.
    pub max_object_pins: Option<Uint128>,
}

impl From<BucketLimits> for Limits {
    fn from(limits: BucketLimits) -> Self {
        Limits {
            max_total_size: limits.max_total_size,
            max_objects: limits.max_objects,
            max_object_size: limits.max_object_size,
            max_object_pins: limits.max_object_pins,
        }
    }
}
pub const BUCKET: Item<Bucket> = Item::new("bucket");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Object {
    /// The id of the object.
    pub id: String,
    /// The owner of the object.
    pub owner: Addr,
    /// The size of the object.
    pub size: Uint128,
}

pub struct ObjectIndexes<'a> {
    pub owner: MultiIndex<'a, Addr, Object, String>,
}

impl IndexList<Object> for ObjectIndexes<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Object>> + '_> {
        Box::new(vec![&self.owner as &dyn Index<Object>].into_iter())
    }
}

pub fn objects<'a>() -> IndexedMap<'a, String, Object, ObjectIndexes<'a>> {
    IndexedMap::new(
        "OBJECT",
        ObjectIndexes {
            owner: MultiIndex::new(|_, object| object.owner.clone(), "OBJECT", "OBJECT__OWNER"),
        },
    )
}
