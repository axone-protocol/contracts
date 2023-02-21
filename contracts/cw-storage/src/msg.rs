use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;
use cosmwasm_std::Uint128;

/// Instantiate messages
#[cw_serde]
pub struct InstantiateMsg {
    /// The name of the bucket.
    pub bucket: String,
    /// The address of the owner of the bucket.
    pub owner: String,
}

/// ObjectId is the type of identifier of an object in the bucket.
type ObjectId = String;

/// Cursor is the opaque type of cursor used for pagination.
type Cursor = String;

/// Execute messages
#[cw_serde]
pub enum ExecuteMsg {
    /// # StoreObject
    /// StoreObject store an object to the bucket and make the sender the owner of the object.
    /// The object is referenced by the hash of its content and this value is returned.
    /// If the object is already stored, this is a no-op.
    /// If pin is true, the object is pinned for the sender.
    StoreObject { object: Binary, pin: bool },

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
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: Cursor,
    pub end_cursor: Cursor,
}

/// # ObjectResponse
/// ObjectResponse is the response of the GetObject query.
#[cw_serde]
pub struct ObjectResponse {
    pub id: ObjectId,
    pub owner: String,
    pub is_pinned: bool,
    pub size: Uint128,
}

/// # ObjectsResponse
/// GetObjectsResponse is the response of the GetObjects query.
#[cw_serde]
pub struct ObjectsResponse {
    /// The list of objects in the bucket.
    pub data: Vec<ObjectResponse>,
    pub page_info: PageInfo,
}

/// # ObjectPinsResponse
/// ObjectPinsResponse is the response of the GetObjectPins query.
#[cw_serde]
pub struct ObjectPinsResponse {
    /// The list of addresses that pinned the object.
    pub data: Vec<String>,
    pub page_info: PageInfo,
}
