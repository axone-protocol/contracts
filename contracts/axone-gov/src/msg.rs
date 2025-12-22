use crate::contract::AxoneGov;

use cosmwasm_schema::QueryResponses;

// This is used for type safety and re-exporting the contract endpoint structs.
abstract_app::app_msg_types!(AxoneGov, AxoneGovExecuteMsg, AxoneGovQueryMsg);

/// App instantiate message
#[cosmwasm_schema::cw_serde]
#[derive(Default)]
pub struct AxoneGovInstantiateMsg {
    #[serde(default)]
    pub count: i32,
}

/// App execute messages
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum AxoneGovExecuteMsg {
    UpdateConfig {},
    /// Increment count by 1
    Increment {},
    /// Admin method - reset count
    Reset {
        /// Count value after reset
        count: i32,
    },
}

#[cosmwasm_schema::cw_serde]
pub struct AxoneGovMigrateMsg {}

/// App query messages
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum AxoneGovQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(CountResponse)]
    Count {},
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}

#[cosmwasm_schema::cw_serde]
pub struct CountResponse {
    pub count: i32,
}
