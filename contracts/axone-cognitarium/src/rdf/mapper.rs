use crate::msg;
use crate::rdf::{Property, Subject, Value};
use axone_rdf::uri::expand_uri;
use cosmwasm_std::StdError;
use std::collections::HashMap;

impl TryFrom<(msg::Node, &HashMap<String, String>)> for Subject {
    type Error = StdError;

    fn try_from(
        (node, prefixes): (msg::Node, &HashMap<String, String>),
    ) -> Result<Self, Self::Error> {
        match node {
            msg::Node::BlankNode(id) => Ok(Subject::BlankNode(id)),
            msg::Node::NamedNode(msg::IRI::Full(uri)) => Ok(Subject::NamedNode(uri)),
            msg::Node::NamedNode(msg::IRI::Prefixed(curie)) => {
                Ok(Subject::NamedNode(expand_uri(&curie, prefixes)?))
            }
        }
    }
}

impl TryFrom<(msg::IRI, &HashMap<String, String>)> for Property {
    type Error = StdError;

    fn try_from(
        (iri, prefixes): (msg::IRI, &HashMap<String, String>),
    ) -> Result<Self, Self::Error> {
        match iri {
            msg::IRI::Full(uri) => Ok(Property(uri)),
            msg::IRI::Prefixed(curie) => Ok(Property(expand_uri(&curie, prefixes)?)),
        }
    }
}

impl TryFrom<(msg::Node, &HashMap<String, String>)> for Value {
    type Error = StdError;

    fn try_from(
        (node, prefixes): (msg::Node, &HashMap<String, String>),
    ) -> Result<Self, Self::Error> {
        match node {
            msg::Node::NamedNode(msg::IRI::Full(uri)) => Ok(Value::NamedNode(uri)),
            msg::Node::NamedNode(msg::IRI::Prefixed(curie)) => {
                Ok(Value::NamedNode(expand_uri(&curie, prefixes)?))
            }
            msg::Node::BlankNode(id) => Ok(Value::BlankNode(id)),
        }
    }
}

impl TryFrom<(msg::Literal, &HashMap<String, String>)> for Value {
    type Error = StdError;

    fn try_from(
        (literal, prefixes): (msg::Literal, &HashMap<String, String>),
    ) -> Result<Self, Self::Error> {
        match literal {
            msg::Literal::Simple(value) => Ok(Value::LiteralSimple(value)),
            msg::Literal::LanguageTaggedString { value, language } => {
                Ok(Value::LiteralLang(value, language))
            }
            msg::Literal::TypedValue {
                value,
                datatype: msg::IRI::Full(uri),
            } => Ok(Value::LiteralDatatype(value, uri)),
            msg::Literal::TypedValue {
                value,
                datatype: msg::IRI::Prefixed(prefix),
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

impl From<Vec<msg::Prefix>> for PrefixMap {
    fn from(as_list: Vec<msg::Prefix>) -> Self {
        PrefixMap(
            as_list
                .into_iter()
                .map(|prefix| (prefix.prefix, prefix.namespace))
                .collect(),
        )
    }
}
