use cosmwasm_schema::{cw_serde, QueryResponses};
#[allow(unused_imports)]
use okp4_logic_bindings::AskResponse;

/// Instantiate message
#[cw_serde]
pub struct InstantiateMsg {
    /// A name to give to the dataverse instantiated.
    pub name: String,
}

/// Execute messages
#[cw_serde]
pub enum ExecuteMsg {
    Foo,
}

/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(FooResponse)]
    Foo,
}

/// # QueryResponses
#[cw_serde]
pub struct FooResponse {}
