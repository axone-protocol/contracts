use crate::parser;
use crate::rdf::{Property, Subject, Value};
use axone_rdf::uri::expand_uri;
use cosmwasm_std::StdError;
use std::collections::HashMap;

impl TryFrom<(parser::Node, &HashMap<String, String>)> for Subject {
    type Error = StdError;

    fn try_from(
        (node, prefixes): (parser::Node, &HashMap<String, String>),
    ) -> Result<Self, Self::Error> {
        match node {
            parser::Node::BlankNode(id) => Ok(Subject::BlankNode(id)),
            parser::Node::NamedNode(parser::IRI::Full(uri)) => Ok(Subject::NamedNode(uri)),
            parser::Node::NamedNode(parser::IRI::Prefixed(curie)) => {
                Ok(Subject::NamedNode(expand_uri(&curie, prefixes)?))
            }
        }
    }
}

impl TryFrom<(parser::IRI, &HashMap<String, String>)> for Property {
    type Error = StdError;

    fn try_from(
        (iri, prefixes): (parser::IRI, &HashMap<String, String>),
    ) -> Result<Self, Self::Error> {
        match iri {
            parser::IRI::Full(uri) => Ok(Property(uri)),
            parser::IRI::Prefixed(curie) => Ok(Property(expand_uri(&curie, prefixes)?)),
        }
    }
}

impl TryFrom<(parser::Node, &HashMap<String, String>)> for Value {
    type Error = StdError;

    fn try_from(
        (node, prefixes): (parser::Node, &HashMap<String, String>),
    ) -> Result<Self, Self::Error> {
        match node {
            parser::Node::NamedNode(parser::IRI::Full(uri)) => Ok(Value::NamedNode(uri)),
            parser::Node::NamedNode(parser::IRI::Prefixed(curie)) => {
                Ok(Value::NamedNode(expand_uri(&curie, prefixes)?))
            }
            parser::Node::BlankNode(id) => Ok(Value::BlankNode(id)),
        }
    }
}

impl TryFrom<(parser::Literal, &HashMap<String, String>)> for Value {
    type Error = StdError;

    fn try_from(
        (literal, prefixes): (parser::Literal, &HashMap<String, String>),
    ) -> Result<Self, Self::Error> {
        match literal {
            parser::Literal::Simple(value) => Ok(Value::LiteralSimple(value)),
            parser::Literal::LanguageTaggedString { value, language } => {
                Ok(Value::LiteralLang(value, language))
            }
            parser::Literal::TypedValue {
                value,
                datatype: parser::IRI::Full(uri),
            } => Ok(Value::LiteralDatatype(value, uri)),
            parser::Literal::TypedValue {
                value,
                datatype: parser::IRI::Prefixed(prefix),
            } => Ok(Value::LiteralDatatype(
                value,
                expand_uri(&prefix, prefixes)?,
            )),
        }
    }
}

#[derive(Default)]
pub struct PrefixMap(HashMap<String, String>);
impl PrefixMap {
    pub fn into_inner(self) -> HashMap<String, String> {
        self.0
    }
}

impl From<Vec<parser::Prefix>> for PrefixMap {
    fn from(as_list: Vec<parser::Prefix>) -> Self {
        PrefixMap(
            as_list
                .into_iter()
                .map(|prefix| (prefix.prefix, prefix.namespace))
                .collect(),
        )
    }
}
