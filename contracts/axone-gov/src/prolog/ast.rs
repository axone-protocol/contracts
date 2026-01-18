use cosmwasm_std::{Int256, SignedDecimal};

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    /// An atom, e.g., `foo`
    Atom(String),
    /// A integer number, e.g., `42`, `-7`
    Integer(Int256),
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

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Atom(s) => write_atom(f, s),
            Term::Integer(i) => write!(f, "{}", i),
            Term::Float(val) => write!(f, "{}", val),
            Term::Variable(s) => write!(f, "{}", s),
            Term::Compound(name, args) => {
                write_atom(f, name)?;
                write!(f, "(")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Term::List(elements, tail) => {
                write!(f, "[")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                if let Some(tail) = tail {
                    write!(f, " | {}", tail)?;
                }
                write!(f, "]")
            }
            Term::Dict(tag, pairs) => {
                write_atom(f, tag)?;
                write!(f, "{{")?;
                for (i, (key, value)) in pairs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write_atom(f, key)?;
                    write!(f, ": {}", value)?;
                }
                write!(f, "}}")
            }
        }
    }
}

fn write_atom(f: &mut std::fmt::Formatter<'_>, s: &str) -> std::fmt::Result {
    if is_simple_atom(s) {
        write!(f, "{}", s)
    } else {
        write!(f, "'{}'", s.replace('\'', "''"))
    }
}

fn is_simple_atom(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => chars.all(|c| c.is_ascii_alphanumeric() || c == '_'),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prolog::term as t;

    #[test]
    fn is_ground() {
        let test_cases = vec![
            (t::atom("foo"), true, "atom"),
            (42.into(), true, "integer"),
            (Term::Float(SignedDecimal::one()), true, "float"),
            (t::variable("X"), false, "variable"),
            (
                t::compound("foo", vec![t::atom("bar"), 1.into()]),
                true,
                "compound with ground args",
            ),
            (
                t::compound("foo", vec![t::atom("bar"), t::variable("X")]),
                false,
                "compound with variable arg",
            ),
            (t::compound("foo", vec![]), true, "compound with no args"),
            (
                t::list(vec![t::atom("a"), 1.into()]),
                true,
                "list with ground elements",
            ),
            (
                t::list(vec![t::atom("a"), t::variable("X")]),
                false,
                "list with variable element",
            ),
            (t::list(vec![]), true, "empty list"),
            (
                Term::List(vec![t::atom("a")], Some(Box::new(t::list(vec![2.into()])))),
                true,
                "list with ground tail",
            ),
            (
                Term::List(vec![t::atom("a")], Some(Box::new(t::variable("Tail")))),
                false,
                "list with variable tail",
            ),
            (
                t::dict(
                    "ctx",
                    vec![
                        ("action".to_string(), t::atom("read")),
                        ("user".to_string(), 123.into()),
                    ],
                ),
                true,
                "dict with ground values",
            ),
            (
                t::dict(
                    "ctx",
                    vec![
                        ("action".to_string(), t::atom("read")),
                        ("user".to_string(), t::variable("User")),
                    ],
                ),
                false,
                "dict with variable value",
            ),
            (t::dict("empty", vec![]), true, "empty dict"),
            (
                t::dict(
                    "ctx",
                    vec![(
                        "nested".to_string(),
                        t::compound("foo", vec![t::list(vec![t::atom("a")])]),
                    )],
                ),
                true,
                "nested ground structures",
            ),
            (
                t::dict(
                    "ctx",
                    vec![(
                        "nested".to_string(),
                        t::compound("foo", vec![t::list(vec![t::variable("X")])]),
                    )],
                ),
                false,
                "nested structures with variable",
            ),
        ];

        for (term, expected, description) in test_cases {
            assert_eq!(
                term.is_ground(),
                expected,
                "Failed for case: {}",
                description
            );
        }
    }
}
