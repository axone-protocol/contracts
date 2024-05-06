use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum TermParseError {
    #[error("Value is not UTF-8 encoded: {0}")]
    NotUtf8Value(FromUtf8Error),

    #[error("Reach unexpected EOF")]
    Eof,

    #[error("Expected ',' or end of sequence and got: '{0}'")]
    ExpectedSeqToken(char),

    #[error("Unexpected end of array or tuple")]
    UnexpectedEndOfSeq,

    #[error("Forbidden token in value: '{0}'")]
    UnexpectedValueToken(char),

    #[error("Unexpected root token: '{0}'")]
    UnexpectedRootToken(char),

    #[error("Empty value in array or tuple")]
    EmptyValue,

    #[error("Empty tuple")]
    EmptyTuple,
}
