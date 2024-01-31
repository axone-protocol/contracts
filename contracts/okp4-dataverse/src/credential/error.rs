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

    #[error("Invalid proof: {0}")]
    InvalidProof(#[from] InvalidProofError),

    #[error("Malformed: {0}")]
    Malformed(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum InvalidProofError {
    #[error("Missing proof type")]
    MissingProofType,

    #[error("Missing verification method")]
    MissingVerificationMethod,

    #[error("Missing created")]
    MissingCreated,

    #[error("Missing proof purpose")]
    MissingProofPurpose,

    #[error("Missing proof value")]
    MissingProofValue,

    #[error("Could not decode public key")]
    InvalidPubKey,

    #[error("Malformed: {0}")]
    Malformed(String),
}
