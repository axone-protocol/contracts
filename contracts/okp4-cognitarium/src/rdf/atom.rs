use cosmwasm_std::StdError;
use rio_api::model::{Literal, NamedNode, Triple};
use std::collections::BTreeMap;
use std::fmt;

use crate::msg;
use crate::msg::TriplePattern;

use super::expand_uri;

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

impl std::fmt::Display for Atom {
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

impl TryFrom<(msg::Value, &[msg::Prefix])> for Subject {
    type Error = StdError;

    fn try_from((value, prefixes): (msg::Value, &[msg::Prefix])) -> Result<Self, Self::Error> {
        match value {
            msg::Value::URI {
                value: msg::IRI::Full(uri),
            } => Ok(Subject::NamedNode(uri)),
            msg::Value::URI {
                value: msg::IRI::Prefixed(curie),
            } => Ok(Subject::NamedNode(expand_uri(&curie, prefixes)?)),
            msg::Value::BlankNode { value: id } => Ok(Subject::BlankNode(id)),
            _ => Err(StdError::generic_err(format!(
                "Unsupported subject value: {value:?}"
            ))),
        }
    }
}

impl TryFrom<(msg::Value, &[msg::Prefix])> for Property {
    type Error = StdError;

    fn try_from((value, prefixes): (msg::Value, &[msg::Prefix])) -> Result<Self, Self::Error> {
        match value {
            msg::Value::URI {
                value: msg::IRI::Full(uri),
            } => Ok(Property(uri)),
            msg::Value::URI {
                value: msg::IRI::Prefixed(curie),
            } => Ok(Property(expand_uri(&curie, prefixes)?)),
            _ => Err(StdError::generic_err(format!(
                "Unsupported predicate value: {value:?}"
            ))),
        }
    }
}

impl TryFrom<(msg::Value, &[msg::Prefix])> for Value {
    type Error = StdError;

    fn try_from((value, prefixes): (msg::Value, &[msg::Prefix])) -> Result<Self, Self::Error> {
        match value {
            msg::Value::URI {
                value: msg::IRI::Full(uri),
            } => Ok(Value::NamedNode(uri)),
            msg::Value::URI {
                value: msg::IRI::Prefixed(curie),
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
                datatype: Some(msg::IRI::Full(uri)),
            } => Ok(Value::LiteralDatatype(value, uri)),
            msg::Value::Literal {
                value,
                lang: None,
                datatype: Some(msg::IRI::Prefixed(curie)),
            } => Ok(Value::LiteralDatatype(value, expand_uri(&curie, prefixes)?)),
            msg::Value::BlankNode { value } => Ok(Value::BlankNode(value)),
            _ => Err(StdError::generic_err(format!(
                "Unsupported object value: {value:?}"
            )))?,
        }
    }
}

impl TryFrom<(msg::Node, &[msg::Prefix])> for Subject {
    type Error = StdError;

    fn try_from((node, prefixes): (msg::Node, &[msg::Prefix])) -> Result<Self, Self::Error> {
        match node {
            msg::Node::BlankNode(id) => Ok(Subject::BlankNode(id)),
            msg::Node::NamedNode(msg::IRI::Full(uri)) => Ok(Subject::NamedNode(uri)),
            msg::Node::NamedNode(msg::IRI::Prefixed(curie)) => {
                Ok(Subject::NamedNode(expand_uri(&curie, prefixes)?))
            }
        }
    }
}

impl TryFrom<(msg::Node, &[msg::Prefix])> for Property {
    type Error = StdError;

    fn try_from((node, prefixes): (msg::Node, &[msg::Prefix])) -> Result<Self, Self::Error> {
        match node {
            msg::Node::NamedNode(msg::IRI::Full(uri)) => Ok(Property(uri)),
            msg::Node::NamedNode(msg::IRI::Prefixed(curie)) => {
                Ok(Property(expand_uri(&curie, prefixes)?))
            }
            _ => Err(StdError::generic_err(format!(
                "Unsupported predicate node: {node:?}"
            ))),
        }
    }
}

impl TryFrom<(msg::Node, &[msg::Prefix])> for Value {
    type Error = StdError;

    fn try_from((node, prefixes): (msg::Node, &[msg::Prefix])) -> Result<Self, Self::Error> {
        match node {
            msg::Node::NamedNode(msg::IRI::Full(uri)) => Ok(Value::NamedNode(uri)),
            msg::Node::NamedNode(msg::IRI::Prefixed(curie)) => {
                Ok(Value::NamedNode(expand_uri(&curie, prefixes)?))
            }
            msg::Node::BlankNode(id) => Ok(Value::BlankNode(id)),
        }
    }
}

impl TryFrom<(msg::Literal, &[msg::Prefix])> for Value {
    type Error = StdError;

    fn try_from((literal, prefixes): (msg::Literal, &[msg::Prefix])) -> Result<Self, Self::Error> {
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

    #[test]
    fn try_from_subject() {
        assert_eq!(
            (
                msg::Value::URI {
                    value: msg::IRI::Full(
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
                    ),
                },
                vec![].as_slice(),
            )
                .try_into(),
            Ok(Subject::NamedNode(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
            ))
        );
        assert_eq!(
            (
                msg::Value::BlankNode {
                    value: "blank".to_string(),
                },
                vec![].as_slice(),
            )
                .try_into(),
            Ok(Subject::BlankNode("blank".to_string()))
        );
        assert_eq!(
            (
                msg::Value::URI {
                    value: msg::IRI::Prefixed("rdf:".to_string()),
                },
                vec![msg::Prefix {
                    prefix: "rdf".to_string(),
                    namespace: "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
                }]
                .as_slice(),
            )
                .try_into(),
            Ok(Subject::NamedNode(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
            ))
        );
        assert_eq!(
            Subject::try_from((
                msg::Value::Literal {
                    value: "rdf".to_string(),
                    lang: None,
                    datatype: None,
                },
                vec![].as_slice(),
            )),
            Err(StdError::generic_err(
                "Unsupported subject value: Literal { value: \"rdf\", lang: None, datatype: None }"
            ))
        );
    }

    #[test]
    fn try_from_property() {
        assert_eq!(
            (
                msg::Value::URI {
                    value: msg::IRI::Full(
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
                    ),
                },
                vec![].as_slice(),
            )
                .try_into(),
            Ok(Property(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
            ))
        );
        assert_eq!(
            (
                msg::Value::URI {
                    value: msg::IRI::Prefixed("rdf:".to_string()),
                },
                vec![msg::Prefix {
                    prefix: "rdf".to_string(),
                    namespace: "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
                }]
                .as_slice(),
            )
                .try_into(),
            Ok(Property(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
            ))
        );
        assert_eq!(
            Property::try_from((
                msg::Value::BlankNode {
                    value: "blank".to_string(),
                },
                vec![].as_slice(),
            )),
            Err(StdError::generic_err(
                "Unsupported predicate value: BlankNode { value: \"blank\" }"
            ))
        );
    }

    #[test]
    fn try_from_value() {
        assert_eq!(
            (
                msg::Value::URI {
                    value: msg::IRI::Full(
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
                    ),
                },
                vec![].as_slice(),
            )
                .try_into(),
            Ok(Value::NamedNode(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
            ))
        );
        assert_eq!(
            (
                msg::Value::URI {
                    value: msg::IRI::Prefixed("rdf:".to_string()),
                },
                vec![msg::Prefix {
                    prefix: "rdf".to_string(),
                    namespace: "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
                }]
                .as_slice(),
            )
                .try_into(),
            Ok(Value::NamedNode(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
            ))
        );
        assert_eq!(
            (
                msg::Value::Literal {
                    value: "foo".to_string(),
                    lang: None,
                    datatype: None,
                },
                vec![].as_slice(),
            )
                .try_into(),
            Ok(Value::LiteralSimple("foo".to_string()))
        );
        assert_eq!(
            (
                msg::Value::Literal {
                    value: "foo".to_string(),
                    lang: Some("en".to_string()),
                    datatype: None,
                },
                vec![].as_slice(),
            )
                .try_into(),
            Ok(Value::LiteralLang("foo".to_string(), "en".to_string()))
        );
        assert_eq!(
            (
                msg::Value::Literal {
                    value: "foo".to_string(),
                    lang: None,
                    datatype: Some(msg::IRI::Full(
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
                    )),
                },
                vec![].as_slice(),
            )
                .try_into(),
            Ok(Value::LiteralDatatype(
                "foo".to_string(),
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
            ))
        );
        assert_eq!(
            (
                msg::Value::Literal {
                    value: "foo".to_string(),
                    lang: None,
                    datatype: Some(msg::IRI::Prefixed("rdf:".to_string())),
                },
                vec![msg::Prefix {
                    prefix: "rdf".to_string(),
                    namespace: "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
                }]
                .as_slice(),
            )
                .try_into(),
            Ok(Value::LiteralDatatype(
                "foo".to_string(),
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
            ))
        );
        assert_eq!(
            (
                msg::Value::BlankNode {
                    value: "foo".to_string()
                },
                vec![].as_slice(),
            )
                .try_into(),
            Ok(Value::BlankNode("foo".to_string()))
        );
        assert_eq!(
            Value::try_from((
                msg::Value::Literal {
                    value: "blank".to_string(),
                    lang: Some("en".to_string()),
                    datatype: Some(msg::IRI::Full(
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
                    )),
                },
                vec![].as_slice(),
            )),
            Err(StdError::generic_err(
                "Unsupported object value: Literal { value: \"blank\", lang: Some(\"en\"), datatype: Some(Full(\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\")) }"
            ))
        );
    }
}
