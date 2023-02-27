use crate::error::BucketError;
use crate::error::BucketError::EmptyName;
use crate::msg::BucketLimits;
use cosmwasm_std::Uint128;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Bucket {
    /// The name of the bucket.
    pub name: String,
    /// The limits of the bucket.
    pub limits: Limits,
}

impl Bucket {
    pub fn new(name: String, limits: Limits) -> Result<Self, BucketError> {
        if name.is_empty() {
            return Err(EmptyName);
        }

        Ok(Self { name, limits })
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
