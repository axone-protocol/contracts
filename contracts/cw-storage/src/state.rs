use crate::error::BucketError;
use crate::error::BucketError::EmptyName;
use crate::msg::{BucketLimits, ObjectResponse, PaginationConfig};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const DATA: Map<String, Vec<u8>> = Map::new("DATA");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Bucket {
    /// The owner of the bucket.
    pub owner: Addr,
    /// The name of the bucket.
    pub name: String,
    /// The limits of the bucket.
    pub limits: Limits,
    /// The configuration for paginated query.
    pub pagination: Pagination,
    /// Some information on the current bucket usage.
    pub stat: BucketStat,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct BucketStat {
    /// The total size of the objects contained in the bucket.
    pub size: Uint128,
    /// The number of objects in the bucket.
    pub object_count: Uint128,
}

impl Bucket {
    pub fn new(
        owner: Addr,
        name: String,
        limits: Limits,
        pagination: Pagination,
    ) -> Result<Self, BucketError> {
        let n: String = name.split_whitespace().collect();
        if n.is_empty() {
            return Err(EmptyName);
        }

        Ok(Self {
            owner,
            name: n,
            limits,
            pagination,
            stat: BucketStat {
                size: Uint128::zero(),
                object_count: Uint128::zero(),
            },
        })
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

impl From<Limits> for BucketLimits {
    fn from(limits: Limits) -> Self {
        BucketLimits {
            max_total_size: limits.max_total_size,
            max_objects: limits.max_objects,
            max_object_size: limits.max_object_size,
            max_object_pins: limits.max_object_pins,
        }
    }
}

/// Pagination is the type carrying configuration for paginated queries.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Pagination {
    /// The maximum elements a page can contains.
    pub max_page_size: u32,
    /// The default number of elements in a page.
    pub default_page_size: u32,
}

impl From<Pagination> for PaginationConfig {
    fn from(value: Pagination) -> Self {
        PaginationConfig {
            max_page_size: Some(value.max_page_size),
            default_page_size: Some(value.default_page_size),
        }
    }
}

impl From<PaginationConfig> for Pagination {
    fn from(value: PaginationConfig) -> Self {
        Pagination {
            max_page_size: value.max_page_size(),
            default_page_size: value.default_page_size(),
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
    /// The number of pin on this object.
    pub pin_count: Uint128,
}

impl From<&Object> for ObjectResponse {
    fn from(object: &Object) -> Self {
        ObjectResponse {
            id: object.id.clone(),
            size: object.size,
            owner: object.owner.clone().into(),
            is_pinned: object.pin_count > Uint128::zero(),
        }
    }
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Pin {
    /// The id of the object.
    pub id: String,
    /// The address that pinned the object.
    pub address: Addr,
}

pub struct PinIndexes<'a> {
    pub object: MultiIndex<'a, String, Pin, (String, Addr)>,
}

impl IndexList<Pin> for PinIndexes<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Pin>> + '_> {
        Box::new(vec![&self.object as &dyn Index<Pin>].into_iter())
    }
}

pub fn pins<'a>() -> IndexedMap<'a, (String, Addr), Pin, PinIndexes<'a>> {
    IndexedMap::new(
        "PIN",
        PinIndexes {
            object: MultiIndex::new(|_, pin| pin.id.clone(), "PIN", "PIN__OBJECT"),
        },
    )
}
