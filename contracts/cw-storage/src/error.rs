use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Not implemented")]
    NotImplemented {},

    #[error("{0}")]
    Bucket(#[from] BucketError),
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum BucketError {
    #[error("Name of bucket could not be empty")]
    EmptyName,

    #[error("Maximum total size exceeded")]
    MaxTotalSizeLimitExceeded,

    #[error("Maximum objects number exceeded")]
    MaxObjectsLimitExceeded,

    #[error("Maximum object size exceeded")]
    MaxObjectSizeLimitExceeded,

    #[error("Maximum object pins number exceeded")]
    MaxObjectPinsLimitExceeded,
}
