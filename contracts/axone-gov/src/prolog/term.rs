use crate::prolog::ast::Term;

use cosmwasm_std::Int64;

impl From<Int64> for Term {
    fn from(v: Int64) -> Self {
        Term::Integer(v)
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
    compound(functor.into(), vec![arg1, arg2])
}
