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
    /// Update the contract configuration.
    ///
    /// This message is reserved for future configuration fields.
    UpdateConfig {},
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
    /// Return the current configuration.
    #[returns(ConfigResponse)]
    Config {},

    /// Return the stored governance constitution program.
    #[returns(ConstitutionResponse)]
    Constitution {},
}

/// Response returned by `QueryMsg::Config`.
#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}

/// Response returned by `QueryMsg::Constitution`.
#[cosmwasm_schema::cw_serde]
pub struct ConstitutionResponse {
    /// Stored Prolog governance program.
    pub governance: Binary,
}
