use crate::prolog::ast::Term;

use cosmwasm_std::{Int256, Int64, Timestamp, Uint128, Uint256};

fn int256_from_uint256_infallible(u: Uint256) -> Int256 {
    match Int256::try_from(u) {
        Ok(i) => i,
        Err(_) => unreachable!("value always fits into Int256"),
    }
}

impl From<Uint128> for Term {
    fn from(v: Uint128) -> Self {
        let u = Uint256::from(v.u128());
        Term::Integer(int256_from_uint256_infallible(u))
    }
}

impl From<Int64> for Term {
    fn from(v: Int64) -> Self {
        Term::Integer(Int256::from_i128(v.into()))
    }
}

impl From<i64> for Term {
    fn from(v: i64) -> Self {
        Term::Integer(Int256::from_i128(v.into()))
    }
}

impl From<u64> for Term {
    fn from(v: u64) -> Self {
        Term::Integer(int256_from_uint256_infallible(Uint256::from(v)))
    }
}

impl From<i32> for Term {
    fn from(v: i32) -> Self {
        Term::Integer(Int256::from_i128(v.into()))
    }
}

impl From<u32> for Term {
    fn from(v: u32) -> Self {
        Term::Integer(int256_from_uint256_infallible(Uint256::from(v)))
    }
}

impl From<Timestamp> for Term {
    fn from(v: Timestamp) -> Self {
        v.seconds().into()
    }
}

pub fn atom(s: impl Into<String>) -> Term {
    Term::Atom(s.into())
}

pub fn dict(tag: impl Into<String>, pairs: Vec<(String, Term)>) -> Term {
    Term::Dict(tag.into(), pairs)
}

pub fn kv(key: impl Into<String>, value: Term) -> (String, Term) {
    (key.into(), value)
}

pub fn list(items: Vec<Term>) -> Term {
    Term::List(items, None)
}

pub fn compound(functor: impl Into<String>, args: Vec<Term>) -> Term {
    Term::Compound(functor.into(), args)
}

pub fn compound2(functor: impl Into<String>, arg1: Term, arg2: Term) -> Term {
    compound(functor, vec![arg1, arg2])
}

#[cfg(test)]
pub fn variable(name: impl Into<String>) -> Term {
    Term::Variable(name.into())
}
