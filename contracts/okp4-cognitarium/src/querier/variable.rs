use crate::msg::{Value, IRI};
use crate::state::{Literal, Object, Predicate, Subject};
use cosmwasm_std::StdResult;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ResolvedVariable {
    Subject(Subject),
    Predicate(Predicate),
    Object(Object),
}

impl ResolvedVariable {
    pub fn as_subject(&self) -> Option<Subject> {
        Some(match self {
            ResolvedVariable::Subject(s) => s.clone(),
            ResolvedVariable::Predicate(p) => Subject::Named(p.clone()),
            ResolvedVariable::Object(o) => match o {
                Object::Named(node) => Subject::Named(node.clone()),
                Object::Blank(node) => Subject::Blank(node.clone()),
                Object::Literal(_) => None?,
            },
        })
    }

    pub fn as_predicate(&self) -> Option<Predicate> {
        Some(match self {
            ResolvedVariable::Subject(s) => match s {
                Subject::Named(node) => node.clone(),
                Subject::Blank(_) => None?,
            },
            ResolvedVariable::Predicate(p) => p.clone(),
            ResolvedVariable::Object(o) => match o {
                Object::Named(node) => node.clone(),
                Object::Blank(_) => None?,
                Object::Literal(_) => None?,
            },
        })
    }

    pub fn as_object(&self) -> Option<Object> {
        Some(match self {
            ResolvedVariable::Subject(s) => match s {
                Subject::Named(node) => Object::Named(node.clone()),
                Subject::Blank(node) => Object::Blank(node.clone()),
            },
            ResolvedVariable::Predicate(p) => Object::Named(p.clone()),
            ResolvedVariable::Object(o) => o.clone(),
        })
    }

    pub fn as_value<F>(&self, ns_fn: &mut F) -> StdResult<Value>
    where
        F: FnMut(u128) -> StdResult<String>,
    {
        Ok(match self {
            ResolvedVariable::Subject(subject) => match subject {
                Subject::Named(named) => named.as_iri(ns_fn).map(|iri| Value::URI {
                    value: IRI::Full(iri),
                })?,
                Subject::Blank(blank) => Value::BlankNode {
                    value: blank.to_string(),
                },
            },
            ResolvedVariable::Predicate(predicate) => {
                predicate.as_iri(ns_fn).map(|iri| Value::URI {
                    value: IRI::Full(iri),
                })?
            }
            ResolvedVariable::Object(object) => match object {
                Object::Named(named) => Value::URI {
                    value: IRI::Full(named.as_iri(ns_fn)?),
                },
                Object::Blank(blank) => Value::BlankNode {
                    value: blank.to_string(),
                },
                Object::Literal(literal) => match literal {
                    Literal::Simple { value } => Value::Literal {
                        value: value.clone(),
                        lang: None,
                        datatype: None,
                    },
                    Literal::I18NString { value, language } => Value::Literal {
                        value: value.clone(),
                        lang: Some(language.clone()),
                        datatype: None,
                    },
                    Literal::Typed { value, datatype } => Value::Literal {
                        value: value.clone(),
                        lang: None,
                        datatype: Some(datatype.as_iri(ns_fn).map(IRI::Full)?),
                    },
                },
            },
        })
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ResolvedVariables {
    variables: Vec<Option<ResolvedVariable>>,
}

impl ResolvedVariables {
    pub fn with_capacity(cap: usize) -> Self {
        let mut variables = Vec::with_capacity(cap);
        for i in 0..cap {
            variables.insert(i, None);
        }

        Self { variables }
    }

    /// Merge with another set of resolved variables, returns None if a variable is set on both side
    /// with different values.
    pub fn merge_with(&self, other: &Self) -> Option<Self> {
        let mut merged = other.variables.clone();

        for (key, var) in self.variables.iter().enumerate() {
            if let Some(val) = var {
                match &other.variables[key] {
                    Some(other_val) => {
                        if val != other_val {
                            return None;
                        }
                    }
                    None => merged[key] = Some(val.clone()),
                }
            }
        }

        Some(Self { variables: merged })
    }

    pub fn set(&mut self, index: usize, var: ResolvedVariable) {
        self.variables[index] = Some(var)
    }

    pub fn get(&self, index: usize) -> &Option<ResolvedVariable> {
        self.variables.get(index).unwrap_or(&None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{BlankNode, Literal, Node};
    use cosmwasm_std::StdError;

    #[test]
    fn conversions() {
        let cases: Vec<(Option<Subject>, Option<Predicate>, Option<Object>)> = vec![
            (
                Some(Subject::Blank("_".to_string())),
                None,
                Some(Object::Blank("_".to_string())),
            ),
            (
                Some(Subject::Named(Node {
                    namespace: 4,
                    value: "test".to_string(),
                })),
                Some(Node {
                    namespace: 4,
                    value: "test".to_string(),
                }),
                Some(Object::Named(Node {
                    namespace: 4,
                    value: "test".to_string(),
                })),
            ),
            (
                None,
                None,
                Some(Object::Literal(Literal::I18NString {
                    value: "test".to_string(),
                    language: "en".to_string(),
                })),
            ),
        ];

        for (s, p, o) in cases {
            if let Some(ref subject) = s {
                let subject = ResolvedVariable::Subject(subject.clone());
                assert_eq!(subject.as_subject(), s);
                assert_eq!(subject.as_predicate(), p);
                assert_eq!(subject.as_object(), o);
            }
            if let Some(ref predicate) = p {
                let predicate = ResolvedVariable::Predicate(predicate.clone());
                assert_eq!(predicate.as_subject(), s);
                assert_eq!(predicate.as_predicate(), p);
                assert_eq!(predicate.as_object(), o);
            }
            if let Some(ref object) = o {
                let object = ResolvedVariable::Object(object.clone());
                assert_eq!(object.as_subject(), s);
                assert_eq!(object.as_predicate(), p);
                assert_eq!(object.as_object(), o);
            }
        }
    }

    fn ns(i: u128) -> StdResult<String> {
        match i {
            0 => Ok("foo".to_string()),
            1 => Ok("bar".to_string()),
            _ => Err(StdError::generic_err("namespace not found")),
        }
    }

    #[test]
    fn values() {
        let cases = vec![
            (
                ResolvedVariable::Subject(Subject::Named(Node {
                    namespace: 0,
                    value: "bar".to_string(),
                })),
                Ok(Value::URI {
                    value: IRI::Full("foobar".to_string()),
                }),
            ),
            (
                ResolvedVariable::Subject(Subject::Blank("_".to_string())),
                Ok(Value::BlankNode {
                    value: "_".to_string(),
                }),
            ),
            (
                ResolvedVariable::Predicate(Node {
                    namespace: 1,
                    value: "foo".to_string(),
                }),
                Ok(Value::URI {
                    value: IRI::Full("barfoo".to_string()),
                }),
            ),
            (
                ResolvedVariable::Object(Object::Named(Node {
                    namespace: 1,
                    value: "foo".to_string(),
                })),
                Ok(Value::URI {
                    value: IRI::Full("barfoo".to_string()),
                }),
            ),
            (
                ResolvedVariable::Object(Object::Blank("_".to_string())),
                Ok(Value::BlankNode {
                    value: "_".to_string(),
                }),
            ),
            (
                ResolvedVariable::Object(Object::Literal(Literal::Simple {
                    value: "foo".to_string(),
                })),
                Ok(Value::Literal {
                    value: "foo".to_string(),
                    lang: None,
                    datatype: None,
                }),
            ),
            (
                ResolvedVariable::Object(Object::Literal(Literal::I18NString {
                    value: "foo".to_string(),
                    language: "fr".to_string(),
                })),
                Ok(Value::Literal {
                    value: "foo".to_string(),
                    lang: Some("fr".to_string()),
                    datatype: None,
                }),
            ),
            (
                ResolvedVariable::Object(Object::Literal(Literal::Typed {
                    value: "foo".to_string(),
                    datatype: Node {
                        namespace: 0,
                        value: "bar".to_string(),
                    },
                })),
                Ok(Value::Literal {
                    value: "foo".to_string(),
                    lang: None,
                    datatype: Some(IRI::Full("foobar".to_string())),
                }),
            ),
        ];

        for (var, expected) in cases {
            assert_eq!(var.as_value(&mut ns), expected)
        }
    }
}
