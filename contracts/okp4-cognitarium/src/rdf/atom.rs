use cosmwasm_std::StdError;
use rio_api::model::{Literal, NamedNode, Triple};
use std::fmt;

use crate::msg::{self, Prefix, IRI};

use super::expand_uri;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Subject {
    NamedNode(String),
    BlankNode(String),
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Subject::NamedNode(s) => write!(f, "{s}"),
            Subject::BlankNode(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Property(String);

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Value {
    NamedNode(String),
    BlankNode(String),
    LiteralSimple(String),
    LiteralLang(String, String),
    LiteralDatatype(String, String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::NamedNode(s) => write!(f, "{s}"),
            Value::BlankNode(s) => write!(f, "{s}"),
            Value::LiteralSimple(s) => write!(f, "{s}"),
            Value::LiteralLang(s, l) => write!(f, "{s}@{l}"),
            Value::LiteralDatatype(s, d) => write!(f, "{s}^^{d}"),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Atom {
    pub subject: Subject,
    pub property: Property,
    pub value: Value,
}

impl std::fmt::Display for Atom {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(&format!(
            "<{}> <{}> '{}'",
            self.subject, self.property, self.value
        ))?;
        Ok(())
    }
}

impl<'a> From<&'a Atom> for Triple<'a> {
    fn from(atom: &'a Atom) -> Self {
        Triple {
            subject: match &atom.subject {
                Subject::NamedNode(s) => NamedNode { iri: s.as_str() },
                Subject::BlankNode(s) => NamedNode { iri: s.as_str() },
            }
            .into(),
            predicate: NamedNode {
                iri: atom.property.0.as_str(),
            },
            object: match &atom.value {
                Value::NamedNode(s) => NamedNode { iri: s.as_str() }.into(),
                Value::BlankNode(s) => NamedNode { iri: s.as_str() }.into(),
                Value::LiteralSimple(s) => Literal::Simple { value: s.as_str() }.into(),
                Value::LiteralLang(s, l) => Literal::LanguageTaggedString {
                    value: s.as_str(),
                    language: l.as_str(),
                }
                .into(),
                Value::LiteralDatatype(s, d) => Literal::Typed {
                    value: s.as_str(),
                    datatype: NamedNode { iri: d.as_str() },
                }
                .into(),
            },
        }
    }
}

impl TryFrom<(msg::Value, &[Prefix])> for Subject {
    type Error = StdError;

    fn try_from((value, prefixes): (msg::Value, &[Prefix])) -> Result<Self, Self::Error> {
        match value {
            msg::Value::URI {
                value: IRI::Full(uri),
            } => Ok(Subject::NamedNode(uri)),
            msg::Value::URI {
                value: IRI::Prefixed(curie),
            } => Ok(Subject::NamedNode(expand_uri(&curie, prefixes)?)),
            msg::Value::BlankNode { value: id } => Ok(Subject::BlankNode(id)),
            _ => Err(StdError::generic_err(format!(
                "Unsupported subject value: {value:?}"
            ))),
        }
    }
}

impl TryFrom<(msg::Value, &[Prefix])> for Property {
    type Error = StdError;

    fn try_from((value, prefixes): (msg::Value, &[Prefix])) -> Result<Self, Self::Error> {
        match value {
            msg::Value::URI {
                value: IRI::Full(uri),
            } => Ok(Property(uri)),
            msg::Value::URI {
                value: IRI::Prefixed(curie),
            } => Ok(Property(expand_uri(&curie, prefixes)?)),
            _ => Err(StdError::generic_err(format!(
                "Unsupported predicate value: {value:?}"
            ))),
        }
    }
}

impl TryFrom<(msg::Value, &[Prefix])> for Value {
    type Error = StdError;

    fn try_from((value, prefixes): (msg::Value, &[Prefix])) -> Result<Self, Self::Error> {
        match value {
            msg::Value::URI {
                value: IRI::Full(uri),
            } => Ok(Value::NamedNode(uri)),
            msg::Value::URI {
                value: IRI::Prefixed(curie),
            } => Ok(Value::NamedNode(expand_uri(&curie, prefixes)?)),
            msg::Value::Literal {
                value,
                lang: None,
                datatype: None,
            } => Ok(Value::LiteralSimple(value)),
            msg::Value::Literal {
                value,
                lang: Some(lang),
                datatype: None,
            } => Ok(Value::LiteralLang(value, lang)),
            msg::Value::Literal {
                value,
                lang: None,
                datatype: Some(IRI::Full(uri)),
            } => Ok(Value::LiteralDatatype(value, uri)),
            msg::Value::Literal {
                value,
                lang: None,
                datatype: Some(IRI::Prefixed(curie)),
            } => Ok(Value::LiteralDatatype(value, expand_uri(&curie, prefixes)?)),
            msg::Value::BlankNode { value } => Ok(Value::BlankNode(value)),
            _ => Err(StdError::generic_err(format!(
                "Unsupported object value: {value:?}"
            )))?,
        }
    }
}
