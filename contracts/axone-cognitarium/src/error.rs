use cosmwasm_std::{StdError, Uint128};
use cw_utils::PaymentError;
use rio_turtle::TurtleError;
use rio_xml::RdfXmlError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    ParseRDF(#[from] RDFParseError),

    #[error("{0}")]
    FormatRDF(String),

    #[error("{0}")]
    Store(#[from] StoreError),

    #[error("Only the owner can perform this operation.")]
    Unauthorized,

    #[error("{0}")]
    Payment(#[from] PaymentError),
}

impl From<RdfXmlError> for ContractError {
    fn from(value: RdfXmlError) -> Self {
        RDFParseError::from(value).into()
    }
}

impl From<TurtleError> for ContractError {
    fn from(value: TurtleError) -> Self {
        RDFParseError::from(value).into()
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum StoreError {
    #[error("Maximum triples number exceeded: {0}")]
    TripleCount(Uint128),

    #[error("Maximum byte size exceeded: {0}")]
    ByteSize(Uint128),

    #[error("Maximum triple byte size exceeded: {0} / {1}")]
    TripleByteSize(Uint128, Uint128),

    #[error("Maximum insert byte size exceeded: {0}")]
    InsertDataByteSize(Uint128),

    #[error("Maximum insert triple count exceeded: {0}")]
    InsertDataTripleCount(Uint128),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RDFParseError {
    #[error("Error parsing XML RDF: {0}")]
    Xml(String),

    #[error("Error parsing Turtle RDF: {0}")]
    Turtle(String),
}

impl From<RdfXmlError> for RDFParseError {
    fn from(value: RdfXmlError) -> Self {
        RDFParseError::Xml(value.to_string())
    }
}

impl From<TurtleError> for RDFParseError {
    fn from(value: TurtleError) -> Self {
        RDFParseError::Xml(value.to_string())
    }
}
