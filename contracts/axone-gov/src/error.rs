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

    #[error("invalid constitution: {0}")]
    InvalidConstitution(String),

    #[error("decision returned no answer")]
    DecisionNoAnswer,

    #[error("invalid case: {0}")]
    InvalidCase(String),

    #[error("decision failed: {0}")]
    DecisionFailed(String),

    #[error("decision returned no result")]
    DecisionNoResult,

    #[error("decision verdict missing in response")]
    DecisionMissingVerdict,

    #[error("decision motivation missing in response")]
    DecisionMissingMotivation,
}
