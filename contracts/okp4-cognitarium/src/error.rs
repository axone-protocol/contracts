use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Store(#[from] StoreError),

    #[error("Only the owner can perform this operation.")]
    Unauthorized {},

    #[error("Not implemented.")]
    NotImplemented,
}

#[derive(Error, Debug, PartialEq)]
pub enum StoreError {
    #[error("Maximum triples number exceeded: {0}")]
    MaxTriplesLimitExceeded(Uint128),

    #[error("Maximum byte size exceeded: {0}")]
    MaxByteSize(Uint128),

    #[error("Maximum triple byte size exceeded: {0} / {1}")]
    MaxTripleByteSize(Uint128, Uint128),

    #[error("Maximum query limit exceeded: {0} / {1}")]
    MaxQueryLimit(Uint128, Uint128),

    #[error("Maximum query variable count exceeded: {0} / {1}")]
    MaxQueryVariableCount(Uint128, Uint128),

    #[error("Maximum insert byte size exceeded: {0}")]
    MaxInsertDataByteSize(Uint128),

    #[error("Maximum insert triple count exceeded: {0}")]
    MaxInsertDataTripleCount(Uint128),
}
