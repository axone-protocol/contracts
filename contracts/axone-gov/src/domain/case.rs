use crate::contract::AxoneGovResult;
use crate::error::AxoneGovError;
use crate::prolog::ast::Term;
use crate::prolog::parser::{ParseError, Parser};

/// A case as a ground Prolog dictionary.
#[derive(Clone, Debug, PartialEq)]
pub struct Case {
    raw: String,
}

impl Case {
    /// Parse a case from a string.
    ///
    /// Returns an error if the input is not valid Prolog syntax,
    /// not a dictionary, or contains variables.
    pub fn new(input: &str) -> AxoneGovResult<Self> {
        let term = parse_term(input)?;
        match term {
            Term::Dict(_, _) => {
                if !term.is_ground() {
                    return Err(AxoneGovError::InvalidCase(
                        "case must be ground (no variables)".to_string(),
                    ));
                }
                Ok(Self {
                    raw: input.to_string(),
                })
            }
            _ => Err(AxoneGovError::InvalidCase(
                "case must be a Prolog dict".to_string(),
            )),
        }
    }
}

impl AsRef<str> for Case {
    fn as_ref(&self) -> &str {
        &self.raw
    }
}

fn parse_term(input: &str) -> AxoneGovResult<Term> {
    let parser = Parser::new(input).map_err(parse_error)?;
    parser.parse_root().map_err(parse_error)
}

fn parse_error(err: ParseError) -> AxoneGovError {
    AxoneGovError::InvalidCase(format!("syntax error at offset {}: {}", err.at, err.msg))
}
