use cosmwasm_std::{StdError, StdResult};
use cw_storage_plus::{Key, KeyDeserialize, Prefixer, PrimaryKey};
use md5;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2;
use sha2::Digest;
use std::any::type_name;
use std::fmt;

/// HashAlgorithm is the type of the hash algorithm.
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

impl HashAlgorithm {
    /// hash returns the hash of the given data using the given algorithm.
    pub fn hash_fn(&self) -> HashFn {
        match self {
            HashAlgorithm::MD5 => md5_hash,
            HashAlgorithm::Sha224 => sha224_hash,
            HashAlgorithm::Sha256 => sha256_hash,
            HashAlgorithm::Sha384 => sha384_hash,
            HashAlgorithm::Sha512 => sha512_hash,
        }
    }
}

/// Hash represent a Object hash as binary value.
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, JsonSchema,
)]
pub struct Hash(Vec<u8>);

/// HashFn is the type of the function used to hash data.
pub type HashFn = fn(&Vec<u8>) -> Hash;

/// hash returns the hash of the given data using the given algorithm.
pub fn hash<'a>(algorithm: &'a HashAlgorithm, data: &'a Vec<u8>) -> Hash {
    algorithm.hash_fn()(data)
}

/// md5_hash returns the MD5 hash of the given data.
fn md5_hash(data: &Vec<u8>) -> Hash {
    md5::Md5::digest(data).to_vec().into()
}

/// sha224_hash returns the SHA-224 hash of the given data.
fn sha224_hash(data: &Vec<u8>) -> Hash {
    sha2::Sha224::digest(data).to_vec().into()
}

/// sha256_hash returns the SHA-256 hash of the given data.
fn sha256_hash(data: &Vec<u8>) -> Hash {
    sha2::Sha256::digest(data).to_vec().into()
}

/// sha384_hash returns the SHA-384 hash of the given data.
fn sha384_hash(data: &Vec<u8>) -> Hash {
    sha2::Sha384::digest(data).to_vec().into()
}

/// sha512_hash returns the SHA-512 hash of the given data.
fn sha512_hash(data: &Vec<u8>) -> Hash {
    sha2::Sha512::digest(data).to_vec().into()
}

impl TryFrom<String> for Hash {
    type Error = StdError;

    fn try_from(s: String) -> StdResult<Hash> {
        base16ct::lower::decode_vec(s)
            .map_err(|e| StdError::parse_err(type_name::<Vec<u8>>(), e.to_string()))
            .map(Hash)
    }
}

// Allows for a (user-friendly) string representation of Hash as a lower Base16 (hex) encoding.
impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex_string = base16ct::lower::encode_string(&self.0);
        write!(f, "{}", hex_string)
    }
}

impl From<Vec<u8>> for Hash {
    fn from(hash: Vec<u8>) -> Self {
        Hash(hash)
    }
}

impl From<Hash> for Vec<u8> {
    fn from(hash: Hash) -> Self {
        hash.0
    }
}

impl From<&Hash> for Vec<u8> {
    fn from(hash: &Hash) -> Self {
        hash.0.clone()
    }
}

impl<'a> PrimaryKey<'a> for Hash {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key<'_>> {
        vec![Key::Ref(self.0.as_ref())]
    }
}

impl KeyDeserialize for Hash {
    type Output = Hash;
    const KEY_ELEMS: u16 = 1;

    #[inline(always)]
    fn from_vec(value: Vec<u8>) -> StdResult<Self::Output> {
        Ok(Hash(value))
    }
}

impl KeyDeserialize for &Hash {
    type Output = Hash;
    const KEY_ELEMS: u16 = 1;

    #[inline(always)]
    fn from_vec(value: Vec<u8>) -> StdResult<Self::Output> {
        Ok(Hash(value))
    }
}

impl<'a> Prefixer<'a> for Hash {
    fn prefix(&self) -> Vec<Key<'_>> {
        vec![Key::Ref(self.0.as_ref())]
    }
}

impl AsRef<[u8]> for Hash {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::Hash;

    #[test]
    fn vec_from_hash() {
        let h = Hash(vec![1, 2, 3]);
        let result: Vec<u8> = h.into();
        assert_eq!(result, vec![1, 2, 3]);

        let h = &Hash(vec![3, 2, 1]);
        let result: Vec<u8> = h.into();
        assert_eq!(result, vec![3, 2, 1])
    }
}
