use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Uint128};
use derive_builder::Builder;

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
    /// The configuration of the bucket.
    pub config: BucketConfig,
    /// The limits of the bucket.
    pub limits: BucketLimits,
    /// The configuration for paginated query.
    pub pagination: PaginationConfig,
}

/// Execute messages
#[cw_serde]
pub enum ExecuteMsg {
    /// # StoreObject
    /// StoreObject store an object to the bucket and make the sender the owner of the object.
    /// The object is referenced by the hash of its content and this value is returned.
    /// If the object is already stored, an error is returned.
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
        first: Option<u32>,
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
        first: Option<u32>,
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
    /// The cursor to the next page.
    pub cursor: Cursor,
}

/// # BucketResponse
/// BucketResponse is the response of the Bucket query.
#[cw_serde]
pub struct BucketResponse {
    /// The name of the bucket.
    pub name: String,
    /// The configuration of the bucket.
    pub config: BucketConfig,
    /// The limits of the bucket.
    pub limits: BucketLimits,
    /// The configuration for paginated query.
    pub pagination: PaginationConfig,
}

/// HashAlgorithm is an enumeration that defines the different hash algorithms
/// supported for hashing the content of objects.
#[cw_serde]
#[derive(Copy)]
pub enum HashAlgorithm {
    /// Represents the MD5 algorithm.
    MD5,
    /// Represents the SHA-224 algorithm.
    Sha224,
    /// Represents the SHA-256 algorithm.
    Sha256,
    /// Represents the SHA-384 algorithm.
    Sha384,
    /// Represents the SHA-512 algorithm.
    Sha512,
}

/// BucketConfig is the type of the configuration of a bucket.
///
/// The configuration is set at the instantiation of the bucket, and is immutable and cannot be changed.
/// The configuration is optional and if not set, the default configuration is used.
#[cw_serde]
#[derive(Default, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct BucketConfig {
    /// The algorithm used to hash the content of the objects to generate the id of the objects.
    /// The algorithm is optional and if not set, the default algorithm is used.
    ///
    /// The default algorithm is Sha256 .
    pub hash_algorithm: Option<HashAlgorithm>,
}

/// BucketLimits is the type of the limits of a bucket.
///
/// The limits are optional and if not set, there is no limit.
#[cw_serde]
#[derive(Default, Builder)]
#[builder(default, setter(into, strip_option))]
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

/// PaginationConfig is the type carrying configuration for paginated queries.
///
/// The fields are optional and if not set, there is a default configuration.
#[cw_serde]
#[derive(Default, Builder)]
#[builder(default, setter(strip_option))]
pub struct PaginationConfig {
    /// The maximum elements a page can contains.
    ///
    /// Shall be less than `u32::MAX - 1`.
    /// Default to '30' if not set.
    pub max_page_size: Option<u32>,
    /// The default number of elements in a page.
    ///
    /// Shall be less or equal than `max_page_size`.
    /// Default to '10' if not set.
    pub default_page_size: Option<u32>,
}

impl PaginationConfig {
    const DEFAULT_PAGE_MAX_SIZE: u32 = 30;
    const DEFAULT_PAGE_DEFAULT_SIZE: u32 = 10;

    pub fn max_page_size_or_default(&self) -> u32 {
        self.max_page_size.unwrap_or(Self::DEFAULT_PAGE_MAX_SIZE)
    }

    pub fn default_page_size_or_default(&self) -> u32 {
        self.default_page_size
            .unwrap_or(Self::DEFAULT_PAGE_DEFAULT_SIZE)
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
