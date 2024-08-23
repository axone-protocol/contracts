use crate::msg::{Value, IRI};
use crate::querier::expression::Term;
use crate::state::{Literal, NamespaceSolver, Object, Predicate, Subject};
use axone_rdf::normalize::IdentifierIssuer;
use cosmwasm_std::StdResult;
use std::collections::BTreeSet;

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
                Object::Blank(node) => Subject::Blank(*node),
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
                Object::Blank(_) | Object::Literal(_) => None?,
            },
        })
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn as_object(&self) -> Option<Object> {
        Some(match self {
            ResolvedVariable::Subject(s) => match s {
                Subject::Named(node) => Object::Named(node.clone()),
                Subject::Blank(node) => Object::Blank(*node),
            },
            ResolvedVariable::Predicate(p) => Object::Named(p.clone()),
            ResolvedVariable::Object(o) => o.clone(),
        })
    }

    pub fn as_value(
        &self,
        ns_fn: &mut dyn NamespaceSolver,
        id_issuer: &mut IdentifierIssuer,
    ) -> StdResult<Value> {
        Ok(match self {
            ResolvedVariable::Subject(subject) => match subject {
                Subject::Named(named) => named.as_iri(ns_fn).map(|iri| Value::URI {
                    value: IRI::Full(iri),
                })?,
                Subject::Blank(blank) => Value::BlankNode {
                    value: id_issuer.get_str_or_issue(blank.to_string()).to_string(),
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
                    value: id_issuer.get_str_or_issue(blank.to_string()).to_string(),
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

    pub fn as_term(&self, ns_solver: &mut dyn NamespaceSolver) -> StdResult<Term> {
        Ok(match self {
            ResolvedVariable::Subject(subject) => match subject {
                Subject::Named(named) => named.as_iri(ns_solver).map(Term::String)?,
                Subject::Blank(blank) => Term::String(format!("_:{}", blank)),
            },
            ResolvedVariable::Predicate(predicate) => {
                predicate.as_iri(ns_solver).map(Term::String)?
            }
            ResolvedVariable::Object(object) => match object {
                Object::Named(named) => named.as_iri(ns_solver).map(Term::String)?,
                Object::Blank(blank) => Term::String(format!("_:{}", blank)),
                Object::Literal(literal) => Term::String(match literal {
                    Literal::Simple { value } => value.clone(),
                    Literal::I18NString { value, language } => {
                        format!("{}{}", value, language)
                    }
                    Literal::Typed { value, datatype } => {
                        format!("{}{}", value, datatype.as_iri(ns_solver)?)
                    }
                }),
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

    pub fn merge_index(&mut self, index: usize, var: ResolvedVariable) -> Option<()> {
        if let Some(old) = self.get(index) {
            (*old == var).then_some(())
        } else {
            self.variables[index] = Some(var);
            Some(())
        }
    }

    pub fn get(&self, index: usize) -> &Option<ResolvedVariable> {
        self.variables.get(index).unwrap_or(&None)
    }
}

pub trait HasBoundVariables {
    fn bound_variables(&self) -> BTreeSet<usize> {
        let mut vars = BTreeSet::new();
        self.lookup_bound_variables(&mut |v| {
            vars.insert(v);
        });
        vars
    }

    fn lookup_bound_variables(&self, callback: &mut impl FnMut(usize));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{InMemoryNamespaceSolver, Literal, Node};
    use cosmwasm_std::StdError;

    #[test]
    fn conversions() {
        let cases: Vec<(Option<Subject>, Option<Predicate>, Option<Object>)> = vec![
            (
                Some(Subject::Blank(0u128)),
                None,
                Some(Object::Blank(0u128)),
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
                ResolvedVariable::Subject(Subject::Blank(0u128)),
                Ok(Value::BlankNode {
                    value: "b0".to_string(),
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
                ResolvedVariable::Object(Object::Blank(0u128)),
                Ok(Value::BlankNode {
                    value: "b0".to_string(),
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
            (
                ResolvedVariable::Subject(Subject::Named(Node {
                    namespace: 12,
                    value: "unknown".to_string(),
                })),
                Err(StdError::not_found("Namespace")),
            ),
            (
                ResolvedVariable::Predicate(Node {
                    namespace: 12,
                    value: "unknown".to_string(),
                }),
                Err(StdError::not_found("Namespace")),
            ),
            (
                ResolvedVariable::Object(Object::Named(Node {
                    namespace: 12,
                    value: "unknown".to_string(),
                })),
                Err(StdError::not_found("Namespace")),
            ),
            (
                ResolvedVariable::Object(Object::Literal(Literal::Typed {
                    datatype: Node {
                        namespace: 12,
                        value: "unknown".to_string(),
                    },
                    value: "unknown".to_string(),
                })),
                Err(StdError::not_found("Namespace")),
            ),
        ];

        let mut id_issuer = IdentifierIssuer::new("b", 0u128);
        let mut ns_solver = InMemoryNamespaceSolver::with(vec![(0, "foo"), (1, "bar")]);
        for (var, expected) in cases {
            assert_eq!(var.as_value(&mut ns_solver, &mut id_issuer), expected)
        }
    }

    #[test]
    fn merged_variables() {
        let mut vars1 = ResolvedVariables::with_capacity(3);
        vars1.merge_index(0, ResolvedVariable::Object(Object::Blank(0u128)));
        vars1.merge_index(2, ResolvedVariable::Object(Object::Blank(1u128)));

        let mut vars2 = ResolvedVariables::with_capacity(3);
        vars2.merge_index(1, ResolvedVariable::Object(Object::Blank(2u128)));
        vars2.merge_index(2, ResolvedVariable::Object(Object::Blank(1u128)));

        assert_eq!(
            vars2.get(1),
            &Some(ResolvedVariable::Object(Object::Blank(2u128)))
        );
        assert_eq!(vars1.get(1), &None);

        let mut expected_result = ResolvedVariables::with_capacity(3);
        expected_result.merge_index(0, ResolvedVariable::Object(Object::Blank(0u128)));
        expected_result.merge_index(1, ResolvedVariable::Object(Object::Blank(2u128)));
        expected_result.merge_index(2, ResolvedVariable::Object(Object::Blank(1u128)));

        let result = vars1.merge_with(&vars2);
        assert_eq!(result, Some(expected_result));

        let mut vars3 = ResolvedVariables::with_capacity(3);
        vars3.merge_index(1, ResolvedVariable::Object(Object::Blank(2u128)));
        vars3.merge_index(
            2,
            ResolvedVariable::Predicate(Node {
                namespace: 0,
                value: "".to_string(),
            }),
        );
        let result2 = vars1.merge_with(&vars3);
        assert_eq!(result2, None);
    }

    #[test]
    fn terms() {
        let cases = vec![
            (
                ResolvedVariable::Subject(Subject::Named(Node {
                    namespace: 0,
                    value: "bar".to_string(),
                })),
                Ok(Term::String("foobar".to_string())),
            ),
            (
                ResolvedVariable::Subject(Subject::Blank(0u128)),
                Ok(Term::String("_:0".to_string())),
            ),
            (
                ResolvedVariable::Predicate(Node {
                    namespace: 1,
                    value: "foo".to_string(),
                }),
                Ok(Term::String("barfoo".to_string())),
            ),
            (
                ResolvedVariable::Object(Object::Named(Node {
                    namespace: 1,
                    value: "foo".to_string(),
                })),
                Ok(Term::String("barfoo".to_string())),
            ),
            (
                ResolvedVariable::Object(Object::Blank(0u128)),
                Ok(Term::String("_:0".to_string())),
            ),
            (
                ResolvedVariable::Object(Object::Literal(Literal::Simple {
                    value: "foo".to_string(),
                })),
                Ok(Term::String("foo".to_string())),
            ),
            (
                ResolvedVariable::Object(Object::Literal(Literal::I18NString {
                    value: "foo".to_string(),
                    language: "fr".to_string(),
                })),
                Ok(Term::String("foofr".to_string())),
            ),
            (
                ResolvedVariable::Object(Object::Literal(Literal::Typed {
                    value: "foo".to_string(),
                    datatype: Node {
                        namespace: 0,
                        value: "bar".to_string(),
                    },
                })),
                Ok(Term::String("foofoobar".to_string())),
            ),
            (
                ResolvedVariable::Subject(Subject::Named(Node {
                    namespace: 12,
                    value: "unknown".to_string(),
                })),
                Err(StdError::not_found("Namespace")),
            ),
            (
                ResolvedVariable::Predicate(Node {
                    namespace: 12,
                    value: "unknown".to_string(),
                }),
                Err(StdError::not_found("Namespace")),
            ),
            (
                ResolvedVariable::Object(Object::Named(Node {
                    namespace: 12,
                    value: "unknown".to_string(),
                })),
                Err(StdError::not_found("Namespace")),
            ),
            (
                ResolvedVariable::Object(Object::Literal(Literal::Typed {
                    datatype: Node {
                        namespace: 12,
                        value: "unknown".to_string(),
                    },
                    value: "unknown".to_string(),
                })),
                Err(StdError::not_found("Namespace")),
            ),
        ];

        let mut ns_solver = InMemoryNamespaceSolver::with(vec![(0, "foo"), (1, "bar")]);
        for (var, expected) in cases {
            assert_eq!(var.as_term(&mut ns_solver), expected)
        }
    }
}
