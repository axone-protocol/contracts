use cosmwasm_std::{Int64, SignedDecimal};

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    /// An atom, e.g., `foo`
    Atom(String),
    /// A integer number, e.g., `42`, `-7`
    Integer(Int64),
    /// A floating-point number, e.g., `3.14`, `-0.001`
    Float(SignedDecimal),
    /// A variable, e.g., `X`
    Variable(String),
    /// A compound term, e.g., `foo(bar, baz)`
    Compound(String, Vec<Term>),
    /// A list, e.g., `[a, b, c]` or `[a, b | Tail]`
    List(Vec<Term>, Option<Box<Term>>),
    /// A dictionary, e.g., `dict{key1: value1, key2: value2}`
    Dict(String, Vec<(String, Term)>),
}
