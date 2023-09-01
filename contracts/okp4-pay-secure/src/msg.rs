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
    /// Specifies the limits configured for the account.
    #[serde(default)]
    pub account_limits: AccountLimitsConfig,
    /// Specifies the limits configured for the pre-authorization.
    #[serde(default)]
    pub pre_authorization_limits: PreAuthorizationLimitsConfig,
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
    /// Initiate the process of closing the Holder's account. Once closed, the account becomes inoperable,
    /// and no further transactions or operations can be conducted. This action is irreversible.
    ///
    /// The account balance must be zero for the closure to proceed. If there are pending pre-authorization
    /// requests, the account can still be closed. This ensures that the Holder's intent to close the account
    /// is not hindered by any pending transactions. If any pre-authorization requests have received approval,
    /// they must be finalized and settled before the Holder can proceed to empty the account and complete the
    /// closure process.
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
    /// Upon initiation, the pre-authorization request is assigned a unique identifier that can be
    /// used to track the status of the request.
    ///
    /// The pre-authorization request comes with two distinct expiration timelines. The 'approval expiration'
    /// sets the time limit within which the client must approve the pre-authorization request. On the other hand,
    /// the 'locking expiration' defines the maximum time period that the funds will remain locked in the smart
    /// contract once the client has given approval.
    ///
    /// ``` plantuml
    ///
    /// @startuml
    ///
    /// hide time-axis
    ///
    /// scale 10 as 150 pixels
    ///
    /// concise "state" as ST
    ///
    /// ST is ""
    ///
    /// @ST
    ///
    /// 0 is pending
    ///
    /// +10 is approved
    ///
    /// +15 is finalized
    ///
    /// highlight 0 to 12 #line:DimGrey : \napproval expiration
    ///
    /// highlight 10 to 30 #Gold;line:Gold : \nlocking expiration
    ///
    /// @enduml
    ///
    /// ```
    ///
    /// ***Actor:*** Provider
    InitiatePreAuthorization {
        /// The amount of tokens to lock in the pre-authorization.
        amount: Vec<Coin>,
        /// The expiration of the waiting period for client approval, expressed as a block height or a block time.
        approval_expiration: Expiration,
        /// The expiration of the locking period, starting from the time of client approval, expressed as a block height or a block time.
        locking_expiration: Expiration,
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

/// Defines the set of possible permissions that can be granted to a provider.
#[cw_serde]
#[derive(Default)]
pub struct WhitelistBlacklistAddress {
    /// A list of whitelisted addresses.
    /// If not provided, any address is allowed.
    #[serde(default)]
    pub whitelisted_addresses: Option<Vec<String>>,
    /// A list of blacklisted addresses. Blacklisted addresses always take precedence over whitelisted addresses.
    /// If not provided, no addresses are blacklisted.
    #[serde(default)]
    pub blacklisted_addresses: Option<Vec<String>>,
}

/// Represents the possible limits that can be configured for an account.
#[cw_serde]
#[derive(Default)]
pub struct AccountLimitsConfig {
    /// Specifies the list of token denominations that the account will accept.
    /// If not provided, the account will accept all known token denominations.
    #[serde(default)]
    pub accepted_denoms: Option<Vec<String>>,
    /// Specifies the addresses that are permitted to deposit funds into the account.
    /// If not provided, deposits from any address will be accepted.
    #[serde(default)]
    pub allowed_deposit_senders: WhitelistBlacklistAddress,
    /// Specifies the addresses that are permitted to receive withdrawals from the account.
    /// If not provided, withdrawals to any address will be allowed.
    #[serde(default)]
    pub allowed_withdraw_recipients: WhitelistBlacklistAddress,
}

/// Represents the possible limits that can be configured for pre-authorization requests.
#[cw_serde]
#[derive(Default)]
pub struct PreAuthorizationLimitsConfig {
    /// Specifies the providers that are permitted to initiate a pre-authorization request for the account.
    /// If not provided, any provider can initiate a pre-authorization.
    #[serde(default)]
    pub allowed_providers: WhitelistBlacklistAddress,
    /// Specifies the maximum duration for which a pre-authorization can be active.
    #[serde(default)]
    pub max_pre_authorization_lifetime: Option<Expiration>,
    /// Specifies the maximum duration that can be set for the approval phase of a pre-authorization.
    #[serde(default)]
    pub max_approval_expiration: Option<Expiration>,
    /// Specifies the maximum duration that can be set for the locking phase of a pre-authorization.
    #[serde(default)]
    pub max_locking_expiration: Option<Expiration>,
}

/// Represents the filters that can be applied when querying pre-authorization requests.
#[cw_serde]
pub enum WhereFilter {
    /// # EqStatus
    /// Filter by the status of the pre-authorization.
    EqStatus(PreAuthorizationStatus),
    /// # EqProvider
    /// Filter by the provider ID.
    EqProvider(String),
}

/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// # Account
    /// Query the details of the account.
    #[returns(AccountResponse)]
    Account {},

    /// # Balance
    /// Query the balance of the account.
    #[returns(BalanceResponse)]
    Balance {},

    /// # Limits
    /// Query the limits of the account.
    #[returns(LimitsResponse)]
    Limits {},

    /// # PreAuthorization
    /// Query the details of a pre-authorization request.
    #[returns(PreAuthorizationResponse)]
    PreAuthorization {
        /// The unique identifier of the pre-authorization request to query.
        id: PreAuthorizationId,
    },

    /// # PreAuthorizations
    /// Query all pre-authorization requests initiated by a provider according to the specified filters.
    #[returns(PreAuthorizationsResponse)]
    PreAuthorizations {
        /// The maximum number of pre-authorization requests to return.
        first: Option<u32>,
        /// The cursor from which to start returning pre-authorization requests.
        after: Option<Cursor>,
        /// The filters to apply to the pre-authorization requests.
        r#where: Option<WhereFilter>,
    },
}

/// Represents the current phase during the pre-authorization process.
///
/// ``` plantuml
///
/// @startuml
///
/// state Declined
///
/// state Expired
///
/// state Finalized #line.bold
///
/// state Cancelled {
///
///   state PreApprovalCancelled
///
///   state PostApprovalCancelled
///
/// }
///
/// state Expired {
///
///  state PreApprovalExpired
///
///  state PostApprovalExpired
///
/// }
///
/// state Approved #line.bold
///
/// state Pending #line.bold
///
/// [*] -[bold]-> Pending : (Provider) Initiate
///
/// Pending --> Pending : [insufficient funds]\n(Client) Approve
///
/// Pending -[bold]-> Approved : [sufficient funds]\n(Client) Approve
///
/// Pending --> PreApprovalCancelled : (Provider) Cancel
///
/// Pending --> Declined : (Client) Decline
///
/// Pending --> PreApprovalExpired : (System) Expire
///
/// Approved -[bold]-> Finalized : (Provider) Finalize
///
/// Approved --> PostApprovalCancelled : (Provider) Cancel
///
/// Approved --> PostApprovalExpired : (System) Expire
///
/// Declined --> [*]
///
/// Cancelled --> [*]
///
/// PreApprovalExpired --> [*]
///
/// PostApprovalExpired --> [*]
///
/// Finalized -[bold]-> [*]
///
/// @enduml
///
/// ```
#[cw_serde]
#[derive(Eq, PartialOrd)]
pub enum PreAuthorizationStatus {
    /// # Pending
    /// Represents the period after the provider has initiated a pre-authorization request and is waiting for the client's approval.
    Pending,
    /// # Approved
    /// The client has approved the pre-authorization request, and the funds are now locked in and reserved for the transaction. The provider can now deliver the service, and upon completion, finalize the transaction.
    Approved,
    /// # Declined
    /// The client has declined the pre-authorization request. No funds are locked or transferred, and the transaction process ends.
    Declined,
    /// # Cancelled
    /// The provider has chosen to cancel the pre-authorization request. This could be due to various reasons,
    /// such as service unavailability or a change in terms. Any locked funds are unlocked for the client, and the transaction process ends.
    Cancelled {
        /// Indicates whether this occurs Pre or Post approval.
        phase: PreAuthorizationPhase,
        /// The reason for the cancellation (if any).
        reason: Option<String>,
    },
    /// # Expired
    /// The pre-authorization request has not been finalized within a specified timeframe and has thus expired. Any locked funds are unlocked for the client, and the transaction process ends.
    Expired {
        /// The phase indicates whether this occurs Pre or Post approval.
        phase: PreAuthorizationPhase,
    },
    /// # Finalized
    /// Upon successful provision of the service, the provider finalizes the transaction. The appropriate amount, which could be all or a portion of the locked funds, is transferred to the provider as payment. Any remaining funds are unlocked for the client, concluding the transaction process.
    Finalized,
}

/// Represents the phase of the pre-authorization process.
#[cw_serde]
#[derive(Copy, Eq, PartialOrd)]
pub enum PreAuthorizationPhase {
    /// # PreApproval
    /// Occurs before the client has given their approval.
    PreApproval,
    /// # PostApproval
    /// Occurs after the client has given their approval (but before finalization).
    PostApproval,
}

/// Represents the status of the Holder's account.
///
/// ``` plantuml
///
/// @startuml
///
/// [*] -[bold]-> Open : (Holder) Create account\n(Smart Contract instantiation)
///
/// Open --> Open : (*) *
///
/// Open --> Closed : [balance is 0]\n(Holder) Close
///
/// Closed --> [*]
///
/// @enduml
///
/// ```
#[cw_serde]
#[derive(Copy, Eq, PartialOrd)]
pub enum AccountStatus {
    /// # Open
    /// The account is open, and all operations can be performed on the Holder's account.
    Open,

    /// # Closed
    /// The account is closed, and no operations can be performed on the Holder's account.
    /// Once in this state, the account becomes inoperable, and no further transactions or operations can be executed on the Holder's account.
    Closed,
}

/// Represents an account response.
#[cw_serde]
pub struct AccountResponse {
    /// The name of the account.
    pub name: Option<String>,
    /// The accepted denominations for this account.
    pub accepted_denoms: Option<Vec<String>>,
    /// The current status of the account.
    pub status: AccountStatus,
}

/// Represents the limits response.
#[cw_serde]
#[derive(Default)]
pub struct LimitsResponse {
    /// The limits specifically configured for the account.
    pub account_limits: AccountLimitsConfig,
    /// The limits specifically configured for pre-authorization requests.
    pub pre_authorization_limits: PreAuthorizationLimitsConfig,
}

/// Represents a balance response.
#[cw_serde]
pub struct BalanceResponse {
    /// The current balance of the account.
    /// This is the total amount of funds that are currently in the account.
    pub balance: Vec<Coin>,
    /// The current available balance of the account.
    /// This is the amount of funds that are currently available for use.
    pub available_balance: Vec<Coin>,
    /// The current locked balance of the account.
    /// This is the amount of funds that are currently locked in pre-authorization requests.
    pub locked_balance: Vec<Coin>,
}

/// Represents a pre-authorization response.
#[cw_serde]
pub struct PreAuthorizationResponse {
    /// The unique identifier of the pre-authorization request.
    pub id: PreAuthorizationId,
    /// The provider's identity.
    pub provider: String,
    /// The amount to be locked from the client's account.
    pub amount: Vec<Coin>,
    /// The expiration of the waiting period for client approval, expressed as a block height or a block time.
    pub approval_expiration: Expiration,
    /// The expiration of the locking period, starting from the time of client approval, expressed as a block height or a block time.
    pub locking_expiration: Expiration,
    /// Account to which the funds will be transferred upon finalization of the pre-authorization.
    pub destination_account: Option<String>,
    /// The current status of the pre-authorization request.
    pub status: PreAuthorizationStatus,
}

/// Represents a pre-authorization set response.
#[cw_serde]
pub struct PreAuthorizationsResponse {
    /// The pre-authorization requests.
    pub data: Vec<PreAuthorizationResponse>,
    /// The page information.
    pub page_info: PageInfo,
}
