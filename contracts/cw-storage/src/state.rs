use schemars::JsonSchema;
use cosmwasm_std::Uint128;
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};
use cosmwasm_schema::cw_serde;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Bucket {
    /// The name of the bucket.
    pub name: String,
    /// The limits of the bucket.
    pub limits: BucketLimits,
}

/// BucketLimits is the type of the limits of a bucket.
///
/// The limits are optional and if not set, there is no limit.
#[cw_serde]
#[derive(Eq)]
pub struct BucketLimits {
    /// The maximum total size of the objects in the bucket.
    pub max_total_size: Option<Uint128>,
    /// The maximum number of objects in the bucket.
    pub max_objects: Option<Uint128>,
    /// The maximum size of the objects in the bucket.
    pub max_object_size: Option<Uint128>,
    /// The maximum number of pins in the bucket for an object.
    pub max_object_pins: Option<Uint128>,
}

pub const BUCKET: Item<'_, Bucket> = Item::new("bucket");
