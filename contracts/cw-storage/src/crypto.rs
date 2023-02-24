use sha2::{Sha256, Digest};

pub fn sha256_hash(data: &Vec<u8>) -> String {
    base16ct::lower::encode_string(&Sha256::digest(data))
}
