use crate::contract::AxoneGovResult;
use crate::error::AxoneGovError;
use crate::prolog::ast::Term;
use crate::prolog::parser::{ParseError, Parser};

pub fn case(case: &str) -> AxoneGovResult<()> {
    let term = parse_term(case)?;
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
