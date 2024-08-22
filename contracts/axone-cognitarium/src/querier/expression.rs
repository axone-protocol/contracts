use crate::msg;
use crate::querier::mapper::iri_as_string;
use crate::querier::ResolvedVariables;
use crate::state::NamespaceSolver;
use cosmwasm_std::{StdError, StdResult};
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

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
}

impl Expression {
    pub fn bound_variables(&self) -> BTreeSet<usize> {
        let mut vars = BTreeSet::new();
        self.lookup_bound_variables(&mut |v| {
            vars.insert(v);
        });
        vars
    }

    pub fn lookup_bound_variables(&self, callback: &mut impl FnMut(usize)) {
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
        }
    }

    pub fn evaluate<'a>(
        &self,
        vars: &'a ResolvedVariables,
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
                return Ok(Term::Boolean(true));
            }
            Expression::Or(exprs) => {
                for expr in exprs {
                    if expr.evaluate(vars, ns_solver)?.as_bool() {
                        return Ok(Term::Boolean(true));
                    }
                }
                return Ok(Term::Boolean(false));
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
                format!("{}{}", value, language).to_string()
            }
            msg::Literal::TypedValue { value, datatype } => {
                format!("{}{}", value, iri_as_string(datatype, prefixes)?).to_string()
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
