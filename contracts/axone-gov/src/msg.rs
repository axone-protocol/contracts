use crate::contract::AxoneGov;

use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Binary;

abstract_app::app_msg_types!(AxoneGov, AxoneGovExecuteMsg, AxoneGovQueryMsg);

/// Instantiate message.
///
/// `constitution` is the Prolog program (UTF-8 bytes) that defines the governance rules.
/// The contract validates that it provides the required predicates (`decide/2` and
/// `decide/3`) during instantiation.
#[cosmwasm_schema::cw_serde]
#[derive(Default)]
pub struct AxoneGovInstantiateMsg {
    /// Prolog governance program.
    pub constitution: Binary,
}

/// Execute messages.
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum AxoneGovExecuteMsg {
    /// No-op execute message
    NoOp {},
}

/// Migrate message.
///
/// Reserved for future migrations.
#[cosmwasm_schema::cw_serde]
pub struct AxoneGovMigrateMsg {}

/// Query messages.
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum AxoneGovQueryMsg {
    /// Return the stored governance constitution program.
    #[returns(ConstitutionResponse)]
    Constitution {},

    /// Decide a case using the constitution's decide/2 or decide/3 predicate.
    ///
    /// The `case` must be a Prolog dict term string representing the decision context.
    /// If `motivated` is true, the response includes the decision motivation.
    #[returns(DecideResponse)]
    Decide { case: String, motivated: bool },
}

/// Response returned by `QueryMsg::Constitution`.
#[cosmwasm_schema::cw_serde]
pub struct ConstitutionResponse {
    /// Stored Prolog governance program.
    pub governance: Binary,
}

/// Response returned by `QueryMsg::Decide`.
#[cosmwasm_schema::cw_serde]
pub struct DecideResponse {
    /// The decision verdict as a Prolog term string.
    pub verdict: String,
    /// The decision motivation (if requested) as a Prolog term string.
    pub motivation: Option<String>,
}
