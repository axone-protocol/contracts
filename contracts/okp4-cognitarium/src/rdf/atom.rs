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
    use crate::msg::IRI;

    #[test]
    fn proper_display() {
        struct TC<'a> {
            input: Box<dyn fmt::Display + 'a>,
            expected: String,
        }
        let cases = vec![
            // # Subject
            TC {
                input: Box::new(Subject::BlankNode("blank".into())),
                expected: "blank".into(),
            },
            TC {
                input: Box::new(Subject::NamedNode("named".into())),
                expected: "named".into(),
            },
            // # Property
            TC {
                input: Box::new(Property("foo".into())),
                expected: "foo".into(),
            },
            // #  Value
            TC {
                input: Box::new(Value::NamedNode("named".into())),
                expected: "named".into(),
            },
            TC {
                input: Box::new(Value::BlankNode("blank".into())),
                expected: "blank".into(),
            },
            TC {
                input: Box::new(Value::LiteralSimple("simple".into())),
                expected: "simple".into(),
            },
            TC {
                input: Box::new(Value::LiteralLang("lang".into(), "en".into())),
                expected: "lang@en".into(),
            },
            TC {
                input: Box::new(Value::LiteralDatatype("data".into(), "uri".into())),
                expected: "data^^uri".into(),
            },
            // # Atom
            TC {
                input: Box::new(Atom {
                    subject: Subject::NamedNode("subject".into()),
                    property: Property("predicate".into()),
                    value: Value::LiteralLang("object".into(), "en".into()),
                }),
                expected: "<subject> <predicate> 'object@en'".into(),
            },
        ];
        for tc in cases {
            assert_eq!(format!("{}", tc.input), tc.expected);
        }
    }

    #[test]
    fn triple_pattern_resolve() {
        struct TC<'a> {
            triple_pattern: TriplePattern,
            bindings: &'a BTreeMap<String, msg::Value>,
            prefixes: &'a Vec<msg::Prefix>,
            expected: Result<Atom, StdError>,
        }
        let bindings: BTreeMap<String, msg::Value> = BTreeMap::from([
            (
                "s1".to_string(),
                msg::Value::URI {
                    value: msg::IRI::Full(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .to_string(),
                    ),
                },
            ),
            (
                "s2".to_string(),
                msg::Value::URI {
                    value: msg::IRI::Prefixed(
                        "core:dcf48417-01c5-4b43-9bc7-49e54c028473".to_string(),
                    ),
                },
            ),
            (
                "s3".to_string(),
                msg::Value::BlankNode {
                    value: "_1".to_string(),
                },
            ),
            (
                "p1".to_string(),
                msg::Value::URI {
                    value: msg::IRI::Full(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .to_string(),
                    ),
                },
            ),
            (
                "p2".to_string(),
                msg::Value::URI {
                    value: msg::IRI::Prefixed(
                        "core:dcf48417-01c5-4b43-9bc7-49e54c028473".to_string(),
                    ),
                },
            ),
            (
                "o1".to_string(),
                msg::Value::URI {
                    value: msg::IRI::Full(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .to_string(),
                    ),
                },
            ),
            (
                "o2".to_string(),
                msg::Value::URI {
                    value: msg::IRI::Prefixed(
                        "core:dcf48417-01c5-4b43-9bc7-49e54c028473".to_string(),
                    ),
                },
            ),
            (
                "o3".to_string(),
                msg::Value::Literal {
                    value: "foo".into(),
                    datatype: None,
                    lang: None,
                },
            ),
            (
                "o4".to_string(),
                msg::Value::Literal {
                    value: "foo".into(),
                    datatype: Some(msg::IRI::Prefixed("owl:foo".into())),
                    lang: None,
                },
            ),
            (
                "o5".to_string(),
                msg::Value::Literal {
                    value: "foo".into(),
                    datatype: None,
                    lang: Some("en".into()),
                },
            ),
        ]);
        let prefixes = vec![
            msg::Prefix {
                prefix: "core".to_string(),
                namespace: "https://ontology.okp4.space/core/".to_string(),
            },
            msg::Prefix {
                prefix: "owl".to_string(),
                namespace: "http://www.w3.org/2002/07/owl#".to_string(),
            },
        ];
        let cases = vec![
            // Bindings
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Variable("s1".into()),
                    predicate: msg::VarOrNode::Variable("p1".into()),
                    object: msg::VarOrNodeOrLiteral::Variable("o1".into()),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Ok(Atom {
                    subject: Subject::NamedNode(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                    property: Property(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                    value: Value::NamedNode(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                }),
            },
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Variable("s2".into()),
                    predicate: msg::VarOrNode::Variable("p2".into()),
                    object: msg::VarOrNodeOrLiteral::Variable("o2".into()),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Ok(Atom {
                    subject: Subject::NamedNode(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                    property: Property(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                    value: Value::NamedNode(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                }),
            },
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Variable("s3".into()),
                    predicate: msg::VarOrNode::Variable("p1".into()),
                    object: msg::VarOrNodeOrLiteral::Variable("o3".into()),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Ok(Atom {
                    subject: Subject::BlankNode("_1".into()),
                    property: Property(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                    value: Value::LiteralSimple("foo".into()),
                }),
            },
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Variable("s1".into()),
                    predicate: msg::VarOrNode::Variable("p1".into()),
                    object: msg::VarOrNodeOrLiteral::Variable("o4".into()),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Ok(Atom {
                    subject: Subject::NamedNode(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                    property: Property(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                    value: Value::LiteralDatatype(
                        "foo".into(),
                        "http://www.w3.org/2002/07/owl#foo".into(),
                    ),
                }),
            },
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Variable("s1".into()),
                    predicate: msg::VarOrNode::Variable("p1".into()),
                    object: msg::VarOrNodeOrLiteral::Variable("o5".into()),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Ok(Atom {
                    subject: Subject::NamedNode(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                    property: Property(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                    value: Value::LiteralLang("foo".into(), "en".into()),
                }),
            },
            // Plain values
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Node(msg::Node::NamedNode(msg::IRI::Prefixed(
                        "core:dcf48417-01c5-4b43-9bc7-49e54c028473".into(),
                    ))),
                    predicate: msg::VarOrNode::Node(msg::Node::NamedNode(msg::IRI::Prefixed(
                        "owl:foo".into(),
                    ))),
                    object: msg::VarOrNodeOrLiteral::Node(msg::Node::NamedNode(
                        msg::IRI::Prefixed("core:dcf48417-01c5-4b43-9bc7-49e54c028473".into()),
                    )),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Ok(Atom {
                    subject: Subject::NamedNode(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                    property: Property("http://www.w3.org/2002/07/owl#foo".into()),
                    value: Value::NamedNode(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                }),
            },
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Node(msg::Node::NamedNode(msg::IRI::Full(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ))),
                    predicate: msg::VarOrNode::Node(msg::Node::NamedNode(msg::IRI::Full(
                        "http://www.w3.org/2002/07/owl#foo".into(),
                    ))),
                    object: msg::VarOrNodeOrLiteral::Node(msg::Node::NamedNode(msg::IRI::Full(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ))),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Ok(Atom {
                    subject: Subject::NamedNode(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                    property: Property("http://www.w3.org/2002/07/owl#foo".into()),
                    value: Value::NamedNode(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                }),
            },
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Node(msg::Node::BlankNode("_1".into())),
                    predicate: msg::VarOrNode::Node(msg::Node::NamedNode(msg::IRI::Full(
                        "http://www.w3.org/2002/07/owl#foo".into(),
                    ))),
                    object: msg::VarOrNodeOrLiteral::Node(msg::Node::BlankNode("_2".into())),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Ok(Atom {
                    subject: Subject::BlankNode("_1".into()),
                    property: Property("http://www.w3.org/2002/07/owl#foo".into()),
                    value: Value::BlankNode("_2".into()),
                }),
            },
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Node(msg::Node::NamedNode(msg::IRI::Prefixed(
                        "core:dcf48417-01c5-4b43-9bc7-49e54c028473".into(),
                    ))),
                    predicate: msg::VarOrNode::Node(msg::Node::NamedNode(msg::IRI::Prefixed(
                        "owl:foo".into(),
                    ))),
                    object: msg::VarOrNodeOrLiteral::Literal(msg::Literal::Simple("foo".into())),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Ok(Atom {
                    subject: Subject::NamedNode(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                    property: Property("http://www.w3.org/2002/07/owl#foo".into()),
                    value: Value::LiteralSimple("foo".into()),
                }),
            },
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Node(msg::Node::NamedNode(msg::IRI::Prefixed(
                        "core:dcf48417-01c5-4b43-9bc7-49e54c028473".into(),
                    ))),
                    predicate: msg::VarOrNode::Node(msg::Node::NamedNode(msg::IRI::Prefixed(
                        "owl:foo".into(),
                    ))),
                    object: msg::VarOrNodeOrLiteral::Literal(msg::Literal::LanguageTaggedString {
                        value: "foo".into(),
                        language: "en".into(),
                    }),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Ok(Atom {
                    subject: Subject::NamedNode(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                    property: Property("http://www.w3.org/2002/07/owl#foo".into()),
                    value: Value::LiteralLang("foo".into(), "en".into()),
                }),
            },
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Node(msg::Node::NamedNode(msg::IRI::Prefixed(
                        "core:dcf48417-01c5-4b43-9bc7-49e54c028473".into(),
                    ))),
                    predicate: msg::VarOrNode::Node(msg::Node::NamedNode(msg::IRI::Prefixed(
                        "owl:foo".into(),
                    ))),
                    object: msg::VarOrNodeOrLiteral::Literal(msg::Literal::TypedValue {
                        value: "foo".into(),
                        datatype: IRI::Prefixed("owl:type".into()),
                    }),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Ok(Atom {
                    subject: Subject::NamedNode(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                    property: Property("http://www.w3.org/2002/07/owl#foo".into()),
                    value: Value::LiteralDatatype(
                        "foo".into(),
                        "http://www.w3.org/2002/07/owl#type".into(),
                    ),
                }),
            },
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Node(msg::Node::NamedNode(msg::IRI::Prefixed(
                        "core:dcf48417-01c5-4b43-9bc7-49e54c028473".into(),
                    ))),
                    predicate: msg::VarOrNode::Node(msg::Node::NamedNode(msg::IRI::Prefixed(
                        "owl:foo".into(),
                    ))),
                    object: msg::VarOrNodeOrLiteral::Literal(msg::Literal::TypedValue {
                        value: "foo".into(),
                        datatype: IRI::Full("http://www.w3.org/2002/07/owl#type".into()),
                    }),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Ok(Atom {
                    subject: Subject::NamedNode(
                        "https://ontology.okp4.space/core/dcf48417-01c5-4b43-9bc7-49e54c028473"
                            .into(),
                    ),
                    property: Property("http://www.w3.org/2002/07/owl#foo".into()),
                    value: Value::LiteralDatatype(
                        "foo".into(),
                        "http://www.w3.org/2002/07/owl#type".into(),
                    ),
                }),
            },
            // Error
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Variable("unknown".into()),
                    predicate: msg::VarOrNode::Variable("p1".into()),
                    object: msg::VarOrNodeOrLiteral::Variable("o1".into()),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Err(StdError::generic_err(
                    "Unbound subject variable: \"unknown\"",
                )),
            },
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Variable("s1".into()),
                    predicate: msg::VarOrNode::Variable("unknown".into()),
                    object: msg::VarOrNodeOrLiteral::Variable("o1".into()),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Err(StdError::generic_err(
                    "Unbound predicate variable: \"unknown\"",
                )),
            },
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Variable("s1".into()),
                    predicate: msg::VarOrNode::Variable("p1".into()),
                    object: msg::VarOrNodeOrLiteral::Variable("unknown".into()),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Err(StdError::generic_err(
                    "Unbound object variable: \"unknown\"",
                )),
            },
            TC {
                triple_pattern: TriplePattern {
                    subject: msg::VarOrNode::Node(msg::Node::BlankNode("_1".into())),
                    predicate: msg::VarOrNode::Node(msg::Node::BlankNode("_2".into())),
                    object: msg::VarOrNodeOrLiteral::Node(msg::Node::NamedNode(
                        msg::IRI::Prefixed("core:dcf48417-01c5-4b43-9bc7-49e54c028473".into()),
                    )),
                },
                bindings: &bindings,
                prefixes: &prefixes,
                expected: Err(StdError::generic_err(
                    "Unsupported predicate node: BlankNode(\"_2\"). Expected URI",
                )),
            },
        ];

        for tc in cases {
            assert_eq!(
                tc.triple_pattern.resolve(tc.bindings, tc.prefixes),
                tc.expected
            );
        }
    }
}
