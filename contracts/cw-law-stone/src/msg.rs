use cosmwasm_schema::{cw_serde, QueryResponses};

/// Instantiate messages
#[cw_serde]
pub struct InstantiateMsg {}

/// Execute messages
#[cw_serde]
pub enum ExecuteMsg {}

/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
