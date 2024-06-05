use crate::compress::CompressionError;
use crate::msg::CompressionAlgorithm;
use cosmwasm_std::{StdError, Uint128};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Bucket(#[from] BucketError),

    #[error("Object is pinned and cannot be forgotten")]
    ObjectPinned {},

    #[error("Compression error: {0}")]
    CompressionError(String),

    #[error("{0}")]
    Payment(#[from] PaymentError),
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum BucketError {
    #[error("Name of bucket could not be empty")]
    EmptyName,

    #[error("Maximum total size exceeded: {0} / {1}")]
    MaxTotalSizeLimitExceeded(Uint128, Uint128),

    #[error("Maximum objects number exceeded: {0} / {1}")]
    MaxObjectsLimitExceeded(Uint128, Uint128),

    #[error("Maximum object size exceeded: {0} / {1}")]
    MaxObjectSizeLimitExceeded(Uint128, Uint128),

    #[error("Maximum object pins number exceeded: {0} / {1}")]
    MaxObjectPinsLimitExceeded(Uint128, Uint128),

    #[error("Compression algorithm is not accepted: {0:?} (accepted: \"{1:?}\")")]
    CompressionAlgorithmNotAccepted(CompressionAlgorithm, Vec<CompressionAlgorithm>),
}

impl From<CompressionError> for ContractError {
    fn from(err: CompressionError) -> Self {
        match err {
            CompressionError::Error(err) => ContractError::CompressionError(err),
        }
    }
}

#[test]
fn test_bucket_error_messages() {
    let cases = vec![
        (ContractError::Std(StdError::generic_err("Software failure. Press left mouse button to continue. Guru Mediation #8000000B.0000AAC00")),
         "Generic error: Software failure. Press left mouse button to continue. Guru Mediation #8000000B.0000AAC00"
        ),
        (
            ContractError::Bucket(BucketError::EmptyName),
            "Name of bucket could not be empty",
        ),
        (
            ContractError::Bucket(BucketError::MaxTotalSizeLimitExceeded(
                200u8.into(),
                100u8.into(),
            )),
            "Maximum total size exceeded: 200 / 100",
        ),
        (
            ContractError::Bucket(BucketError::MaxObjectsLimitExceeded(42u8.into(), 40u8.into())),
            "Maximum objects number exceeded: 42 / 40",
        ),
        (
            ContractError::Bucket(BucketError::MaxObjectSizeLimitExceeded(
                603u16.into(),
                111u16.into(),
            )),
            "Maximum object size exceeded: 603 / 111",
        ),
        (
            ContractError::Bucket(BucketError::MaxObjectPinsLimitExceeded(5u8.into(), 2u8.into())),
            "Maximum object pins number exceeded: 5 / 2",
        ),
        (
            ContractError::Bucket(BucketError::CompressionAlgorithmNotAccepted(
                CompressionAlgorithm::Snappy,
                vec![CompressionAlgorithm::Passthrough],
            )),
            "Compression algorithm is not accepted: Snappy (accepted: \"[Passthrough]\")",
        ),
        (ContractError::ObjectPinned {}, "Object is pinned and cannot be forgotten"),
        (
            ContractError::CompressionError("Insufficient ch'i to compress file".to_string()),
            "Compression error: Insufficient ch'i to compress file",
        ),
        (
            CompressionError::Error("Cannot compress empty data".to_string()).into(),
            "Compression error: Cannot compress empty data",
        ),
    ];

    for (error, expected_message) in cases {
        assert_eq!(error.to_string(), expected_message);
    }
}
