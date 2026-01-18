use std::fmt;

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

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

fn write_atom(f: &mut fmt::Formatter<'_>, s: &str) -> fmt::Result {
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
    use crate::prolog::{parser::Parser, term as t};

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

    #[test]
    fn display_term() {
        let test_cases = vec![
            // Simple atoms (unquoted)
            ("foo", "foo", "simple lowercase atom"),
            ("'foo'", "foo", "quoted simple atom"),
            ("bar_baz", "bar_baz", "atom with underscore"),
            ("atom123", "atom123", "atom with numbers"),
            ("a", "a", "single letter atom"),
            ("test_atom_123", "test_atom_123", "complex simple atom"),
            (
                r"'and an emoji 👌'",
                r"'and an emoji 👌'",
                "atom with emoji",
            ),
            (r"'\x41\'", "'A'", "atom with escape sequence"),
            // Atoms requiring quotes
            ("'Foo'", "'Foo'", "atom starting with uppercase"),
            ("'123'", "'123'", "atom starting with number"),
            ("'foo bar'", "'foo bar'", "atom with space"),
            ("'foo-bar'", "'foo-bar'", "atom with hyphen"),
            ("'foo.bar'", "'foo.bar'", "atom with dot"),
            ("''", "''", "empty atom"),
            ("'hello''world'", "'hello''world'", "atom with single quote"),
            ("'a''b''c'", "'a''b''c'", "atom with multiple quotes"),
            (
                "'Hello World!'",
                "'Hello World!'",
                "atom with special chars",
            ),
            ("'_leading'", "'_leading'", "atom starting with underscore"),
            // Integers
            ("0", "0", "zero integer"),
            ("1", "1", "one integer"),
            ("42", "42", "positive integer"),
            ("-7", "-7", "negative integer"),
            ("1000000", "1000000", "large integer"),
            // Floats
            ("0.0", "0", "zero float"),
            ("1.0", "1", "one float"),
            ("3.14", "3.14", "positive float"),
            ("-0.001", "-0.001", "negative small float"),
            // Variables
            ("X", "X", "single letter variable"),
            ("Y", "Y", "another variable"),
            ("Result", "Result", "capitalized variable"),
            ("_", "_", "anonymous variable"),
            ("_Result", "_Result", "variable with underscore"),
            ("Var123", "Var123", "variable with numbers"),
            // Compounds with no arguments
            ("foo()", "foo()", "nullary compound"),
            ("test()", "test()", "another nullary"),
            // Compounds with one argument
            ("foo(bar)", "foo(bar)", "unary compound with atom"),
            ("parent(X)", "parent(X)", "unary compound with variable"),
            ("num(42)", "num(42)", "unary compound with integer"),
            // Compounds with multiple arguments
            ("foo(bar, baz)", "foo(bar, baz)", "binary compound"),
            ("add(1, 2, Result)", "add(1, 2, Result)", "ternary compound"),
            (
                "test(a, 1, X, 1.0)",
                "test(a, 1, X, 1)",
                "compound with mixed args",
            ),
            // Compounds with special atom names
            (
                "'Hello World'(test)",
                "'Hello World'(test)",
                "compound with quoted functor",
            ),
            // Nested compounds
            ("outer(inner(a))", "outer(inner(a))", "nested compound"),
            // Empty list
            ("[]", "[]", "empty list"),
            // Lists with single element
            ("[a]", "[a]", "list with one atom"),
            ("[1]", "[1]", "list with one integer"),
            ("[X]", "[X]", "list with one variable"),
            // Lists with multiple elements
            ("[a, b]", "[a, b]", "list with two atoms"),
            ("[1, 2, 3]", "[1, 2, 3]", "list with three integers"),
            ("[a, 1, X]", "[a, 1, X]", "list with mixed elements"),
            // Lists with tail
            ("[a | Tail]", "[a | Tail]", "list with variable tail"),
            (
                "[1, 2 | Rest]",
                "[1, 2 | Rest]",
                "list with multiple elements and tail",
            ),
            ("[a | [b, c]]", "[a | [b, c]]", "list with list tail"),
            // Nested lists
            ("[[a], [b]]", "[[a], [b]]", "list of lists"),
            (
                "[foo([1])]",
                "[foo([1])]",
                "list with compound containing list",
            ),
            // Empty dict
            ("tag{}", "tag{}", "empty dict"),
            ("ctx{}", "ctx{}", "empty dict with different tag"),
            // Dict with one pair
            (
                "user{name: alice}",
                "user{name: alice}",
                "dict with one atom value",
            ),
            (
                "data{count: 42}",
                "data{count: 42}",
                "dict with one integer value",
            ),
            (
                "result{value: X}",
                "result{value: X}",
                "dict with one variable value",
            ),
            // Dict with multiple pairs
            (
                "ctx{action: read, user: alice}",
                "ctx{action: read, user: alice}",
                "dict with two pairs",
            ),
            (
                "info{name: test, count: 42, active: Status}",
                "info{name: test, count: 42, active: Status}",
                "dict with three pairs",
            ),
            // Dict with special keys
            (
                "data{'my key': value}",
                "data{'my key': value}",
                "dict with quoted key",
            ),
            (
                "test{'Key-1': 1}",
                "test{'Key-1': 1}",
                "dict with key needing quotes",
            ),
            // Dict with special tag
            (
                "'My Tag'{key: value}",
                "'My Tag'{key: value}",
                "dict with quoted tag",
            ),
            // Dict with nested values
            (
                "outer{inner: foo(bar)}",
                "outer{inner: foo(bar)}",
                "dict with compound value",
            ),
            (
                "data{list: [1, 2]}",
                "data{list: [1, 2]}",
                "dict with list value",
            ),
            (
                "nested{dict: inner{key: value}}",
                "nested{dict: inner{key: value}}",
                "dict with dict value",
            ),
            // Complex nested structures
            (
                "query(select, [name, age], where{age: '>'(18), active: true})",
                "query(select, [name, age], where{age: '>'(18), active: true})",
                "complex query structure",
            ),
            (
                "[user{id: 1}, user{id: 2}]",
                "[user{id: 1}, user{id: 2}]",
                "list of dicts",
            ),
            (
                "tree(1, node(2, leaf(3), leaf(4)))",
                "tree(1, node(2, leaf(3), leaf(4)))",
                "tree structure",
            ),
            (
                "[head | [middle | Tail]]",
                "[head | [middle | Tail]]",
                "list with nested tail",
            ),
        ];

        for (input, expected, description) in test_cases {
            let term = Parser::new(input)
                .and_then(|p| p.parse_root())
                .unwrap_or_else(|e| {
                    panic!("Failed to parse '{}' ({}): {}", input, description, e.msg)
                });

            assert_eq!(
                format!("{}", term),
                expected,
                "Failed for case: {} (input: '{}')",
                description,
                input
            );
        }
    }
}
