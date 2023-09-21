use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;

/// InstantiateMsg is the default config used for the tax distribution contract.
///
/// This contract is intended to be stored with `--instantiate-nobody ` args to prevent only the
/// governance that is allowed to configure and instantiate this contract and be set as the contract
/// admin.
#[cw_serde]
pub struct InstantiateMsg {
    /// Configure the destination of the remaining tokens after distribution to other recipients
    /// (defined in `destinations`).
    default_recipient: Recipient,
    /// Define the distribution rate of tokens to the intended recipients.
    /// The total rate sum should not exceed 1.
    destinations: Vec<Destination>,
}

#[cw_serde]
pub enum Recipient {
    /// Send token to the community pool.
    CommunityPool,
    /// Burn token.
    Burn,
    /// Send token to a specific wallet address.
    Address(String),
}

#[cw_serde]
pub struct Destination {
    /// Recipient of token
    recipient: Recipient,
    /// Set the token rate to receive for this recipient.
    /// Value should be between zero exclusive and one exclusive.
    ratio: Decimal,
}

/// Execute messages
#[cw_serde]
pub enum ExecuteMsg {
    /// # Distribute
    /// Distributes the tokens received from the transaction to the recipients following
    /// the configured apportionment.
    Distribute,

    /// # UpdateDefaultRecipient
    /// Change the default recipient used for distribute remaining token.
    ///
    /// Only contract admin can update the default recipient.
    UpdateDefaultRecipient { recipient: Recipient },

    /// # AddDestination
    /// Add a new recipient for receiving token with it's corresponding ratio.
    /// Don't forget that the total ratio already configured shouldn't exceed 1.
    AddDestination { destination: Destination },

    /// # RemoveRecipient
    /// Remove a recipient from the tax distribution.
    RemoveRecipient { recipient: Recipient },
}

/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// # Destinations
    /// Returns the current configuration for all tax distribution destinations with their
    /// corresponding ratio.
    #[returns(DestinationsResponse)]
    Destinations,
}

/// # DestinationsResponse
#[cw_serde]
pub struct DestinationsResponse {
    /// The current configured default recipient for remaining token after distribution.
    pub default_recipient: Recipient,
    /// All recipients with their corresponding token ratio.
    pub destinations: Vec<Destination>,
}
