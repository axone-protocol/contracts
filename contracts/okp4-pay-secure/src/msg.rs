use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;
use cw_utils::Expiration;

/// Message to initialize a new instance of the contract, which represents a new Account,
/// owned by the Holder, which is the sender of the message.
#[cw_serde]
pub struct InstantiateMsg {
    /// The name of the account. It can be used to provide a human-readable name for the account.
    /// This is an optional field. If not provided, the account will be left unnamed.
    pub name: Option<String>,
    /// The list of denominations that are accepted by the account.
    /// If not provided, the account will accept all denominations.
    pub accepted_denoms: Option<Vec<String>>,
}


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
