use std::fmt::{self};

use crate::contract::AxoneGovResult;
use crate::error::AxoneGovError;
use crate::prolog::ast::Term;
use crate::prolog::parser::{ParseError, Parser};

/// A case as a ground Prolog dictionary.
#[derive(Clone, Debug, PartialEq)]
pub struct Case(Term);

impl Default for Case {
    fn default() -> Self {
        Self(Term::Dict("ctx".to_string(), vec![]))
    }
}

impl Case {
    /// Parse a case from a string.
    ///
    /// Returns an error if the input is not valid Prolog syntax,
    /// not a dictionary, or contains variables (is not ground).
    pub fn new(input: &str) -> AxoneGovResult<Self> {
        let term = parse_term(input)?;
        term.try_into()
    }

    /// Merge another case into this one.
    ///
    /// Keys from `other` replace keys in `self` if they collide. The dictionary
    /// tag of `self` is preserved.
    pub fn merge(&mut self, other: &Case) {
        let Term::Dict(_, pairs) = &mut self.0 else {
            unreachable!("Case invariant violated: expected Prolog dict");
        };
        let Term::Dict(_, other_pairs) = &other.0 else {
            unreachable!("Case invariant violated: expected Prolog dict");
        };

        for (key, value) in other_pairs {
            pairs.retain(|(k, _)| k != key);
            pairs.push((key.clone(), value.clone()));
        }
    }
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<Term> for Case {
    fn as_ref(&self) -> &Term {
        &self.0
    }
}

impl TryFrom<Term> for Case {
    type Error = AxoneGovError;

    fn try_from(term: Term) -> Result<Self, Self::Error> {
        validate_case(&term)?;
        Ok(Self(term))
    }
}

fn validate_case(term: &Term) -> AxoneGovResult<()> {
    match term {
        Term::Dict(_, _) => {
            if !term.is_ground() {
                return Err(AxoneGovError::InvalidCase(
                    "case must be ground (no variables)".to_string(),
                ));
            }
            Ok(())
        }
        _ => Err(AxoneGovError::InvalidCase(
            "case must be a Prolog dict".to_string(),
        )),
    }
}

fn parse_term(input: &str) -> AxoneGovResult<Term> {
    let parser = Parser::new(input).map_err(parse_error)?;
    parser.parse_root().map_err(parse_error)
}

fn parse_error(err: ParseError) -> AxoneGovError {
    AxoneGovError::InvalidCase(format!("syntax error at offset {}: {}", err.at, err.msg))
}
