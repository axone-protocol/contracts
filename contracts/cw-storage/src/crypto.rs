use sha2::{Digest, Sha256};

pub fn sha256_hash(data: &Vec<u8>) -> String {
    base16ct::lower::encode_string(&Sha256::digest(data))
}
