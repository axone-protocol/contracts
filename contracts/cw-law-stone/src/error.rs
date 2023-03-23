use cosmwasm_std::StdError;
use cw_utils::ParseReplyError;
use thiserror::Error;
use url::ParseError;

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

    #[error("Failed parse dependency uri {uri:?}: {error:?}")]
    DependencyUri {
        error: UriError,
        uri: String
    }
}

#[derive(Error, Debug)]
pub enum UriError {
    #[error("{0}")]
    Parse(#[from] ParseError),

    #[error("Incompatible uri scheme {scheme:?}. Should be {wanted:?}")]
    WrongScheme{ scheme: String, wanted: Vec<String> }
}

