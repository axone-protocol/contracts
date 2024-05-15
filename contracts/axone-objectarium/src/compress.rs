use std::io;

use enum_iterator::Sequence;
use lzma_rs;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use snap;
use thiserror::Error;

/// CompressionAlgorithm is an enumeration that defines the different compression algorithms
/// supported for compressing the content of objects.
#[derive(Serialize, Copy, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Sequence)]
pub enum CompressionAlgorithm {
    /// Represents the "No compression" algorithm.
    Passthrough,
    /// Represents the Snappy algorithm.
    Snappy,
    /// Represents the LZMA algorithm.
    Lzma,
}

impl CompressionAlgorithm {
    /// compress returns the compressed data using the given algorithm.
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let compressor = match self {
            CompressionAlgorithm::Passthrough => passthrough,
            CompressionAlgorithm::Snappy => snappy_compress,
            CompressionAlgorithm::Lzma => lzma_compress,
        };
        compressor(data)
    }

    /// decompress returns the decompressed data using the given algorithm.
    /// The data must be compressed using the same algorithm.
    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let decompressor = match self {
            CompressionAlgorithm::Passthrough => passthrough,
            CompressionAlgorithm::Snappy => snappy_decompress,
            CompressionAlgorithm::Lzma => lzma_decompress,
        };
        decompressor(data)
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CompressionError {
    #[error("{0}")]
    Error(String),
}

impl From<io::Error> for CompressionError {
    fn from(err: io::Error) -> Self {
        CompressionError::Error(err.to_string())
    }
}

impl From<lzma_rs::error::Error> for CompressionError {
    fn from(err: lzma_rs::error::Error) -> Self {
        CompressionError::Error(err.to_string())
    }
}

/// pass_through returns the data as is.
#[inline]
#[allow(clippy::unnecessary_wraps)]
fn passthrough(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    Ok(data.to_vec())
}

// snappy_compress returns the Snappy compressed data.
#[inline]
fn snappy_compress(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    let mut reader = io::Cursor::new(data);
    let mut writer = Vec::new();
    {
        let mut snappy_writer = snap::write::FrameEncoder::new(&mut writer);
        io::copy(&mut reader, &mut snappy_writer)?;
    }
    Ok(writer)
}

// snappy_decompress returns the Snappy decompressed data.
#[inline]
fn snappy_decompress(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    let reader = io::Cursor::new(data);
    let mut snappy_reader = snap::read::FrameDecoder::new(reader);
    let mut writer = Vec::new();
    io::copy(&mut snappy_reader, &mut writer)?;
    Ok(writer)
}

// lzma_compress returns the LZMA compressed data.
#[inline]
fn lzma_compress(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    let mut reader = io::Cursor::new(data);
    let mut writer = Vec::new();
    lzma_rs::lzma_compress(&mut reader, &mut writer)?;
    Ok(writer)
}

// lzma_decompress returns the LZMA decompressed data.
#[inline]
fn lzma_decompress(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    let mut reader = io::Cursor::new(data);
    let mut writer = Vec::new();
    lzma_rs::lzma_decompress(&mut reader, &mut writer)?;
    Ok(writer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_io_decompress_error() {
        let cases = vec![
            (
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "the expected decompressed size differs, actual 998, expected 1000",
                ),
                CompressionError::Error(
                    "the expected decompressed size differs, actual 998, expected 1000".to_string(),
                ),
            ),
            (
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    lzma_rs::error::Error::IoError(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "the expected decompressed size differs, actual 998, expected 1000",
                    )),
                ),
                CompressionError::Error(
                    "io error: the expected decompressed size differs, actual 998, expected 1000"
                        .to_string(),
                ),
            ),
        ];

        for (error, expected_error) in cases {
            let compression_err = CompressionError::from(error);

            assert_eq!(compression_err, expected_error);
        }
    }
}
