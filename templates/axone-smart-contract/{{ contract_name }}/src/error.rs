use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
}
