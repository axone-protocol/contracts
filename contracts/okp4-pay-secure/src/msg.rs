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

/// `PreAuthorizationId` defines the type for a pre-authorization identifier.
pub type PreAuthorizationId = String;

/// `Cursor` defines the opaque type for a cursor.
/// It is used to paginate through a list of items.
pub type Cursor = String;

/// # PageInfo
/// PageInfo is the page information returned for paginated queries.
#[cw_serde]
pub struct PageInfo {
    /// Tells if there is a next page.
    pub has_next_page: bool,
    /// The cursor to the next page.
    pub cursor: Cursor,
}

/// The set of possible actions that can be performed on the Smart Contract account.
#[cw_serde]
pub enum ExecuteMsg {
    /// # DepositFunds
    /// Initiates the deposit action on the Holder smart contract account. If successful, the
    /// specified amount of tokens are transferred from the sender's external wallet
    /// to the Holder's smart contract instance.
    ///
    /// **Actor**: Sender
    ///
    /// **Preconditions**:
    ///
    /// - The Sender must have sufficient funds to cover the deposit.
    DepositFunds {},
    /// # WithdrawFunds
    ///
    /// Initiates the withdraw action on the smart contract account allowing the Holder to retrieve
    /// their available funds and transfer them back to their external wallet.
    ///
    /// **Actor**: Sender
    ///
    /// **Preconditions**:
    ///
    /// - The sender of the message must be the Holder.
    ///
    /// - The amount must be greater than the amount of available funds, i.e. the amount of funds that are not currently locked in a pre-authorization.
    WithdrawFunds {
        /// The amount of tokens to withdraw.
        amount: Vec<Coin>,
        /// The recipient of the withdrawn funds.
        ///
        /// If not provided, the funds will be sent to the Holder's external wallet.
        to: Option<String>,
    },
    /// # CloseAccount
    ///
    /// Initiate the process of closing the Holder's account. Once the account is closed, no further
    /// operations can be performed on it. This action is irreversible.
    ///
    /// ***Actor:*** Holder
    ///
    /// ***Preconditions:***
    ///
    /// - The account balance must be zero.
    CloseAccount {},
    /// # InitiatePreAuthorization
    /// Start the initial step taken by a Provider to request a pre-authorization of funds from a
    /// client. By initiating this, the provider signals the intent to reserve a specified amount of
    /// the client's funds in the smart contract as a guarantee for a future transaction.
    ///
    /// ***Actor:*** Provider
    InitiatePreAuthorization {
        /// The amount of tokens to lock in the pre-authorization.
        amount: Vec<Coin>,
        /// The expiration of the pre-authorization, expressed as a block height or a block time.
        expiration: Expiration,
        /// Account to which the funds will be transferred upon finalization of the pre-authorization.
        /// This field allows for the specification of an intermediary account, like an escrow service, to
        /// temporarily hold the funds until all parties reach a mutual agreement on the transaction.
        ///
        /// If not provided, the funds will be transferred to the Provider's account.
        #[serde(default)]
        destination_account: Option<String>,
    },
    /// # ApprovePreAuthorization
    ///
    /// Approve the pre-authorization request initiated by a provider.
    /// Upon approval, the specified amount of tokens are locked and reserved in the smart contract for a maximum duration specified by the expiration.
    ///
    /// ***Actor:*** Client
    ///
    /// ***Preconditions:***
    ///
    /// - The status of the pre-authorization request is `Pending`.
    ///
    /// - The client has sufficient available funds (i.e., funds that aren't tied to any other active pre-authorizations) in their smart contract account to cover the requested pre-authorization amount.
    ApprovePreAuthorization {
        /// The unique identifier of the pre-authorization request to approve.
        id: PreAuthorizationId,
    },
    /// # DeclinePreAuthorization
    /// Decline a pre-authorization request initiated by a provider.
    /// By declining, the client signals that they do not agree to lock any funds for the future transaction.
    ///
    /// ***Actor:*** Client
    ///
    /// ***Preconditions:***
    ///
    /// - The sender of the message must be the Client.
    ///
    /// - The client has received a pre-authorization request from a provider.
    DeclinePreAuthorization {
        /// The unique identifier of the pre-authorization request to decline.
        id: PreAuthorizationId,
    },
    /// # CancelPreAuthorization
    /// Allows the provider to cancel a pre-authorization request that they have initiated.
    /// By canceling, the provider signals that they no longer wish to proceed with the transaction.
    /// Any funds that were locked for this specific pre-authorization are unlocked, and the transaction process ends.
    ///
    /// ***Actor:*** Provider
    ///
    /// ***Preconditions:***
    ///
    /// - A pre-authorization request has been initiated by the provider and is in the `Pending` or `Approved` state.
    CancelPreAuthorization {
        /// The unique identifier of the pre-authorization request to cancel.
        id: PreAuthorizationId,
        /// The reason for canceling the pre-authorization request.
        /// This field is optional and can be used to provide additional context to the client.
        reason: Option<String>,
    },
    /// # FinalizePreAuthorization
    ///
    /// Finalizes a pre-authorization request by transferring the locked funds from the client's account to the account specified in the
    /// pre-authorization request. The amount transferred could be all or a portion of the initially locked funds, depending on the final cost of the service.
    /// Any remaining locked funds are unlocked and returned to the client's account, and the transaction process ends.
    ///
    /// This action is taken by the provider after the service has been successfully delivered.
    ///
    /// ***Actor:*** Provider
    ///
    /// ***Preconditions:***
    ///
    /// - The pre-authorization request must be in the `Approved` state.
    ///
    /// - The final amount must be less than or equal to the initially locked amount.
    FinalizePreAuthorization {
        /// The unique identifier of the pre-authorization request to be finalized.
        id: String,
        /// The final amount to be transferred from the client's locked funds to the provider's account.
        /// This amount must be less than or equal to the initially locked amount.
        final_amount: Vec<Coin>,
    },
}
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
