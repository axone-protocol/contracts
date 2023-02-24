use crate::state::Limits;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;
use cosmwasm_std::Uint128;

/// ObjectId is the type of identifier of an object in the bucket.
pub type ObjectId = String;

/// Cursor is the opaque type of cursor used for pagination.
pub type Cursor = String;

/// Instantiate messages
#[cw_serde]
pub struct InstantiateMsg {
    /// The name of the bucket.
    /// The name could not be empty or contains whitespaces.
    /// If name contains whitespace, they will be removed.
    pub bucket: String,
    /// The limits of the bucket.
    pub limits: BucketLimits,
}

/// Execute messages
#[cw_serde]
pub enum ExecuteMsg {
    /// # StoreObject
    /// StoreObject store an object to the bucket and make the sender the owner of the object.
    /// The object is referenced by the hash of its content and this value is returned.
    /// If the object is already stored, this is a no-op.
    /// If pin is true, the object is pinned for the sender.
    StoreObject { data: Binary, pin: bool },

    /// # ForgetObject
    /// ForgetObject first unpin the object from the bucket for the considered sender, then remove
    /// it from the storage if it is not pinned anymore.
    /// If the object is pinned for other senders, it is not removed from the storage and an error is returned.
    /// If the object is not pinned for the sender, this is a no-op.
    ForgetObject { id: ObjectId },

    /// # PinObject
    /// PinObject pins the object in the bucket for the considered sender. If the object is already pinned
    /// for the sender, this is a no-op.
    /// While an object is pinned, it cannot be removed from the storage.
    PinObject { id: ObjectId },

    /// # UnpinObject
    /// UnpinObject unpins the object in the bucket for the considered sender. If the object is not pinned
    /// for the sender, this is a no-op.
    /// The object can be removed from the storage if it is not pinned anymore.
    UnpinObject { id: ObjectId },
}

/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// # Bucket
    /// Bucket returns the bucket information.
    #[returns(BucketResponse)]
    Bucket {},

    /// # Object
    /// Object returns the object information with the given id.
    #[returns(ObjectResponse)]
    Object {
        /// The id of the object to get.
        id: ObjectId,
    },

    /// # Objects
    /// Objects returns the list of objects in the bucket with support for pagination.
    #[returns(ObjectsResponse)]
    Objects {
        /// The owner of the objects to get.
        address: Option<String>,
        /// The number of objects to return.
        first: Option<Uint128>,
        /// The point in the sequence to start returning objects.
        after: Option<Cursor>,
    },

    /// # ObjectData
    /// ObjectData returns the content of the object with the given id.
    #[returns(Binary)]
    ObjectData {
        /// The id of the object to get.
        id: ObjectId,
    },

    /// # ObjectPins
    /// ObjectPins returns the list of addresses that pinned the object with the given id with
    /// support for pagination.
    #[returns(ObjectPinsResponse)]
    ObjectPins {
        /// The id of the object to get the pins for.
        id: ObjectId,
        /// The number of pins to return.
        first: Option<Uint128>,
        /// The point in the sequence to start returning pins.
        after: Option<Cursor>,
    },
}

/// # PageInfo
/// PageInfo is the page information returned for paginated queries.
#[cw_serde]
pub struct PageInfo {
    /// Tells if there is a next page.
    pub has_next_page: bool,
    /// Tells if there is a previous page.
    pub has_previous_page: bool,
    /// The cursor to the next page.
    pub start_cursor: Cursor,
    /// The cursor to the previous page.
    pub end_cursor: Cursor,
}

/// # BucketResponse
/// BucketResponse is the response of the Bucket query.
#[cw_serde]
pub struct BucketResponse {
    /// The name of the bucket.
    pub name: String,
    /// The limits of the bucket.
    pub limits: BucketLimits,
}

/// BucketLimits is the type of the limits of a bucket.
///
/// The limits are optional and if not set, there is no limit.
#[cw_serde]
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

/// # ObjectResponse
/// ObjectResponse is the response of the Object query.
#[cw_serde]
pub struct ObjectResponse {
    /// The id of the object.
    pub id: ObjectId,
    /// The owner of the object.
    pub owner: String,
    /// Tells if the object is pinned by at least one address.
    pub is_pinned: bool,
    /// The size of the object.
    pub size: Uint128,
}

/// # ObjectsResponse
/// ObjectsResponse is the response of the Objects query.
#[cw_serde]
pub struct ObjectsResponse {
    /// The list of objects in the bucket.
    pub data: Vec<ObjectResponse>,
    /// The page information.
    pub page_info: PageInfo,
}

/// # ObjectPinsResponse
/// ObjectPinsResponse is the response of the GetObjectPins query.
#[cw_serde]
pub struct ObjectPinsResponse {
    /// The list of addresses that pinned the object.
    pub data: Vec<String>,
    /// The page information.
    pub page_info: PageInfo,
}
