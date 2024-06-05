use crate::credential::error::{InvalidCredentialError, VerificationError};
use axone_rdf::serde::NQuadsReadError;
use cosmwasm_std::{Instantiate2AddressError, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Instantiate2Address(#[from] Instantiate2AddressError),

    #[error("Couldn't parse RDF: '{0}'")]
    ParseRDF(#[from] NQuadsReadError),

    #[error("Invalid credential: '{0}'")]
    InvalidCredential(#[from] InvalidCredentialError),

    #[error("Credential verification failed: '{0}'")]
    CredentialVerification(#[from] VerificationError),

    #[error("Credential not supported: '{0}'")]
    UnsupportedCredential(String),

    #[error("Credential already exists: '{0}'")]
    CredentialAlreadyExists(String),

    #[error("An unexpected error occurred: {0}")]
    Unexpected(String),

    #[error("{0}")]
    Payment(#[from] PaymentError),
}
