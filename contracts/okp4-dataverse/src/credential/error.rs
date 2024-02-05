use okp4_rdf::normalize::NormalizationError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum InvalidCredentialError {
    #[error("Missing identifier")]
    MissingIdentifier,

    #[error("Missing issuer")]
    MissingIssuer,

    #[error("Missing issuance date")]
    MissingIssuanceDate,

    #[error("Missing proof, at least a supported one")]
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

    #[error("Malformed proof value: {0}")]
    MalformedProofValue(#[from] multibase::Error),

    #[error("Could not decode public key")]
    InvalidPubKey,

    #[error("Malformed: {0}")]
    Malformed(String),

    // Used internally only
    #[error("Unsupported proof type")]
    Unsupported,
}

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("Couldn't canonicalize document: {0}")]
    RdfCanonError(#[from] NormalizationError),

    #[error("Couldn't verify signature: {0}")]
    SignatureError(#[from] ed25519_compact::Error),

    #[error("Couldn't find a suitable proof")]
    NoSuitableProof,
}
