use crate::ContractError::Std;
use cosmwasm_std::StdError;
use cw_utils::ParseReplyError;
use logic_bindings::error::CosmwasmUriError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Parse(#[from] ParseReplyError),

    #[error("Invalid reply message: {0}")]
    InvalidReplyMsg(StdError),

    #[error("Cannot parse cosmwasm uri {uri:?}: {error:?}")]
    ParseCosmwasmUri {
        error: CosmwasmUriError,
        uri: String,
    },

    #[error("Only the contract admin can perform this operation.")]
    Unauthorized {},
}

impl From<ContractError> for StdError {
    fn from(value: ContractError) -> Self {
        match value {
            Std(e) => e,
            _ => StdError::generic_err(value.to_string()),
        }
    }
}
