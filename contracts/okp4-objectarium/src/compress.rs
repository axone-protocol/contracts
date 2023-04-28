use enum_iterator::Sequence;
use lz4_flex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// CompressionAlgorithm is an enumeration that defines the different compression algorithms
/// supported for compressing the content of objects.
#[derive(Serialize, Copy, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Sequence)]
pub enum CompressionAlgorithm {
    /// Represents the "No compression" algorithm.
    Passthrough,
    /// Represents the LZ4 algorithm.
    Lz4,
}

impl CompressionAlgorithm {
    /// compress returns the compressed data using the given algorithm.
    pub fn compress(&self, data: &[u8]) -> Result<Box<[u8]>, CompressionError> {
        let compressor = match self {
            CompressionAlgorithm::Passthrough => passthrough,
            CompressionAlgorithm::Lz4 => lz4_compress,
        };
        compressor(data)
    }

    /// decompress returns the decompressed data using the given algorithm.
    /// The data must be compressed using the same algorithm.
    pub fn decompress(&self, data: &[u8]) -> Result<Box<[u8]>, CompressionError> {
        let decompressor = match self {
            CompressionAlgorithm::Passthrough => passthrough,
            CompressionAlgorithm::Lz4 => lz4_decompress,
        };
        decompressor(data)
    }
}

#[derive(Debug)]
pub enum CompressionError {
    Error(String),
}

impl From<lz4_flex::block::DecompressError> for CompressionError {
    fn from(err: lz4_flex::block::DecompressError) -> Self {
        CompressionError::Error(err.to_string())
    }
}

/// pass_through returns the data as is.
fn passthrough(data: &[u8]) -> Result<Box<[u8]>, CompressionError> {
    Ok(data.to_vec().into_boxed_slice())
}

// lz4_compress returns the LZ4 compressed data.
fn lz4_compress(data: &[u8]) -> Result<Box<[u8]>, CompressionError> {
    Ok(lz4_flex::compress_prepend_size(data).into())
}

// lz4_decompress returns the LZ4 decompressed data.
fn lz4_decompress(data: &[u8]) -> Result<Box<[u8]>, CompressionError> {
    Ok(lz4_flex::decompress_size_prepended(data)?.into())
}
