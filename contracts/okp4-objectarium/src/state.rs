use crate::compress::CompressionAlgorithm;
use crate::crypto::Hash;
use crate::error::BucketError;
use crate::error::BucketError::EmptyName;
use crate::msg;
use crate::msg::{ObjectResponse, PaginationConfig};
use cosmwasm_std::{Addr, StdError, StdResult, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use enum_iterator::all;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const DATA: Map<Hash, Vec<u8>> = Map::new("DATA");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Bucket {
    /// The owner of the bucket.
    pub owner: Addr,
    /// The name of the bucket.
    pub name: String,
    /// The configuration for the bucket.
    pub config: BucketConfig,
    /// The limits of the bucket.
    pub limits: BucketLimits,
    /// The configuration for paginated query.
    pub pagination: Pagination,
    /// Some information on the current bucket usage.
    pub stat: BucketStat,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct BucketStat {
    /// The total size of the objects contained in the bucket.
    pub size: Uint128,
    /// The total size of the objects contained in the bucket after compression.
    pub compressed_size: Uint128,
    /// The number of objects in the bucket.
    pub object_count: Uint128,
}

impl Bucket {
    pub fn try_new(
        owner: Addr,
        name: String,
        config: BucketConfig,
        limits: BucketLimits,
        pagination: Pagination,
    ) -> Result<Self, BucketError> {
        let n: String = name.split_whitespace().collect();
        if n.is_empty() {
            return Err(EmptyName);
        }

        Ok(Self {
            owner,
            name: n,
            config,
            limits,
            pagination,
            stat: BucketStat {
                size: Uint128::zero(),
                compressed_size: Uint128::zero(),
                object_count: Uint128::zero(),
            },
        })
    }
}

/// HashAlgorithm is an enumeration that defines the different hash algorithms
/// supported for hashing the content of objects.
#[derive(Serialize, Copy, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
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

impl Default for HashAlgorithm {
    fn default() -> Self {
        HashAlgorithm::Sha256
    }
}

impl From<msg::HashAlgorithm> for HashAlgorithm {
    fn from(algorithm: msg::HashAlgorithm) -> Self {
        match algorithm {
            msg::HashAlgorithm::MD5 => HashAlgorithm::MD5,
            msg::HashAlgorithm::Sha224 => HashAlgorithm::Sha224,
            msg::HashAlgorithm::Sha256 => HashAlgorithm::Sha256,
            msg::HashAlgorithm::Sha384 => HashAlgorithm::Sha384,
            msg::HashAlgorithm::Sha512 => HashAlgorithm::Sha512,
        }
    }
}

impl From<HashAlgorithm> for msg::HashAlgorithm {
    fn from(algorithm: HashAlgorithm) -> Self {
        match algorithm {
            HashAlgorithm::MD5 => msg::HashAlgorithm::MD5,
            HashAlgorithm::Sha224 => msg::HashAlgorithm::Sha224,
            HashAlgorithm::Sha256 => msg::HashAlgorithm::Sha256,
            HashAlgorithm::Sha384 => msg::HashAlgorithm::Sha384,
            HashAlgorithm::Sha512 => msg::HashAlgorithm::Sha512,
        }
    }
}

impl From<msg::CompressionAlgorithm> for CompressionAlgorithm {
    fn from(algorithm: msg::CompressionAlgorithm) -> Self {
        match algorithm {
            msg::CompressionAlgorithm::Passthrough => CompressionAlgorithm::Passthrough,
            msg::CompressionAlgorithm::Snappy => CompressionAlgorithm::Snappy,
        }
    }
}

impl From<CompressionAlgorithm> for msg::CompressionAlgorithm {
    fn from(algorithm: CompressionAlgorithm) -> Self {
        match algorithm {
            CompressionAlgorithm::Passthrough => msg::CompressionAlgorithm::Passthrough,
            CompressionAlgorithm::Snappy => msg::CompressionAlgorithm::Snappy,
        }
    }
}

impl CompressionAlgorithm {
    pub fn values() -> Vec<CompressionAlgorithm> {
        all::<CompressionAlgorithm>().collect::<Vec<_>>()
    }
}

/// BucketConfig is the type of the configuration of a bucket.
///
/// The configuration is set at the instantiation of the bucket, and is immutable and cannot be changed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct BucketConfig {
    /// The algorithm used to hash the content of the objects to generate the id of the objects.
    /// The algorithm is optional and if not set, the default algorithm is used.
    ///
    /// The default algorithm is Sha256 .
    pub hash_algorithm: Option<HashAlgorithm>,
}

impl BucketConfig {
    pub fn hash_algorithm_or_default(&self) -> HashAlgorithm {
        self.hash_algorithm.as_ref().copied().unwrap_or_default()
    }
}

impl From<msg::BucketConfig> for BucketConfig {
    fn from(config: msg::BucketConfig) -> Self {
        BucketConfig {
            hash_algorithm: config.hash_algorithm.map(|a| a.into()),
        }
    }
}

impl From<BucketConfig> for msg::BucketConfig {
    fn from(config: BucketConfig) -> Self {
        msg::BucketConfig {
            hash_algorithm: config.hash_algorithm.map(|a| a.into()),
        }
    }
}

/// BucketLimits is the type of the limits of a bucket.
///
/// The limits are optional and if not set, there is no limit.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct BucketLimits {
    /// The maximum total size of the objects in the bucket.
    pub max_total_size: Option<Uint128>,
    /// The maximum number of objects in the bucket.
    pub max_objects: Option<Uint128>,
    /// The maximum size of the objects in the bucket.
    pub max_object_size: Option<Uint128>,
    /// The maximum number of pins in the bucket for an object.
    pub max_object_pins: Option<Uint128>,
    /// The accepted compression algorithms for the objects in the bucket.
    pub accepted_compression_algorithms: Vec<CompressionAlgorithm>,
}

impl From<msg::BucketLimits> for BucketLimits {
    fn from(limits: msg::BucketLimits) -> Self {
        BucketLimits {
            max_total_size: limits.max_total_size,
            max_objects: limits.max_objects,
            max_object_size: limits.max_object_size,
            max_object_pins: limits.max_object_pins,
            accepted_compression_algorithms: limits
                .accepted_compression_algorithms
                .map(|it| it.into_iter().map(Into::into).collect())
                .unwrap_or_else(CompressionAlgorithm::values),
        }
    }
}

impl From<BucketLimits> for msg::BucketLimits {
    fn from(limits: BucketLimits) -> Self {
        msg::BucketLimits {
            max_total_size: limits.max_total_size,
            max_objects: limits.max_objects,
            max_object_size: limits.max_object_size,
            max_object_pins: limits.max_object_pins,
            accepted_compression_algorithms: Some(
                limits
                    .accepted_compression_algorithms
                    .into_iter()
                    .map(|a| a.into())
                    .collect::<Vec<_>>(),
            ),
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

const MAX_PAGE_MAX_SIZE: u32 = u32::MAX - 1;

impl Pagination {
    fn try_new(max_page_size: u32, default_page_size: u32) -> StdResult<Pagination> {
        if max_page_size > MAX_PAGE_MAX_SIZE {
            return Err(StdError::generic_err(
                "'max_page_size' cannot exceed 'u32::MAX - 1'",
            ));
        }

        if default_page_size > max_page_size {
            return Err(StdError::generic_err(
                "'default_page_size' cannot exceed 'max_page_size'",
            ));
        }

        Ok(Pagination {
            max_page_size,
            default_page_size,
        })
    }
}

impl From<Pagination> for PaginationConfig {
    fn from(value: Pagination) -> Self {
        PaginationConfig {
            max_page_size: value.max_page_size,
            default_page_size: value.default_page_size,
        }
    }
}

impl TryFrom<PaginationConfig> for Pagination {
    type Error = StdError;

    fn try_from(value: PaginationConfig) -> StdResult<Pagination> {
        Pagination::try_new(value.max_page_size, value.default_page_size)
    }
}

pub const BUCKET: Item<Bucket> = Item::new("bucket");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Object {
    /// The id of the object.
    pub id: Hash,
    /// The owner of the object.
    pub owner: Addr,
    /// The size of the object.
    pub size: Uint128,
    /// The number of pin on this object.
    pub pin_count: Uint128,
    /// The compression algorithm used to compress the object.
    pub compression: CompressionAlgorithm,
    /// The size of the object after compression.
    pub compressed_size: Uint128,
}

impl From<&Object> for ObjectResponse {
    fn from(object: &Object) -> Self {
        ObjectResponse {
            id: object.id.clone().into(),
            size: object.size,
            owner: object.owner.clone().into(),
            is_pinned: object.pin_count > Uint128::zero(),
            compressed_size: object.compressed_size,
            compression_algorithm: object.compression.into(),
        }
    }
}

pub struct ObjectIndexes<'a> {
    pub owner: MultiIndex<'a, Addr, Object, Hash>,
}

impl IndexList<Object> for ObjectIndexes<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Object>> + '_> {
        Box::new(vec![&self.owner as &dyn Index<Object>].into_iter())
    }
}

pub fn objects<'a>() -> IndexedMap<'a, Hash, Object, ObjectIndexes<'a>> {
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
    pub id: Hash,
    /// The address that pinned the object.
    pub address: Addr,
}

pub struct PinIndexes<'a> {
    pub object: MultiIndex<'a, Hash, Pin, (Hash, Addr)>,
}

impl IndexList<Pin> for PinIndexes<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Pin>> + '_> {
        Box::new(vec![&self.object as &dyn Index<Pin>].into_iter())
    }
}

pub fn pins<'a>() -> IndexedMap<'a, (Hash, Addr), Pin, PinIndexes<'a>> {
    IndexedMap::new(
        "PIN",
        PinIndexes {
            object: MultiIndex::new(|_, pin| pin.id.clone(), "PIN", "PIN__OBJECT"),
        },
    )
}
