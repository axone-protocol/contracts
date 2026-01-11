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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_ground() {
        let test_cases = vec![
            (Term::Atom("foo".to_string()), true, "atom"),
            (Term::Integer(Int64::new(42)), true, "integer"),
            (Term::Float(SignedDecimal::one()), true, "float"),
            (Term::Variable("X".to_string()), false, "variable"),
            (
                Term::Compound(
                    "foo".to_string(),
                    vec![Term::Atom("bar".to_string()), Term::Integer(Int64::new(1))],
                ),
                true,
                "compound with ground args",
            ),
            (
                Term::Compound(
                    "foo".to_string(),
                    vec![
                        Term::Atom("bar".to_string()),
                        Term::Variable("X".to_string()),
                    ],
                ),
                false,
                "compound with variable arg",
            ),
            (
                Term::Compound("foo".to_string(), vec![]),
                true,
                "compound with no args",
            ),
            (
                Term::List(
                    vec![Term::Atom("a".to_string()), Term::Integer(Int64::new(1))],
                    None,
                ),
                true,
                "list with ground elements",
            ),
            (
                Term::List(
                    vec![Term::Atom("a".to_string()), Term::Variable("X".to_string())],
                    None,
                ),
                false,
                "list with variable element",
            ),
            (Term::List(vec![], None), true, "empty list"),
            (
                Term::List(
                    vec![Term::Atom("a".to_string())],
                    Some(Box::new(Term::List(
                        vec![Term::Integer(Int64::new(2))],
                        None,
                    ))),
                ),
                true,
                "list with ground tail",
            ),
            (
                Term::List(
                    vec![Term::Atom("a".to_string())],
                    Some(Box::new(Term::Variable("Tail".to_string()))),
                ),
                false,
                "list with variable tail",
            ),
            (
                Term::Dict(
                    "ctx".to_string(),
                    vec![
                        ("action".to_string(), Term::Atom("read".to_string())),
                        ("user".to_string(), Term::Integer(Int64::new(123))),
                    ],
                ),
                true,
                "dict with ground values",
            ),
            (
                Term::Dict(
                    "ctx".to_string(),
                    vec![
                        ("action".to_string(), Term::Atom("read".to_string())),
                        ("user".to_string(), Term::Variable("User".to_string())),
                    ],
                ),
                false,
                "dict with variable value",
            ),
            (Term::Dict("empty".to_string(), vec![]), true, "empty dict"),
            (
                Term::Dict(
                    "ctx".to_string(),
                    vec![(
                        "nested".to_string(),
                        Term::Compound(
                            "foo".to_string(),
                            vec![Term::List(vec![Term::Atom("a".to_string())], None)],
                        ),
                    )],
                ),
                true,
                "nested ground structures",
            ),
            (
                Term::Dict(
                    "ctx".to_string(),
                    vec![(
                        "nested".to_string(),
                        Term::Compound(
                            "foo".to_string(),
                            vec![Term::List(vec![Term::Variable("X".to_string())], None)],
                        ),
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
