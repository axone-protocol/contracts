use crate::credential::error::{InvalidCredentialError, VerificationError};
use cosmwasm_std::{Instantiate2AddressError, StdError};
use okp4_rdf::serde::NQuadsReadError;
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

    #[error("Credential verification failed: {0}")]
    CredentialVerification(#[from] VerificationError),
}
