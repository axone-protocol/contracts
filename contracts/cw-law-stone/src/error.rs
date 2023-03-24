use cosmwasm_std::StdError;
use cw_utils::ParseReplyError;
use serde_json_wasm::de::Error;
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

    #[error("Invalid reply message: {0}")]
    InvalidReplyMsg(StdError),

    #[error("Failed parse dependency uri {uri:?}: {error:?}")]
    LogicLoadUri { error: UriError, uri: String },
}

impl ContractError {
    pub fn dependency_uri(error: UriError, uri: String) -> ContractError {
        ContractError::LogicLoadUri { error, uri }
    }
}
#[derive(Error, Debug)]
pub enum UriError {
    #[error("{0}")]
    Parse(#[from] ParseError),

    #[error("Incompatible uri scheme {scheme:?}. Should be {wanted:?}")]
    WrongScheme { scheme: String, wanted: Vec<String> },

    #[error("The given path doesn't correspond to a cw-storage uri")]
    IncompatiblePath,

    #[error("URI doesn't contains needed query key")]
    MissingQueryKey,

    #[error("{0}")]
    JSONDecoding(#[from] Error),

    #[error("The given query is not compatible")]
    IncompatibleQuery,
}
