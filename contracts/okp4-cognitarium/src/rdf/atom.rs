use cosmwasm_std::StdError;
use rio_api::model::{Literal, NamedNode, Triple};
use std::collections::BTreeMap;
use std::fmt;

use crate::msg;
use crate::msg::TriplePattern;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Subject {
    NamedNode(String),
    BlankNode(String),
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Subject::NamedNode(s) | Subject::BlankNode(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Property(pub String);

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
            Value::NamedNode(s) | Value::BlankNode(s) | Value::LiteralSimple(s) => write!(f, "{s}"),
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

impl fmt::Display for Atom {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
                Subject::NamedNode(s) | Subject::BlankNode(s) => NamedNode { iri: s.as_str() },
            }
            .into(),
            predicate: NamedNode {
                iri: atom.property.0.as_str(),
            },
            object: match &atom.value {
                Value::NamedNode(s) | Value::BlankNode(s) => NamedNode { iri: s.as_str() }.into(),
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

impl TriplePattern {
    pub fn resolve(
        &self,
        bindings: &BTreeMap<String, msg::Value>,
        prefixes: &[msg::Prefix],
    ) -> Result<Atom, StdError> {
        let subject = match &self.subject {
            msg::VarOrNode::Variable(var) => {
                let value = bindings.get(var).ok_or_else(|| {
                    StdError::generic_err(format!("Unbound subject variable: {:?}", var))
                })?;
                (value.clone(), prefixes).try_into()?
            }
            msg::VarOrNode::Node(node) => (node.clone(), prefixes).try_into()?,
        };

        let property = match &self.predicate {
            msg::VarOrNode::Variable(var) => {
                let value = bindings.get(var).ok_or_else(|| {
                    StdError::generic_err(format!("Unbound predicate variable: {:?}", var))
                })?;
                (value.clone(), prefixes).try_into()?
            }
            msg::VarOrNode::Node(node) => (node.clone(), prefixes).try_into()?,
        };

        let value = match &self.object {
            msg::VarOrNodeOrLiteral::Variable(var) => {
                let value = bindings.get(var).ok_or_else(|| {
                    StdError::generic_err(format!("Unbound object variable: {:?}", var))
                })?;
                (value.clone(), prefixes).try_into()?
            }
            msg::VarOrNodeOrLiteral::Node(node) => (node.clone(), prefixes).try_into()?,
            msg::VarOrNodeOrLiteral::Literal(literal) => (literal.clone(), prefixes).try_into()?,
        };

        Ok(Atom {
            subject,
            property,
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proper_display() {
        // # Subject
        assert_eq!(
            format!("{}", Subject::BlankNode("blank".to_string())),
            "blank".to_string()
        );
        assert_eq!(
            format!("{}", Subject::NamedNode("named".to_string())),
            "named".to_string()
        );

        // # Property
        assert_eq!(
            format!("{}", Property("foo".to_string())),
            "foo".to_string()
        );

        // # Value
        assert_eq!(
            format!("{}", Value::NamedNode("named".to_string())),
            "named".to_string()
        );
        assert_eq!(
            format!("{}", Value::BlankNode("blank".to_string())),
            "blank".to_string()
        );
        assert_eq!(
            format!("{}", Value::LiteralSimple("simple".to_string())),
            "simple".to_string()
        );
        assert_eq!(
            format!(
                "{}",
                Value::LiteralLang("lang".to_string(), "en".to_string())
            ),
            "lang@en".to_string()
        );
        assert_eq!(
            format!(
                "{}",
                Value::LiteralDatatype("data".to_string(), "uri".to_string())
            ),
            "data^^uri".to_string()
        );

        // # Atom
        assert_eq!(
            format!(
                "{}",
                Atom {
                    subject: Subject::NamedNode("subject".to_string()),
                    property: Property("predicate".to_string()),
                    value: Value::LiteralLang("object".to_string(), "en".to_string()),
                }
            ),
            "<subject> <predicate> 'object@en'".to_string()
        );
    }
}
