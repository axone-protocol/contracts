use crate::contract::AxoneGov;

use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Binary;

abstract_app::app_msg_types!(AxoneGov, AxoneGovExecuteMsg, AxoneGovQueryMsg);

/// App instantiate message
#[cosmwasm_schema::cw_serde]
#[derive(Default)]
pub struct AxoneGovInstantiateMsg {
    pub constitution: Binary,
}

/// App execute messages
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum AxoneGovExecuteMsg {
    UpdateConfig {},
}

#[cosmwasm_schema::cw_serde]
pub struct AxoneGovMigrateMsg {}

/// App query messages
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum AxoneGovQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(ConstitutionResponse)]
    Constitution {},
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}

#[cosmwasm_schema::cw_serde]
pub struct ConstitutionResponse {
    pub governance: Binary,
}
