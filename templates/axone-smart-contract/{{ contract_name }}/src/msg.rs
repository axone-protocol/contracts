use cosmwasm_schema::{cw_serde, QueryResponses};

/// Instantiate message
#[cw_serde]
pub struct InstantiateMsg {}

/// Execute messages
#[cw_serde]
pub enum ExecuteMsg {
    /// # Foo
    Foo,
}

/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// # Bar
    #[returns(BarResponse)]
    Bar { foo: String },
}

/// # BarResponse
#[cw_serde]
pub struct BarResponse {
    /// The foo value
    pub foo: String,
}
