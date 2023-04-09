use sha2::{Digest, Sha256, Sha512};

/// HashAlgorithm is the type of the hash algorithm.
pub enum HashAlgorithm {
    /// Represents the SHA-256 algorithm.
    Sha256,
    /// Represents the SHA-512 algorithm.
    Sha512,
}

/// HashFn is the type of the function used to hash data.
pub type HashFn = fn(&Vec<u8>) -> String;

/// hash returns the hash of the given data using the given algorithm.
/// If the algorithm is not supported, an error is returned.
pub fn hash(algorithm: &HashAlgorithm, data: &Vec<u8>) -> String {
    let hash_fn = match algorithm {
        HashAlgorithm::Sha256 => sha256_hash,
        HashAlgorithm::Sha512 => sha512_hash,
    };
    hash_fn(data)
}

/// sha256_hash returns the SHA-256 hash of the given data.
fn sha256_hash(data: &Vec<u8>) -> String {
    base16ct::lower::encode_string(&Sha256::digest(data))
}

/// sha512_hash returns the SHA-512 hash of the given data.
fn sha512_hash(data: &Vec<u8>) -> String {
    base16ct::lower::encode_string(&Sha512::digest(data))
}
