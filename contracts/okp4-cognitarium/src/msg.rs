use cosmwasm_schema::{cw_serde, QueryResponses};

/// Instantiate message
#[cw_serde]
pub struct InstantiateMsg {
    /// Limitations regarding store usage.
    pub limits: StoreLimits,
}

/// Execute messages
#[cw_serde]
pub enum ExecuteMsg {}

/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

/// # StoreLimits
/// Contains limitations regarding store usages.
#[cw_serde]
pub struct StoreLimits {
    /// max_triple_count denotes the maximum number of triples the store can contains.
    /// If None, there is no limit on the number of triples.
    pub max_triple_count: Option<Uint128>,
}
