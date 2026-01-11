use abstract_app::sdk::AbstractSdkError;
use abstract_app::std::AbstractError;
use abstract_app::AppError;
use cosmwasm_std::StdError;
use cw_asset::AssetError;
use cw_controllers::AdminError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum AxoneGovError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Abstract(#[from] AbstractError),

    #[error("{0}")]
    AbstractSdk(#[from] AbstractSdkError),

    #[error("{0}")]
    Asset(#[from] AssetError),

    #[error("{0}")]
    Admin(#[from] AdminError),

    #[error("{0}")]
    DappError(#[from] AppError),

    // Constitution errors
    /// The constitution is not valid UTF-8.
    /// This occurs when the constitution binary cannot be decoded as UTF-8 text.
    #[error("constitution is not valid UTF-8: {0}")]
    ConstitutionUtf8(String),

    /// The constitution is invalid.
    /// This is raised when the constitution fails validation checks, such as missing
    /// required predicates or containing malformed Prolog code.
    #[error("constitution is invalid: {0}")]
    ConstitutionInvalid(String),

    // Prolog engine errors
    /// The Prolog engine returned no answer.
    /// This indicates the logic module VM failed to produce any response to a query.
    #[error("prolog engine returned no answer")]
    PrologEngineNoAnswer,

    /// The Prolog engine encountered an error during execution.
    /// This captures failures within the logic module VM itself.
    #[error("prolog engine error: {0}")]
    PrologEngineError(String),

    // Decision errors
    /// The case query parameter is invalid.
    /// This is raised when the provided case does not meet validation requirements.
    #[error("invalid case: {0}")]
    InvalidCase(String),

    /// The decision query failed with an error.
    /// This captures errors returned in the Prolog result during decision evaluation.
    #[error("decision failed: {0}")]
    DecisionFailed(String),

    /// The decision query returned no results.
    /// This occurs when the Prolog query succeeds but produces an empty result set.
    #[error("decision returned no result")]
    DecisionNoResult,

    /// The decision verdict is missing from the response.
    /// This is raised when the expected 'Verdict' variable is not found in substitutions.
    #[error("decision verdict missing in response")]
    DecisionMissingVerdict,

    /// The decision motivation is missing from the response.
    /// This is raised when a motivated decision is requested but the 'Motivation' variable
    /// is not found in substitutions.
    #[error("decision motivation missing in response")]
    DecisionMissingMotivation,
}
