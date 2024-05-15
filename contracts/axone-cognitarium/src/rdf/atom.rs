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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}> <{}> '{}'", self.subject, self.property, self.value)
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
                iri: &atom.property.0,
            },
            object: match &atom.value {
                Value::NamedNode(s) | Value::BlankNode(s) => NamedNode { iri: s.as_str() }.into(),
                Value::LiteralSimple(s) => Literal::Simple { value: s.as_str() }.into(),
                Value::LiteralLang(s, l) => Literal::LanguageTaggedString {
                    value: s,
                    language: l,
                }
                .into(),
                Value::LiteralDatatype(s, d) => Literal::Typed {
                    value: s,
                    datatype: NamedNode { iri: d },
                }
                .into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
