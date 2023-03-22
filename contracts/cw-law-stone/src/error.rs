use cosmwasm_std::StdError;
use cw_utils::ParseReplyError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Not implemented")]
    NotImplemented {},

    #[error("{0}")]
    Parse(#[from] ParseReplyError),

    #[error("Could not find ObjectId of stored program")]
    NoObjectId,

    #[error("Empty data on reply")]
    EmptyReply,

    #[error("Invalid reply message: {0}")]
    InvalidReplyMsg(StdError),
}
