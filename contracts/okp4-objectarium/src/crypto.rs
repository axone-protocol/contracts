use md5;
use sha2;
use sha2::Digest;
use std::fmt::{Display, Formatter};

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

pub type Hash = Vec<u8>;

/// HashFn is the type of the function used to hash data.
pub type HashFn = fn(&Vec<u8>) -> Hash;

/// hash returns the hash of the given data using the given algorithm.
pub fn hash<'a>(algorithm: &'a HashAlgorithm, data: &'a Vec<u8>) -> Hash {
    algorithm.hash_fn()(data)
}

/// md5_hash returns the MD5 hash of the given data.
fn md5_hash(data: &Vec<u8>) -> Hash {
    md5::Md5::digest(data).to_vec()
}

/// sha224_hash returns the SHA-224 hash of the given data.
fn sha224_hash(data: &Vec<u8>) -> Hash {
    sha2::Sha224::digest(data).to_vec()
}

/// sha256_hash returns the SHA-256 hash of the given data.
fn sha256_hash(data: &Vec<u8>) -> Hash {
    sha2::Sha256::digest(data).to_vec()
}

/// sha384_hash returns the SHA-384 hash of the given data.
fn sha384_hash(data: &Vec<u8>) -> Hash {
    sha2::Sha384::digest(data).to_vec()
}

/// sha512_hash returns the SHA-512 hash of the given data.
fn sha512_hash(data: &Vec<u8>) -> Hash {
    sha2::Sha512::digest(data).to_vec()
}
