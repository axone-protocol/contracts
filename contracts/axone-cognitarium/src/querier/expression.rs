use crate::msg;
use crate::querier::mapper::iri_as_string;
use crate::querier::variable::HasBoundVariables;
use crate::querier::ResolvedVariables;
use crate::state::NamespaceSolver;
use cosmwasm_std::{StdError, StdResult};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Expression {
    Constant(Term),
    Variable(usize),
    And(Vec<Self>),
    Or(Vec<Self>),
    Equal(Box<Self>, Box<Self>),
    Greater(Box<Self>, Box<Self>),
    GreaterOrEqual(Box<Self>, Box<Self>),
    Less(Box<Self>, Box<Self>),
    LessOrEqual(Box<Self>, Box<Self>),
    Not(Box<Self>),
}

impl Expression {
    pub fn evaluate(
        &self,
        vars: &ResolvedVariables,
        ns_solver: &mut dyn NamespaceSolver,
    ) -> StdResult<Term> {
        match self {
            Expression::Constant(term) => Ok(term.clone()),
            Expression::Variable(v) => vars
                .get(*v)
                .clone()
                .ok_or(StdError::generic_err("Unbound filter variable"))
                .and_then(|v| v.as_term(ns_solver)),
            Expression::And(exprs) => {
                for expr in exprs {
                    if !expr.evaluate(vars, ns_solver)?.as_bool() {
                        return Ok(Term::Boolean(false));
                    }
                }
                Ok(Term::Boolean(true))
            }
            Expression::Or(exprs) => {
                for expr in exprs {
                    if expr.evaluate(vars, ns_solver)?.as_bool() {
                        return Ok(Term::Boolean(true));
                    }
                }
                Ok(Term::Boolean(false))
            }
            Expression::Equal(left, right) => Ok(Term::Boolean(
                left.evaluate(vars, ns_solver)? == right.evaluate(vars, ns_solver)?,
            )),
            Expression::Greater(left, right) => Ok(Term::Boolean(
                left.evaluate(vars, ns_solver)? > right.evaluate(vars, ns_solver)?,
            )),
            Expression::GreaterOrEqual(left, right) => Ok(Term::Boolean(
                left.evaluate(vars, ns_solver)? >= right.evaluate(vars, ns_solver)?,
            )),
            Expression::Less(left, right) => Ok(Term::Boolean(
                left.evaluate(vars, ns_solver)? < right.evaluate(vars, ns_solver)?,
            )),
            Expression::LessOrEqual(left, right) => Ok(Term::Boolean(
                left.evaluate(vars, ns_solver)? <= right.evaluate(vars, ns_solver)?,
            )),
            Expression::Not(expr) => Ok(Term::Boolean(!expr.evaluate(vars, ns_solver)?.as_bool())),
        }
    }
}

impl HasBoundVariables for Expression {
    fn lookup_bound_variables(&self, callback: &mut impl FnMut(usize)) {
        match self {
            Expression::Constant(_) => {}
            Expression::Variable(v) => {
                callback(*v);
            }
            Expression::And(exprs) | Expression::Or(exprs) => {
                exprs
                    .iter()
                    .for_each(|e| e.lookup_bound_variables(callback));
            }
            Expression::Equal(left, right)
            | Expression::Greater(left, right)
            | Expression::GreaterOrEqual(left, right)
            | Expression::Less(left, right)
            | Expression::LessOrEqual(left, right) => {
                left.lookup_bound_variables(callback);
                right.lookup_bound_variables(callback);
            }
            Expression::Not(expr) => {
                expr.lookup_bound_variables(callback);
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Term {
    String(String),
    Boolean(bool),
}

impl Term {
    pub fn from_iri(iri: msg::IRI, prefixes: &HashMap<String, String>) -> StdResult<Self> {
        Ok(Term::String(iri_as_string(iri, prefixes)?))
    }

    pub fn from_literal(
        literal: msg::Literal,
        prefixes: &HashMap<String, String>,
    ) -> StdResult<Self> {
        Ok(Term::String(match literal {
            msg::Literal::Simple(value) => value,
            msg::Literal::LanguageTaggedString { value, language } => {
                format!("{}{}", value, language)
            }
            msg::Literal::TypedValue { value, datatype } => {
                format!("{}{}", value, iri_as_string(datatype, prefixes)?)
            }
        }))
    }

    pub fn as_string(&self) -> String {
        match self {
            Term::String(t) => t.clone(),
            Term::Boolean(b) => b.to_string(),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Term::String(s) => !s.is_empty(),
            Term::Boolean(b) => *b,
        }
    }
}

impl PartialOrd<Term> for Term {
    fn partial_cmp(&self, other: &Term) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        match (self, other) {
            (Term::String(left), Term::String(right)) => Some(left.cmp(right)),
            (Term::Boolean(left), Term::Boolean(right)) => Some(left.cmp(right)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::querier::variable::ResolvedVariable;
    use crate::state::{InMemoryNamespaceSolver, Node, Object};
    use std::collections::BTreeSet;

    #[test]
    fn expression_bound_variables() {
        let cases = vec![
            (
                Expression::Constant(Term::String("foo".to_string())),
                vec![],
            ),
            (Expression::Variable(0), vec![0]),
            (
                Expression::And(vec![Expression::Variable(0), Expression::Variable(1)]),
                vec![0, 1],
            ),
            (
                Expression::Or(vec![Expression::Variable(0), Expression::Variable(1)]),
                vec![0, 1],
            ),
            (
                Expression::Equal(
                    Box::new(Expression::Variable(0)),
                    Box::new(Expression::Variable(1)),
                ),
                vec![0, 1],
            ),
            (
                Expression::Greater(
                    Box::new(Expression::Variable(0)),
                    Box::new(Expression::Variable(1)),
                ),
                vec![0, 1],
            ),
            (
                Expression::GreaterOrEqual(
                    Box::new(Expression::Variable(0)),
                    Box::new(Expression::Variable(1)),
                ),
                vec![0, 1],
            ),
            (
                Expression::Less(
                    Box::new(Expression::Variable(0)),
                    Box::new(Expression::Variable(1)),
                ),
                vec![0, 1],
            ),
            (
                Expression::LessOrEqual(
                    Box::new(Expression::Variable(0)),
                    Box::new(Expression::Variable(1)),
                ),
                vec![0, 1],
            ),
            (Expression::Not(Box::new(Expression::Variable(0))), vec![0]),
        ];

        for case in cases {
            assert_eq!(case.0.bound_variables(), BTreeSet::from_iter(case.1));
        }
    }

    #[test]
    fn expression_evaluate() {
        let cases = vec![
            (
                Expression::Constant(Term::Boolean(true)),
                Ok(Term::Boolean(true)),
            ),
            (
                Expression::Variable(0),
                Ok(Term::String("http:://example.com/foo".to_string())),
            ),
            (
                Expression::Variable(1),
                Err(StdError::not_found("Namespace")),
            ),
            (
                Expression::Variable(12),
                Err(StdError::generic_err("Unbound filter variable")),
            ),
            (
                Expression::And(vec![
                    Expression::Constant(Term::Boolean(true)),
                    Expression::Constant(Term::Boolean(true)),
                ]),
                Ok(Term::Boolean(true)),
            ),
            (Expression::And(vec![]), Ok(Term::Boolean(true))),
            (
                Expression::And(vec![
                    Expression::Constant(Term::Boolean(true)),
                    Expression::Constant(Term::Boolean(false)),
                ]),
                Ok(Term::Boolean(false)),
            ),
            (
                Expression::Or(vec![
                    Expression::Constant(Term::Boolean(true)),
                    Expression::Constant(Term::Boolean(false)),
                ]),
                Ok(Term::Boolean(true)),
            ),
            (Expression::Or(vec![]), Ok(Term::Boolean(false))),
            (
                Expression::Or(vec![
                    Expression::Constant(Term::Boolean(false)),
                    Expression::Constant(Term::Boolean(false)),
                ]),
                Ok(Term::Boolean(false)),
            ),
            (
                Expression::Equal(
                    Box::new(Expression::Constant(Term::String("foo".to_string()))),
                    Box::new(Expression::Constant(Term::String("foo".to_string()))),
                ),
                Ok(Term::Boolean(true)),
            ),
            (
                Expression::Equal(
                    Box::new(Expression::Constant(Term::String("foo".to_string()))),
                    Box::new(Expression::Constant(Term::String("bar".to_string()))),
                ),
                Ok(Term::Boolean(false)),
            ),
            (
                Expression::Greater(
                    Box::new(Expression::Constant(Term::String("1".to_string()))),
                    Box::new(Expression::Constant(Term::String("1".to_string()))),
                ),
                Ok(Term::Boolean(false)),
            ),
            (
                Expression::Greater(
                    Box::new(Expression::Constant(Term::String("2".to_string()))),
                    Box::new(Expression::Constant(Term::String("1".to_string()))),
                ),
                Ok(Term::Boolean(true)),
            ),
            (
                Expression::GreaterOrEqual(
                    Box::new(Expression::Constant(Term::String("1".to_string()))),
                    Box::new(Expression::Constant(Term::String("2".to_string()))),
                ),
                Ok(Term::Boolean(false)),
            ),
            (
                Expression::GreaterOrEqual(
                    Box::new(Expression::Constant(Term::String("1".to_string()))),
                    Box::new(Expression::Constant(Term::String("1".to_string()))),
                ),
                Ok(Term::Boolean(true)),
            ),
            (
                Expression::GreaterOrEqual(
                    Box::new(Expression::Constant(Term::String("2".to_string()))),
                    Box::new(Expression::Constant(Term::String("1".to_string()))),
                ),
                Ok(Term::Boolean(true)),
            ),
            (
                Expression::Less(
                    Box::new(Expression::Constant(Term::String("1".to_string()))),
                    Box::new(Expression::Constant(Term::String("2".to_string()))),
                ),
                Ok(Term::Boolean(true)),
            ),
            (
                Expression::Less(
                    Box::new(Expression::Constant(Term::String("1".to_string()))),
                    Box::new(Expression::Constant(Term::String("1".to_string()))),
                ),
                Ok(Term::Boolean(false)),
            ),
            (
                Expression::LessOrEqual(
                    Box::new(Expression::Constant(Term::String("1".to_string()))),
                    Box::new(Expression::Constant(Term::String("2".to_string()))),
                ),
                Ok(Term::Boolean(true)),
            ),
            (
                Expression::LessOrEqual(
                    Box::new(Expression::Constant(Term::String("1".to_string()))),
                    Box::new(Expression::Constant(Term::String("1".to_string()))),
                ),
                Ok(Term::Boolean(true)),
            ),
            (
                Expression::LessOrEqual(
                    Box::new(Expression::Constant(Term::String("2".to_string()))),
                    Box::new(Expression::Constant(Term::String("1".to_string()))),
                ),
                Ok(Term::Boolean(false)),
            ),
            (
                Expression::Not(Box::new(Expression::Constant(Term::Boolean(true)))),
                Ok(Term::Boolean(false)),
            ),
            (
                Expression::Not(Box::new(Expression::Constant(Term::Boolean(false)))),
                Ok(Term::Boolean(true)),
            ),
        ];

        let mut vars = ResolvedVariables::with_capacity(2);
        vars.merge_index(
            0,
            ResolvedVariable::Object(Object::Named(Node {
                namespace: 0,
                value: "foo".to_string(),
            })),
        );
        vars.merge_index(
            1,
            ResolvedVariable::Object(Object::Named(Node {
                namespace: 12,
                value: "foo".to_string(),
            })),
        );

        let mut ns_solver = InMemoryNamespaceSolver::with(vec![(0, "http:://example.com/")]);
        for case in cases {
            assert_eq!(case.0.evaluate(&vars, &mut ns_solver), case.1);
        }
    }

    #[test]
    fn term_from_iri() {
        let cases = vec![
            (
                msg::IRI::Prefixed("foo:bar".to_string()),
                Ok(Term::String("http://example.com/bar".to_string())),
            ),
            (
                msg::IRI::Full("foo:bar".to_string()),
                Ok(Term::String("foo:bar".to_string())),
            ),
            (
                msg::IRI::Prefixed("unknown:bar".to_string()),
                Err(StdError::generic_err("Prefix not found: unknown")),
            ),
        ];

        let mut prefixes = HashMap::new();
        prefixes.insert("foo".to_string(), "http://example.com/".to_string());

        for case in cases {
            assert_eq!(Term::from_iri(case.0, &prefixes), case.1);
        }
    }

    #[test]
    fn term_from_literal() {
        let cases = vec![
            (
                msg::Literal::Simple("foo".to_string()),
                Ok(Term::String("foo".to_string())),
            ),
            (
                msg::Literal::LanguageTaggedString {
                    value: "foo".to_string(),
                    language: "en".to_string(),
                },
                Ok(Term::String("fooen".to_string())),
            ),
            (
                msg::Literal::TypedValue {
                    value: "foo".to_string(),
                    datatype: msg::IRI::Prefixed("foo:bar".to_string()),
                },
                Ok(Term::String("foohttp://example.com/bar".to_string())),
            ),
            (
                msg::Literal::TypedValue {
                    value: "foo".to_string(),
                    datatype: msg::IRI::Prefixed("unknown:bar".to_string()),
                },
                Err(StdError::generic_err("Prefix not found: unknown")),
            ),
        ];

        let mut prefixes = HashMap::new();
        prefixes.insert("foo".to_string(), "http://example.com/".to_string());

        for case in cases {
            assert_eq!(Term::from_literal(case.0, &prefixes), case.1);
        }
    }

    #[test]
    fn term_as_string() {
        let cases = vec![
            (Term::String("foo".to_string()), "foo"),
            (Term::Boolean(true), "true"),
            (Term::Boolean(false), "false"),
        ];
        for case in cases {
            assert_eq!(case.0.as_string(), case.1);
        }
    }

    #[test]
    fn term_as_bool() {
        let cases = vec![
            (Term::String("foo".to_string()), true),
            (Term::String("".to_string()), false),
            (Term::Boolean(true), true),
            (Term::Boolean(false), false),
        ];
        for case in cases {
            assert_eq!(case.0.as_bool(), case.1);
        }
    }

    #[test]
    fn term_partial_cmp() {
        let cases = vec![
            (
                Term::String("a".to_string()),
                Term::String("b".to_string()),
                Some(Ordering::Less),
            ),
            (
                Term::String("b".to_string()),
                Term::String("a".to_string()),
                Some(Ordering::Greater),
            ),
            (
                Term::String("a".to_string()),
                Term::String("a".to_string()),
                Some(Ordering::Equal),
            ),
            (
                Term::Boolean(true),
                Term::Boolean(false),
                Some(Ordering::Greater),
            ),
            (
                Term::Boolean(false),
                Term::Boolean(true),
                Some(Ordering::Less),
            ),
            (
                Term::Boolean(true),
                Term::Boolean(true),
                Some(Ordering::Equal),
            ),
            (Term::String("a".to_string()), Term::Boolean(true), None),
            (Term::Boolean(true), Term::String("a".to_string()), None),
        ];
        for case in cases {
            assert_eq!(case.0.partial_cmp(&case.1), case.2);
        }
    }
}
