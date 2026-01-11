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

impl Term {
    /// Returns true if the term is ground (fully instantiated with no variables).
    pub fn is_ground(&self) -> bool {
        match self {
            Term::Atom(_) | Term::Integer(_) | Term::Float(_) => true,
            Term::Variable(_) => false,
            Term::Compound(_, args) => args.iter().all(Term::is_ground),
            Term::List(elements, tail) => {
                elements.iter().all(Term::is_ground)
                    && tail.as_ref().map_or(true, |t| t.is_ground())
            }
            Term::Dict(_, pairs) => pairs.iter().all(|(_, value)| value.is_ground()),
        }
    }
}
