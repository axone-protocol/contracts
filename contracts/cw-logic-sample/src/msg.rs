use cosmwasm_schema::{cw_serde, QueryResponses};
use logic_bindings::AskResponse;

/// Instantiate messages
#[cw_serde]
pub struct InstantiateMsg {
    pub program: String,
}

/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// # Ask
    /// Ask returns the evaluation of the query using the program context through the logic module.
    #[returns(AskResponse)]
    Ask { query: String },
}
