use thiserror::Error;
use url::ParseError;

#[derive(Debug, Eq, Error, PartialEq)]
pub enum CosmwasmUriError {
    #[error("{0}")]
    ParseURI(#[from] ParseError),

    #[error("{0}")]
    ParseQuery(String),

    #[error("{0}")]
    SerializeQuery(String),

    #[error("Malformed URI: {0}")]
    Malformed(String),
}
