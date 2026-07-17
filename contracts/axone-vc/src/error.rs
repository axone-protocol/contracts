use crate::{
    domain::CredentialError,
    services::{IssueCredentialError, RevokeCredentialError},
    translation::CredentialDecodingError,
};
use abstract_app::sdk::AbstractSdkError;
use abstract_app::std::AbstractError;
use abstract_app::AppError;
use cosmwasm_std::StdError;
use cw_controllers::AdminError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum AxoneVcError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Abstract(#[from] AbstractError),

    #[error("{0}")]
    AbstractSdk(#[from] AbstractSdkError),

    #[error("{0}")]
    DappError(#[from] AppError),

    #[error("{0}")]
    Admin(#[from] AdminError),

    #[error(transparent)]
    IssueCredential(#[from] IssueCredentialError),

    #[error(transparent)]
    RevokeCredential(#[from] RevokeCredentialError),

    #[error(transparent)]
    Credential(#[from] CredentialError),

    #[error(transparent)]
    CredentialDecode(#[from] CredentialDecodingError),
}
