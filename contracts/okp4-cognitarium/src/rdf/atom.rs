use rio_api::model::{Literal, NamedNode, Triple};
use std::fmt;

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
    pub property: String,
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
                iri: atom.property.as_str(),
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
