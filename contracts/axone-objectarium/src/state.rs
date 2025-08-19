use crate::compress::CompressionAlgorithm;
use crate::crypto::Hash;
use crate::error::BucketError;
use crate::error::BucketError::EmptyName;
use crate::msg;
use crate::msg::{ObjectResponse, PaginationConfig};
use cosmwasm_std::{ensure, ensure_ne, Addr, StdError, StdResult, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const DATA: Map<Hash, Vec<u8>> = Map::new("DATA");

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct Bucket {
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

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
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
        name: String,
        config: BucketConfig,
        limits: BucketLimits,
        pagination: Pagination,
    ) -> Result<Self, BucketError> {
        let n: String = name.split_whitespace().collect();
        ensure!(!n.is_empty(), EmptyName);

        Ok(Self {
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
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub enum HashAlgorithm {
    /// Represents the MD5 algorithm.
    MD5,
    /// Represents the SHA-224 algorithm.
    Sha224,
    /// Represents the SHA-256 algorithm.
    #[default]
    Sha256,
    /// Represents the SHA-384 algorithm.
    Sha384,
    /// Represents the SHA-512 algorithm.
    Sha512,
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
            msg::CompressionAlgorithm::Lzma => CompressionAlgorithm::Lzma,
        }
    }
}

impl From<CompressionAlgorithm> for msg::CompressionAlgorithm {
    fn from(algorithm: CompressionAlgorithm) -> Self {
        match algorithm {
            CompressionAlgorithm::Passthrough => msg::CompressionAlgorithm::Passthrough,
            CompressionAlgorithm::Snappy => msg::CompressionAlgorithm::Snappy,
            CompressionAlgorithm::Lzma => msg::CompressionAlgorithm::Lzma,
        }
    }
}

/// BucketConfig is the type of the configuration of a bucket.
///
/// The configuration is set at the instantiation of the bucket, and is immutable and cannot be changed.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct BucketConfig {
    /// The algorithm used to hash the content of the objects to generate the id of the objects.
    /// The algorithm is optional and if not set, the default algorithm is used.
    ///
    /// The default algorithm is Sha256.
    pub hash_algorithm: HashAlgorithm,
    /// The compression algorithm used for all objects in the bucket.
    /// All objects stored in the bucket will use this compression algorithm.
    /// The default algorithm is Passthrough.
    pub compression_algorithm: CompressionAlgorithm,
}

impl BucketConfig {
    fn new(
        hash_algorithm: HashAlgorithm,
        compression_algorithm: CompressionAlgorithm,
    ) -> BucketConfig {
        BucketConfig {
            hash_algorithm,
            compression_algorithm,
        }
    }
}

impl From<msg::BucketConfig> for BucketConfig {
    fn from(config: msg::BucketConfig) -> BucketConfig {
        BucketConfig::new(
            config.hash_algorithm.into(),
            config.compression_algorithm.into(),
        )
    }
}

impl From<BucketConfig> for msg::BucketConfig {
    fn from(config: BucketConfig) -> Self {
        msg::BucketConfig {
            hash_algorithm: config.hash_algorithm.into(),
            compression_algorithm: config.compression_algorithm.into(),
        }
    }
}

/// BucketLimits is the type of the limits of a bucket.
///
/// The limits are optional and if not set, there is no limit.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
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

impl From<BucketLimits> for msg::BucketLimits {
    fn from(limits: BucketLimits) -> Self {
        msg::BucketLimits {
            max_total_size: limits.max_total_size,
            max_objects: limits.max_objects,
            max_object_size: limits.max_object_size,
            max_object_pins: limits.max_object_pins,
        }
    }
}

impl From<BucketStat> for msg::BucketStat {
    fn from(stat: BucketStat) -> Self {
        msg::BucketStat {
            size: stat.size,
            compressed_size: stat.compressed_size,
            object_count: stat.object_count,
        }
    }
}
impl BucketLimits {
    fn try_new(
        max_total_size: Option<Uint128>,
        max_objects: Option<Uint128>,
        max_object_size: Option<Uint128>,
        max_object_pins: Option<Uint128>,
    ) -> StdResult<BucketLimits> {
        ensure_ne!(
            max_total_size,
            Some(Uint128::zero()),
            StdError::generic_err("'max_total_size' cannot be zero")
        );
        ensure_ne!(
            max_objects,
            Some(Uint128::zero()),
            StdError::generic_err("'max_objects' cannot be zero")
        );
        ensure_ne!(
            max_object_size,
            Some(Uint128::zero()),
            StdError::generic_err("'max_object_size' cannot be zero")
        );
        ensure!(
            !matches!(
                (max_total_size, max_object_size),
                (Some(max_total_size), Some(max_object_size)) if max_total_size < max_object_size
            ),
            StdError::generic_err("'max_total_size' cannot be less than 'max_object_size'")
        );

        Ok(BucketLimits {
            max_total_size,
            max_objects,
            max_object_size,
            max_object_pins,
        })
    }
}

impl TryFrom<msg::BucketLimits> for BucketLimits {
    type Error = StdError;

    fn try_from(limits: msg::BucketLimits) -> StdResult<BucketLimits> {
        BucketLimits::try_new(
            limits.max_total_size,
            limits.max_objects,
            limits.max_object_size,
            limits.max_object_pins,
        )
    }
}
/// Pagination is the type carrying configuration for paginated queries.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct Pagination {
    /// The maximum elements a page can contain.
    pub max_page_size: u32,
    /// The default number of elements in a page.
    pub default_page_size: u32,
}

const MAX_PAGE_MAX_SIZE: u32 = u32::MAX - 1;

impl Pagination {
    fn try_new(max_page_size: u32, default_page_size: u32) -> StdResult<Pagination> {
        ensure!(
            max_page_size <= MAX_PAGE_MAX_SIZE,
            StdError::generic_err("'max_page_size' cannot exceed 'u32::MAX - 1'")
        );
        ensure_ne!(
            default_page_size,
            0,
            StdError::generic_err("'default_page_size' cannot be zero")
        );
        ensure!(
            default_page_size <= max_page_size,
            StdError::generic_err("'default_page_size' cannot exceed 'max_page_size'")
        );

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

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct Object {
    /// The id of the object.
    pub id: Hash,
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
            id: object.id.to_string(),
            size: object.size,
            is_pinned: object.pin_count > Uint128::zero(),
            compressed_size: object.compressed_size,
        }
    }
}

pub const OBJECT: Map<Hash, Object> = Map::new("OBJECT");

#[derive(Clone, Deserialize, Serialize)]
pub struct Pin {
    /// The id of the object.
    pub id: Hash,
    /// The address that pinned the object.
    pub address: Addr,
}

/// The primary key for a pin is a tuple of the object id and the address that pinned it.
pub type PinPK = (Hash, Addr);

pub struct PinIndexes<'a> {
    pub object: MultiIndex<'a, Hash, Pin, PinPK>,
    pub address: MultiIndex<'a, Addr, Pin, PinPK>,
}

impl IndexList<Pin> for PinIndexes<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Pin>> + '_> {
        let object: &dyn Index<Pin> = &self.object;
        let address: &dyn Index<Pin> = &self.address;
        Box::new(vec![object, address].into_iter())
    }
}

pub fn pins<'a>() -> IndexedMap<PinPK, Pin, PinIndexes<'a>> {
    IndexedMap::new(
        "PIN",
        PinIndexes {
            object: MultiIndex::new(|_, p| p.id.clone(), "PIN", "PIN__OBJECT"),
            address: MultiIndex::new(|_, p| p.address.clone(), "PIN", "PIN__ADDRESS"),
        },
    )
}
