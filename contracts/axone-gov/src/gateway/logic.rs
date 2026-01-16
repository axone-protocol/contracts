use cosmwasm_std::{CustomQuery, QuerierWrapper, StdResult};
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct QueryServiceAskRequest {
    pub program: String,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

impl QueryServiceAskRequest {
    pub fn new(program: impl Into<String>, query: impl Into<String>, limit: Option<u64>) -> Self {
        Self {
            program: program.into(),
            query: query.into(),
            limit,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AxoneLogicQuery {
    Ask(QueryServiceAskRequest),
}

impl CustomQuery for AxoneLogicQuery {}

impl From<QueryServiceAskRequest> for AxoneLogicQuery {
    fn from(request: QueryServiceAskRequest) -> Self {
        Self::Ask(request)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct QueryServiceAskResponse {
    pub height: u64,
    pub gas_used: u64,
    pub answer: Option<Answer>,
    pub user_output: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Substitution {
    pub variable: String,
    pub expression: String,
}

impl Substitution {
    pub fn expression_term(&self) -> std::result::Result<Term, ParseError> {
        Parser::new(&self.expression)?.parse_root()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Result {
    pub error: Option<String>,
    pub substitutions: Vec<Substitution>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Answer {
    pub has_more: bool,
    pub variables: Vec<String>,
    pub results: Vec<Result>,
}

/// Build a Prolog decide query without motivation.
pub fn build_decide_query(case: &Case) -> String {
    format!("decide({}, Verdict).", case.as_ref())
}

/// Build a Prolog decide query with motivation.
pub fn build_decide_query_with_motivation(case: &Case) -> String {
    format!("decide({}, Verdict, Motivation).", case.as_ref())
}

#[cfg(not(feature = "mock-logic-query"))]
pub fn query_service_ask(
    querier: &QuerierWrapper<'_, AxoneLogicQuery>,
    request: QueryServiceAskRequest,
) -> StdResult<QueryServiceAskResponse> {
    let query: AxoneLogicQuery = request.into();
    querier.query(&query.into())
}

#[cfg(feature = "mock-logic-query")]
mod mock {
    use super::*;
    use cosmwasm_std::StdError;
    use std::cell::RefCell;

    type QueryServiceAskHandler =
        dyn Fn(&QueryServiceAskRequest) -> StdResult<QueryServiceAskResponse>;

    thread_local! {
        static QUERY_SERVICE_ASK_HANDLER: RefCell<Option<Box<QueryServiceAskHandler>>> =
            RefCell::new(None);
    }

    pub struct QueryServiceAskMockGuard {
        previous: Option<Box<QueryServiceAskHandler>>,
    }

    impl Drop for QueryServiceAskMockGuard {
        fn drop(&mut self) {
            let previous = self.previous.take();
            QUERY_SERVICE_ASK_HANDLER.with(|cell| {
                *cell.borrow_mut() = previous;
            });
        }
    }

    pub fn set_query_service_ask_handler<F>(handler: F) -> QueryServiceAskMockGuard
    where
        F: Fn(&QueryServiceAskRequest) -> StdResult<QueryServiceAskResponse> + 'static,
    {
        let previous = QUERY_SERVICE_ASK_HANDLER.with(|cell| {
            let mut cell = cell.borrow_mut();
            let previous = cell.take();
            *cell = Some(Box::new(handler));
            previous
        });

        QueryServiceAskMockGuard { previous }
    }

    pub fn query_service_ask(
        _querier: &QuerierWrapper<AxoneLogicQuery>,
        request: QueryServiceAskRequest,
    ) -> StdResult<QueryServiceAskResponse> {
        QUERY_SERVICE_ASK_HANDLER.with(|cell| {
            let cell = cell.borrow();
            let handler = cell
                .as_ref()
                .ok_or_else(|| StdError::not_found("mock query handler"))?;
            handler(&request)
        })
    }
}

#[cfg(feature = "mock-logic-query")]
pub use mock::{query_service_ask, set_query_service_ask_handler, QueryServiceAskMockGuard};
