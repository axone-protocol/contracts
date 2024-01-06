use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum InvalidCredentialError {
    #[error("Missing identifier")]
    MissingIdentifier,

    #[error("Missing issuer")]
    MissingIssuer,

    #[error("Missing issuance date")]
    MissingIssuanceDate,

    #[error("Missing proof")]
    MissingProof,

    #[error("Missing proof type")]
    MissingProofType,

    #[error("Malformed: {0}")]
    Malformed(String),
}
