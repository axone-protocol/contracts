use enum_iterator::Sequence;
use lz4_flex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let compressor = match self {
            CompressionAlgorithm::Passthrough => passthrough,
            CompressionAlgorithm::Lz4 => lz4_compress,
        };
        compressor(data)
    }

    /// decompress returns the decompressed data using the given algorithm.
    /// The data must be compressed using the same algorithm.
    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let decompressor = match self {
            CompressionAlgorithm::Passthrough => passthrough,
            CompressionAlgorithm::Lz4 => lz4_decompress,
        };
        decompressor(data)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum CompressionError {
    #[error("{0}")]
    Error(String),
}

impl From<lz4_flex::block::DecompressError> for CompressionError {
    fn from(err: lz4_flex::block::DecompressError) -> Self {
        CompressionError::Error(err.to_string())
    }
}

/// pass_through returns the data as is.
#[inline]
fn passthrough(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    Ok(data.to_vec())
}

// lz4_compress returns the LZ4 compressed data.
#[inline]
fn lz4_compress(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    Ok(lz4_flex::compress_prepend_size(data))
}

// lz4_decompress returns the LZ4 decompressed data.
#[inline]
fn lz4_decompress(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    Ok(lz4_flex::decompress_size_prepended(data)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_lz4_flex_decompress_error() {
        // Define test cases as tuples of input DecompressError and expected CompressionError
        let test_cases = vec![(
            lz4_flex::block::DecompressError::UncompressedSizeDiffers {
                expected: 1000,
                actual: 998,
            },
            CompressionError::Error(
                "the expected decompressed size differs, actual 998, expected 1000".to_string(),
            ),
        )];

        // Iterate over test cases using a for loop
        for (error, expected_error) in test_cases {
            let compression_err = CompressionError::from(error);

            assert_eq!(compression_err, expected_error);
        }
    }
}
