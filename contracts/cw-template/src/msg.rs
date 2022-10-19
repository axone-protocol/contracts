use cosmwasm_schema::{cw_serde, QueryResponses};

/// Instantiate messages
#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

/// Execute messages
#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
}

/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// # GetCount
    /// GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

/// # GetCountResponse
/// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}
