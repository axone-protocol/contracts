use crate::{contract::AxoneGov, state::ConstitutionStatus};

use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Binary;

abstract_app::app_msg_types!(AxoneGov, AxoneGovExecuteMsg, AxoneGovQueryMsg);

/// Instantiate message.
///
/// This contract stores a governance constitution as a Prolog program on the resource AA.
/// The constitution defines governance rules using Prolog predicates.
/// The `constitution` field must contain a UTF-8 encoded Prolog program.
///
/// During instantiation, the contract validates that the constitution defines the required predicates:
/// - `decide/2` which takes a `Case` argument and returns a verdict term.
/// - `decide/3` which takes a `Case` argument and returns both a verdict and a motivation term.
///
/// The `decide/2` predicate returns a verdict Prolog term indicating the decision outcome.
/// The `decide/3` predicate returns both a verdict and a motivation term providing reasoning for the decision.
#[cosmwasm_schema::cw_serde]
#[derive(Default)]
pub struct AxoneGovInstantiateMsg {
    /// Prolog governance program defining the constitution.
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
    /// Return the stored governance constitution program bytes.
    #[returns(ConstitutionResponse)]
    Constitution {},

    /// Return the stored constitution status metadata.
    #[returns(ConstitutionStatusResponse)]
    ConstitutionStatus {},

    /// Decide a case using the constitution's `decide/2` or `decide/3` predicate.
    ///
    /// The `case` parameter is a Prolog dict term string that represents the decision context.
    /// This string is passed as the `Case` argument to the `decide` predicate.
    ///
    /// Example of a case dict:
    /// `ctx{action:read, user:"did:example:123", object:"obj:42"}`
    ///
    /// The `verdict` returned is an arbitrary Prolog term (which can be an atom or a compound term,
    /// e.g., `permitted` or `pay(user_1)`), representing the decision outcome.
    ///
    /// If `motivated` is true, the contract calls `decide/3` and returns both `verdict` and `motivation`.
    /// The `motivation` is a Prolog term that provides reasoning behind the decision.
    #[returns(DecideResponse)]
    Decide { case: String, motivated: bool },
}

/// Response returned by `QueryMsg::Constitution`.
#[cosmwasm_schema::cw_serde]
pub struct ConstitutionResponse {
    /// The stored Prolog governance constitution program bytes.
    pub governance: Binary,
}

/// Response returned by `QueryMsg::ConstitutionStatus`.
#[cosmwasm_schema::cw_serde]
pub struct ConstitutionStatusResponse {
    /// The stored constitution revision.
    pub constitution_revision: u64,
    /// The stored constitution hash (32 bytes).
    pub constitution_hash: Binary,
}

impl From<&ConstitutionStatus> for ConstitutionStatusResponse {
    fn from(status: &ConstitutionStatus) -> Self {
        Self {
            constitution_revision: status.constitution_revision,
            constitution_hash: Binary::from(status.constitution_hash),
        }
    }
}

/// Response returned by `QueryMsg::Decide`.
#[cosmwasm_schema::cw_serde]
pub struct DecideResponse {
    /// The decision verdict as a Prolog term string.
    pub verdict: String,
    /// Optional motivation term returned as the third argument by `decide/3`.
    pub motivation: Option<String>,
}
