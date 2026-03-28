use crate::contract::AxoneVc;

use cosmwasm_schema::QueryResponses;

abstract_app::app_msg_types!(AxoneVc, AxoneVcExecuteMsg, AxoneVcQueryMsg);

#[cosmwasm_schema::cw_serde]
#[derive(Default)]
pub struct AxoneVcInstantiateMsg {}

#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum AxoneVcExecuteMsg {
    Foo { value: String },
}

#[cosmwasm_schema::cw_serde]
pub struct AxoneVcMigrateMsg {}

#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum AxoneVcQueryMsg {
    #[returns(FooResponse)]
    Foo {},
}

#[cosmwasm_schema::cw_serde]
pub struct FooResponse {
    pub value: String,
}
