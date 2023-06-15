#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Value {
    NamedNode(String),
    BlankNode(String),
    LiteralSimple(String),
    LiteralLang(String, String),
    LiteralDatatype(String, String),
}

use std::fmt;

use rio_api::model::{Literal, NamedNode, Triple};
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::NamedNode(s) => write!(f, "{}", s),
            Value::BlankNode(s) => write!(f, "{}", s),
            Value::LiteralSimple(s) => write!(f, "{}", s),
            Value::LiteralLang(s, l) => write!(f, "{}@{}", s, l),
            Value::LiteralDatatype(s, d) => write!(f, "{}^^{}", s, d),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Atom {
    pub subject: String,
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
            subject: NamedNode::from(NamedNode {
                iri: atom.subject.as_str(),
            })
            .into(),
            predicate: NamedNode::from(NamedNode {
                iri: atom.property.as_str(),
            }),
            object: match &atom.value {
                Value::NamedNode(s) => NamedNode::from(NamedNode { iri: s.as_str() }).into(),
                Value::BlankNode(s) => NamedNode::from(NamedNode { iri: s.as_str() }).into(),
                Value::LiteralSimple(s) => {
                    Literal::from(Literal::Simple { value: s.as_str() }).into()
                }
                Value::LiteralLang(s, l) => Literal::from(Literal::LanguageTaggedString {
                    value: s.as_str(),
                    language: l.as_str(),
                })
                .into(),
                Value::LiteralDatatype(s, d) => Literal::from(Literal::Typed {
                    value: s.as_str(),
                    datatype: NamedNode::from(NamedNode { iri: d.as_str() }),
                })
                .into(),
            },
        }
    }
}
