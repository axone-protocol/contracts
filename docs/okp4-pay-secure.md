# okp4-pay-secure

A [CosmWasm](https://cosmwasm.com/) smart contract designed to facilitate secure, pre-authorized token transactions between clients and providers, akin to credit card pre-authorizations in traditional finance systems.

## Purpose

The primary objective of this smart contract is to provide a robust pre-authorization framework for token-based transactions. It allows clients to earmark a specific amount of tokens in favor of a provider without actually transferring them, thereby confirming the client's ability to cover the transaction cost.

## Rationale

In scenarios where a provider offers services on a deferred payment basis, there's a need for a mechanism that not only verifies but also ensures that the client has adequate funds to fulfill the payment obligation. This smart contract addresses this by "reserving" a specified amount of tokens in the client's account. These reserved funds are deducted from the available balance but remain in the client's account until the provider initiates the final settlement. At that point, the reserved tokens are transferred to the designated recipient, which could be the provider's account or an optional intermediary account, such as an escrow service.

## Terminology

### Funds

Tokens allocated in the Smart Contract associated with a Holder's profile.

### Locked Funds

A specific portion of funds retained within the smart contract during the pre-authorization phase initiated by the Provider. These funds act as a commitment from the Client for the execution of a service and remain inaccessible until the pre-authorization concludes, is revoked or expires.

### Available Funds

Funds present in the Holder's smart contract account that aren't tied to any pre-authorization. The Contract Holder has full autonomy over these funds, allowing them to utilize, transfer, or retrieve them as they see fit.

### Pre-authorization

A procedure set in motion by the Provider to confirm the financial capability of the Client's account. This entails momentarily reserving a defined sum of funds as an assurance for service payment. These reserved funds stay in this condition until the service transaction finalizes, the pre-authorization concludes, is revoked or expires.

### Capture

A procedure activated by the Provider to complete the service transaction. This involves instructing the payment processor to move the funds that were previously pre-authorized (reserved) in the Client's account. The transfer can be directed to the Provider's own account or to an escrow account, which adds an additional layer of security to the transaction and allows for dispute resolution.

## Overview

### Stakeholders

``` plantuml
@startuml

left to right direction

:Sender:
:Reciever:
:Holder:
:Client:
:Provider:

:Holder: <|- :Client:

rectangle "\nokp4-pay-secure" <<smart contract>> as System

Sender "*" -- System
System -- "*" Reciever
Holder "1" --     System
Client "1" --     System
System     -- "*" Provider

@enduml
```

#### Sender

The entity that initiates transactions to transfer funds to the Smart Contract.

#### Receiver

The entity that receives funds from the Smart Contract.

#### Holder

The entity that possesses the Smart Contract and controls the funds stored within it.

#### Client

The Holder who aims to acquire a service, securing a specific token quantity as a pre-approval for the Provider.

#### Provider

An entity that offers a service and seeks a token pre-authorization before delivering the said service.

### Features

The smart contract offers two distinct categories of features.

#### [Smart Contract Account (SCA)](https://blog.ambire.com/eoas-vs-smart-contract-accounts/) Capabilities

The smart contract is under the ownership and control of the holder. Similar to an Externally Owned Account (EOA), the holder has the ability to deposit, withdraw, and transfer funds to and from this account.

``` plantuml
@startuml

left to right direction

:Sender:
:Holder: as Holder1
:Holder: as Holder2
:Reciever:


:Sender: <|- :Holder1:
:Reciever: <|- :Holder2:

rectangle "\nokp4-pay-secure" <<smart contract>> as System {
  usecase "Deposit" as UC1
  usecase "Withdraw" as UC2
  usecase "CloseAccount" as UC3
}

Sender -- UC1
UC1 -- Reciever
Holder1 -- UC2
UC2 -- Reciever
Holder1 -- UC3

@enduml
```

#### Pre-authorization Capabilities

A provider has the option to pre-authorize a specific amount of tokens along with designating a recipient account for a predetermined duration, after which the pre-authorization will expire. This designated recipient account can serve as an intermediary for holding the pre-authorized funds, such as an escrow service.

``` plantuml
@startuml

left to right direction

:Client:
:Provider:
:Sender:
:Escrow:

:Sender: <|- :Escrow:
:Holder: <|- :Client:

rectangle "\nokp4-pay-secure" <<smart contract>> as System {
  usecase "Initiate" as UC1
  usecase "Approve" as UC2
  usecase "Decline" as UC3
  usecase "Cancel" as UC4
  usecase "Finalize" as UC5
}

Provider -- UC1
Client -- UC2
Client -- UC3
Provider -- UC4
Provider -- UC5
UC5 -- Sender

@enduml
```

### Scenarios

#### Pre-authorization & Capture to Provider

In this scenarios, the Client places an order with the Provider, who then initiates a pre-authorization request through the `okp4-pay-secure`` smart contract. The Client approves this request, allowing the Provider to finalize the transaction and capture the portion of the funds corresponding to the actual cost of the service.

1. *Order*: The Client places an order with the Provider.
2. *Initiate*: The Provider initiates a pre-authorization request in the smart contract.
3. *Approve*: The Client approves the pre-authorization, locking the funds.
4. *Finalize and Partial Capture*: The Provider finalizes the transaction and captures only the portion of the locked funds that corresponds to the actual cost.

``` plantuml
@startuml

left to right direction

:Client:
:Provider:
:Reciever:
:Reciever: <|- :Provider:
rectangle "\nokp4-pay-secure" <<smart contract>> as System

Client -.-> Provider : "1. Order"

System <-- Provider  : "2. Initiate"

Client --> System  : "3. Approve"

System <-- Provider : "4.1 Finalize"

System -.-> Provider : "4.2 Capture" 

@enduml
```

#### Pre-authorization & Capture to Escrow

In this scenario, the process is similar to the first scenario, but with an added layer of security: an `escrow` service. After the Client approves the pre-authorization, the Provider can finalize the transaction. However, instead of capturing the funds directly, only the portion corresponding to the actual cost is sent to an `escrow` smart contract. This allows for a third-party Arbitrator to resolve any disputes.

- *Order*: The Client places an order with the Provider.
- *Initiate*: The Provider initiates a pre-authorization request in the smart contract.
- *Approve*: The Client approves the pre-authorization, locking the funds.
- *Finalize and Partial Capture to Escrow*: The Provider finalizes the transaction, but only the portion of the funds corresponding to the actual cost is captured to an `escrow`` smart contract. Both the Client and Provider, as well as an Arbitrator, have access to this escrow.

``` plantuml
@startuml

left to right direction

:Client:
:Provider:
:Arbitrator:

rectangle "\nokp4-pay-secure" <<smart contract>> as System
rectangle "\nescrow" <<smart contract>> as Escrow

Client -.-> Provider : "1. Order"

System <-- Provider  : "2. Initiate"

Client --> System  : "3. Approve"

System <-- Provider : "4.1 Finalize"

System -.-> Escrow : "4.2 Capture" 

Client -- Escrow
Provider -- Escrow
Arbitrator -- Escrow

@enduml
```

## InstantiateMsg

Message to initialize a new instance of the contract, which represents a new Account, owned by the Holder, which is the sender of the message.

|parameter|description|
|----------|-----------|
|`account_limits`|**[AccountLimitsConfig](#accountlimitsconfig)**. Specifies the limits configured for the account.|
|`account_limits.accepted_denoms`|**array\|null**. Specifies the list of token denominations that the account will accept. If not provided, the account will accept all known token denominations.|
|`account_limits.allowed_deposit_senders`|**[WhitelistBlacklistAddress](#whitelistblacklistaddress)**. Specifies the addresses that are permitted to deposit funds into the account. If not provided, deposits from any address will be accepted.<br />**Default:** `{"blacklisted_addresses":null,"whitelisted_addresses":null}`|
|`account_limits.allowed_withdraw_recipients`|**[WhitelistBlacklistAddress](#whitelistblacklistaddress)**. Specifies the addresses that are permitted to receive withdrawals from the account. If not provided, withdrawals to any address will be allowed.<br />**Default:** `{"blacklisted_addresses":null,"whitelisted_addresses":null}`|
|`name`|**string\|null**. The name of the account. It can be used to provide a human-readable name for the account. This is an optional field. If not provided, the account will be left unnamed.|
|`pre_authorization_limits`|**[PreAuthorizationLimitsConfig](#preauthorizationlimitsconfig)**. Specifies the limits configured for the pre-authorization.|
|`pre_authorization_limits.allowed_providers`|**[WhitelistBlacklistAddress](#whitelistblacklistaddress)**. Specifies the providers that are permitted to initiate a pre-authorization request for the account. If not provided, any provider can initiate a pre-authorization.<br />**Default:** `{"blacklisted_addresses":null,"whitelisted_addresses":null}`|
|`pre_authorization_limits.max_approval_expiration`|**[Expiration](#expiration)\|null**. Specifies the maximum duration that can be set for the approval phase of a pre-authorization.|
|`pre_authorization_limits.max_locking_expiration`|**[Expiration](#expiration)\|null**. Specifies the maximum duration that can be set for the locking phase of a pre-authorization.|
|`pre_authorization_limits.max_pre_authorization_lifetime`|**[Expiration](#expiration)\|null**. Specifies the maximum duration for which a pre-authorization can be active.|

## ExecuteMsg

The set of possible actions that can be performed on the Smart Contract account.

### ExecuteMsg::DepositFunds

Initiates the deposit action on the Holder smart contract account. If successful, the specified amount of tokens are transferred from the sender's external wallet to the Holder's smart contract instance.

**Actor**: Sender

**Preconditions**:

- The Sender must have sufficient funds to cover the deposit.

|parameter|description|
|----------|-----------|
|`deposit_funds`|*(Required.) * **object**. |

### ExecuteMsg::WithdrawFunds

Initiates the withdraw action on the smart contract account allowing the Holder to retrieve their available funds and transfer them back to their external wallet.

**Actor**: Sender

**Preconditions**:

- The sender of the message must be the Holder.

- The amount must be greater than the amount of available funds, i.e. the amount of funds that are not currently locked in a pre-authorization.

|parameter|description|
|----------|-----------|
|`withdraw_funds`|*(Required.) * **object**. |
|`withdraw_funds.amount`|*(Required.) * **Array&lt;[Coin](#coin)&gt;**. The amount of tokens to withdraw.|
|`withdraw_funds.to`|**string\|null**. The recipient of the withdrawn funds.<br /><br />If not provided, the funds will be sent to the Holder's external wallet.|

### ExecuteMsg::CloseAccount

Initiate the process of closing the Holder's account. Once closed, the account becomes inoperable, and no further transactions or operations can be conducted. This action is irreversible.

The account balance must be zero for the closure to proceed. If there are pending pre-authorization requests, the account can still be closed. This ensures that the Holder's intent to close the account is not hindered by any pending transactions. If any pre-authorization requests have received approval, they must be finalized and settled before the Holder can proceed to empty the account and complete the closure process.

***Actor:*** Holder

***Preconditions:***

- The account balance must be zero.

|parameter|description|
|----------|-----------|
|`close_account`|*(Required.) * **object**. |

### ExecuteMsg::InitiatePreAuthorization

Start the initial step taken by a Provider to request a pre-authorization of funds from a client. By initiating this, the provider signals the intent to reserve a specified amount of the client's funds in the smart contract as a guarantee for a future transaction.

Upon initiation, the pre-authorization request is assigned a unique identifier that can be used to track the status of the request.

The pre-authorization request comes with two distinct expiration timelines. The 'approval expiration' sets the time limit within which the client must approve the pre-authorization request. On the other hand, the 'locking expiration' defines the maximum time period that the funds will remain locked in the smart contract once the client has given approval.

``` plantuml

@startuml

hide time-axis

scale 10 as 150 pixels

concise "state" as ST

ST is ""

@ST

0 is pending

+10 is approved

+15 is finalized

highlight 0 to 12 #line:DimGrey : \napproval expiration

highlight 10 to 30 #Gold;line:Gold : \nlocking expiration

@enduml

```

***Actor:*** Provider

|parameter|description|
|----------|-----------|
|`initiate_pre_authorization`|*(Required.) * **object**. |
|`initiate_pre_authorization.amount`|*(Required.) * **Array&lt;[Coin](#coin)&gt;**. The amount of tokens to lock in the pre-authorization.|
|`initiate_pre_authorization.approval_expiration`|*(Required.) * **[Expiration](#expiration)**. The expiration of the waiting period for client approval, expressed as a block height or a block time.|
|`initiate_pre_authorization.destination_account`|**string\|null**. Account to which the funds will be transferred upon finalization of the pre-authorization. This field allows for the specification of an intermediary account, like an escrow service, to temporarily hold the funds until all parties reach a mutual agreement on the transaction.<br /><br />If not provided, the funds will be transferred to the Provider's account.|
|`initiate_pre_authorization.locking_expiration`|*(Required.) * **[Expiration](#expiration)**. The expiration of the locking period, starting from the time of client approval, expressed as a block height or a block time.|
|`initiate_pre_authorization.reference_id`|**string\|null**. An external identifier for the pre-authorization request. This field can be used to link the pre-authorization request to an external system.|

### ExecuteMsg::ApprovePreAuthorization

Approve the pre-authorization request initiated by a provider. Upon approval, the specified amount of tokens are locked and reserved in the smart contract for a maximum duration specified by the expiration.

***Actor:*** Client

***Preconditions:***

- The status of the pre-authorization request is `Pending`.

- The client has sufficient available funds (i.e., funds that aren't tied to any other active pre-authorizations) in their smart contract account to cover the requested pre-authorization amount.

|parameter|description|
|----------|-----------|
|`approve_pre_authorization`|*(Required.) * **object**. |
|`approve_pre_authorization.id`|*(Required.) * **string**. The unique identifier of the pre-authorization request to approve.|

### ExecuteMsg::DeclinePreAuthorization

Decline a pre-authorization request initiated by a provider. By declining, the client signals that they do not agree to lock any funds for the future transaction.

***Actor:*** Client

***Preconditions:***

- The sender of the message must be the Client.

- The client has received a pre-authorization request from a provider.

|parameter|description|
|----------|-----------|
|`decline_pre_authorization`|*(Required.) * **object**. |
|`decline_pre_authorization.id`|*(Required.) * **string**. The unique identifier of the pre-authorization request to decline.|

### ExecuteMsg::CancelPreAuthorization

Allows the provider to cancel a pre-authorization request that they have initiated. By canceling, the provider signals that they no longer wish to proceed with the transaction. Any funds that were locked for this specific pre-authorization are unlocked, and the transaction process ends.

***Actor:*** Provider

***Preconditions:***

- A pre-authorization request has been initiated by the provider and is in the `Pending` or `Approved` state.

|parameter|description|
|----------|-----------|
|`cancel_pre_authorization`|*(Required.) * **object**. |
|`cancel_pre_authorization.id`|*(Required.) * **string**. The unique identifier of the pre-authorization request to cancel.|
|`cancel_pre_authorization.reason`|**string\|null**. The reason for canceling the pre-authorization request. This field is optional and can be used to provide additional context to the client.|

### ExecuteMsg::FinalizePreAuthorization

Finalizes a pre-authorization request by transferring the locked funds from the client's account to the account specified in the pre-authorization request. The amount transferred could be all or a portion of the initially locked funds, depending on the final cost of the service. Any remaining locked funds are unlocked and returned to the client's account, and the transaction process ends.

This action is taken by the provider after the service has been successfully delivered.

***Actor:*** Provider

***Preconditions:***

- The pre-authorization request must be in the `Approved` state.

- The final amount must be less than or equal to the initially locked amount.

|parameter|description|
|----------|-----------|
|`finalize_pre_authorization`|*(Required.) * **object**. |
|`finalize_pre_authorization.final_amount`|*(Required.) * **Array&lt;[Coin](#coin)&gt;**. The final amount to be transferred from the client's locked funds to the provider's account. This amount must be less than or equal to the initially locked amount.|
|`finalize_pre_authorization.id`|*(Required.) * **string**. The unique identifier of the pre-authorization request to be finalized.|

## QueryMsg

Query messages

### QueryMsg::Account

Query the details of the account.

|parameter|description|
|----------|-----------|
|`account`|*(Required.) * **object**. |

### QueryMsg::Balance

Query the balance of the account.

|parameter|description|
|----------|-----------|
|`balance`|*(Required.) * **object**. |

### QueryMsg::Limits

Query the limits of the account.

|parameter|description|
|----------|-----------|
|`limits`|*(Required.) * **object**. |

### QueryMsg::PreAuthorization

Query the details of a pre-authorization request.

|parameter|description|
|----------|-----------|
|`pre_authorization`|*(Required.) * **object**. |
|`pre_authorization.by`|*(Required.) * **[ByFilter](#byfilter)**. The filter to apply to the pre-authorization request.|

### QueryMsg::PreAuthorizations

Query all pre-authorization requests initiated by a provider according to the specified filters.

|parameter|description|
|----------|-----------|
|`pre_authorizations`|*(Required.) * **object**. |
|`pre_authorizations.after`|**string\|null**. The cursor from which to start returning pre-authorization requests.|
|`pre_authorizations.first`|**integer\|null**. The maximum number of pre-authorization requests to return.|
|`pre_authorizations.where`|**[WhereFilter](#wherefilter)\|null**. The filters to apply to the pre-authorization requests.|

## Responses

### account

Represents an account response.

|property|description|
|----------|-----------|
|`accepted_denoms`|**array\|null**. The accepted denominations for this account.|
|`name`|**string\|null**. The name of the account.|
|`status`|*(Required.) * **[AccountStatus](#accountstatus)**. The current status of the account.|

### balance

Represents a balance response.

|property|description|
|----------|-----------|
|`available_balance`|*(Required.) * **Array&lt;[Coin](#coin)&gt;**. The current available balance of the account. This is the amount of funds that are currently available for use.|
|`balance`|*(Required.) * **Array&lt;[Coin](#coin)&gt;**. The current balance of the account. This is the total amount of funds that are currently in the account.|
|`locked_balance`|*(Required.) * **Array&lt;[Coin](#coin)&gt;**. The current locked balance of the account. This is the amount of funds that are currently locked in pre-authorization requests.|

### limits

Represents the limits response.

|property|description|
|----------|-----------|
|`account_limits`|*(Required.) * **[AccountLimitsConfig](#accountlimitsconfig)**. The limits specifically configured for the account.|
|`account_limits.accepted_denoms`|**array\|null**. Specifies the list of token denominations that the account will accept. If not provided, the account will accept all known token denominations.|
|`account_limits.allowed_deposit_senders`|**[WhitelistBlacklistAddress](#whitelistblacklistaddress)**. Specifies the addresses that are permitted to deposit funds into the account. If not provided, deposits from any address will be accepted.<br />**Default:** `{"blacklisted_addresses":null,"whitelisted_addresses":null}`|
|`account_limits.allowed_withdraw_recipients`|**[WhitelistBlacklistAddress](#whitelistblacklistaddress)**. Specifies the addresses that are permitted to receive withdrawals from the account. If not provided, withdrawals to any address will be allowed.<br />**Default:** `{"blacklisted_addresses":null,"whitelisted_addresses":null}`|
|`pre_authorization_limits`|*(Required.) * **[PreAuthorizationLimitsConfig](#preauthorizationlimitsconfig)**. The limits specifically configured for pre-authorization requests.|
|`pre_authorization_limits.allowed_providers`|**[WhitelistBlacklistAddress](#whitelistblacklistaddress)**. Specifies the providers that are permitted to initiate a pre-authorization request for the account. If not provided, any provider can initiate a pre-authorization.<br />**Default:** `{"blacklisted_addresses":null,"whitelisted_addresses":null}`|
|`pre_authorization_limits.max_approval_expiration`|**[Expiration](#expiration)\|null**. Specifies the maximum duration that can be set for the approval phase of a pre-authorization.|
|`pre_authorization_limits.max_locking_expiration`|**[Expiration](#expiration)\|null**. Specifies the maximum duration that can be set for the locking phase of a pre-authorization.|
|`pre_authorization_limits.max_pre_authorization_lifetime`|**[Expiration](#expiration)\|null**. Specifies the maximum duration for which a pre-authorization can be active.|

### pre_authorization

Represents a pre-authorization response.

|property|description|
|----------|-----------|
|`amount`|*(Required.) * **Array&lt;[Coin](#coin)&gt;**. The amount to be locked from the client's account.|
|`approval_expiration`|*(Required.) * **[Expiration](#expiration)**. The expiration of the waiting period for client approval, expressed as a block height or a block time.|
|`destination_account`|**string\|null**. Account to which the funds will be transferred upon finalization of the pre-authorization.|
|`id`|*(Required.) * **string**. The unique identifier of the pre-authorization request.|
|`locking_expiration`|*(Required.) * **[Expiration](#expiration)**. The expiration of the locking period, starting from the time of client approval, expressed as a block height or a block time.|
|`provider`|*(Required.) * **string**. The provider's identity.|
|`reference_id`|**string\|null**. The reference identifier of the pre-authorization request.|
|`status`|*(Required.) * **[PreAuthorizationStatus](#preauthorizationstatus)**. The current status of the pre-authorization request.|

### pre_authorizations

Represents a pre-authorization set response.

|property|description|
|----------|-----------|
|`data`|*(Required.) * **Array&lt;[PreAuthorizationResponse](#preauthorizationresponse)&gt;**. The pre-authorization requests.|
|`page_info`|*(Required.) * **[PageInfo](#pageinfo)**. The page information.|
|`page_info.cursor`|**string**. The cursor to the next page.|
|`page_info.has_next_page`|**boolean**. Tells if there is a next page.|

## Definitions

### AccountLimitsConfig

Represents the possible limits that can be configured for an account.

|property|description|
|----------|-----------|
|`accepted_denoms`|**array\|null**. Specifies the list of token denominations that the account will accept. If not provided, the account will accept all known token denominations.|
|`allowed_deposit_senders`|**[WhitelistBlacklistAddress](#whitelistblacklistaddress)**. Specifies the addresses that are permitted to deposit funds into the account. If not provided, deposits from any address will be accepted.|
|`allowed_deposit_senders.blacklisted_addresses`|**array\|null**. A list of blacklisted addresses. Blacklisted addresses always take precedence over whitelisted addresses. If not provided, no addresses are blacklisted.|
|`allowed_deposit_senders.whitelisted_addresses`|**array\|null**. A list of whitelisted addresses. If not provided, any address is allowed.|
|`allowed_withdraw_recipients`|**[WhitelistBlacklistAddress](#whitelistblacklistaddress)**. Specifies the addresses that are permitted to receive withdrawals from the account. If not provided, withdrawals to any address will be allowed.|
|`allowed_withdraw_recipients.blacklisted_addresses`|**array\|null**. A list of blacklisted addresses. Blacklisted addresses always take precedence over whitelisted addresses. If not provided, no addresses are blacklisted.|
|`allowed_withdraw_recipients.whitelisted_addresses`|**array\|null**. A list of whitelisted addresses. If not provided, any address is allowed.|

### AccountStatus

Represents the status of the Holder's account.

``` plantuml

@startuml

[*] -[bold]-> Open : (Holder) Create account\n(Smart Contract instantiation)

Open --> Open : (*) *

Open --> Closed : [balance is 0]\n(Holder) Close

Closed --> [*]

@enduml

```

|variant|description|
|-------|-----------|
|[Open](#open)|**string**: `open`. The account is open, and all operations can be performed on the Holder's account.|
|[Closed](#closed)|**string**: `closed`. The account is closed, and no operations can be performed on the Holder's account. Once in this state, the account becomes inoperable, and no further transactions or operations can be executed on the Holder's account.|

### Approved

The client has approved the pre-authorization request, and the funds are now locked in and reserved for the transaction. The provider can now deliver the service, and upon completion, finalize the transaction.

|literal|
|-------|
|`"approved"`|

### ByFilter

Represents the filter that can be applied when querying for a specific pre-authorization request.

|variant|description|
|-------|-----------|
|[EqId](#eqid)|**object**. Filter by the unique identifier of the pre-authorization request.|
|[EqReferenceId](#eqreferenceid)|**object**. |

### Cancelled

The provider has chosen to cancel the pre-authorization request. This could be due to various reasons, such as service unavailability or a change in terms. Any locked funds are unlocked for the client, and the transaction process ends.

|property|description|
|----------|-----------|
|`cancelled`|*(Required.) * **object**. |
|`cancelled.phase`|*(Required.) * **[PreAuthorizationPhase](#preauthorizationphase)**. Indicates whether this occurs Pre or Post approval.|
|`cancelled.reason`|**string\|null**. The reason for the cancellation (if any).|

### Closed

The account is closed, and no operations can be performed on the Holder's account. Once in this state, the account becomes inoperable, and no further transactions or operations can be executed on the Holder's account.

|literal|
|-------|
|`"closed"`|

### Coin



|property|description|
|----------|-----------|
|`amount`|*(Required.) * **string**. |
|`denom`|*(Required.) * **string**. |

### Declined

The client has declined the pre-authorization request. No funds are locked or transferred, and the transaction process ends.

|literal|
|-------|
|`"declined"`|

### EqId

Filter by the unique identifier of the pre-authorization request.

|property|description|
|----------|-----------|
|`eq_id`|*(Required.) * **string**. |

### EqProvider

Filter by the provider ID.

|property|description|
|----------|-----------|
|`eq_provider`|*(Required.) * **string**. |

### EqReferenceId



|property|description|
|----------|-----------|
|`eq_reference_id`|*(Required.) * **string**. |

### EqStatus

Filter by the status of the pre-authorization.

|property|description|
|----------|-----------|
|`eq_status`|*(Required.) * **[Pending](#pending)\|[Approved](#approved)\|[Declined](#declined)\|[Cancelled](#cancelled)\|[Expired](#expired)\|[Finalized](#finalized)**. |

### Expiration

Expiration represents a point in time when some event happens. It can compare with a BlockInfo and will return is_expired() == true once the condition is hit (and for every block in the future)

|variant|description|
|-------|-----------|
|undefined|**object**. AtHeight will expire when `env.block.height` &gt;= height|
|undefined|**object**. AtTime will expire when `env.block.time` &gt;= time|
|undefined|**object**. Never will never expire. Used to express the empty variant|

### Expired

The pre-authorization request has not been finalized within a specified timeframe and has thus expired. Any locked funds are unlocked for the client, and the transaction process ends.

|property|description|
|----------|-----------|
|`expired`|*(Required.) * **object**. |
|`expired.phase`|*(Required.) * **[PreAuthorizationPhase](#preauthorizationphase)**. The phase indicates whether this occurs Pre or Post approval.|

### Finalized

Upon successful provision of the service, the provider finalizes the transaction. The appropriate amount, which could be all or a portion of the locked funds, is transferred to the provider as payment. Any remaining funds are unlocked for the client, concluding the transaction process.

|literal|
|-------|
|`"finalized"`|

### Open

The account is open, and all operations can be performed on the Holder's account.

|literal|
|-------|
|`"open"`|

### PageInfo

PageInfo is the page information returned for paginated queries.

|property|description|
|----------|-----------|
|`cursor`|*(Required.) * **string**. The cursor to the next page.|
|`has_next_page`|*(Required.) * **boolean**. Tells if there is a next page.|

### Pending

Represents the period after the provider has initiated a pre-authorization request and is waiting for the client's approval.

|literal|
|-------|
|`"pending"`|

### PostApproval

Occurs after the client has given their approval (but before finalization).

|literal|
|-------|
|`"post_approval"`|

### PreApproval

Occurs before the client has given their approval.

|literal|
|-------|
|`"pre_approval"`|

### PreAuthorizationLimitsConfig

Represents the possible limits that can be configured for pre-authorization requests.

|property|description|
|----------|-----------|
|`allowed_providers`|**[WhitelistBlacklistAddress](#whitelistblacklistaddress)**. Specifies the providers that are permitted to initiate a pre-authorization request for the account. If not provided, any provider can initiate a pre-authorization.|
|`allowed_providers.blacklisted_addresses`|**array\|null**. A list of blacklisted addresses. Blacklisted addresses always take precedence over whitelisted addresses. If not provided, no addresses are blacklisted.|
|`allowed_providers.whitelisted_addresses`|**array\|null**. A list of whitelisted addresses. If not provided, any address is allowed.|
|`max_approval_expiration`|**[Expiration](#expiration)\|null**. Specifies the maximum duration that can be set for the approval phase of a pre-authorization.|
|`max_locking_expiration`|**[Expiration](#expiration)\|null**. Specifies the maximum duration that can be set for the locking phase of a pre-authorization.|
|`max_pre_authorization_lifetime`|**[Expiration](#expiration)\|null**. Specifies the maximum duration for which a pre-authorization can be active.|

### PreAuthorizationPhase

Represents the phase of the pre-authorization process.

|variant|description|
|-------|-----------|
|[PreApproval](#preapproval)|**string**: `pre_approval`. Occurs before the client has given their approval.|
|[PostApproval](#postapproval)|**string**: `post_approval`. Occurs after the client has given their approval (but before finalization).|

### PreAuthorizationResponse

Represents a pre-authorization response.

|property|description|
|----------|-----------|
|`amount`|*(Required.) * **Array&lt;[Coin](#coin)&gt;**. The amount to be locked from the client's account.|
|`approval_expiration`|*(Required.) * **[Expiration](#expiration)**. The expiration of the waiting period for client approval, expressed as a block height or a block time.|
|`destination_account`|**string\|null**. Account to which the funds will be transferred upon finalization of the pre-authorization.|
|`id`|*(Required.) * **string**. The unique identifier of the pre-authorization request.|
|`locking_expiration`|*(Required.) * **[Expiration](#expiration)**. The expiration of the locking period, starting from the time of client approval, expressed as a block height or a block time.|
|`provider`|*(Required.) * **string**. The provider's identity.|
|`reference_id`|**string\|null**. The reference identifier of the pre-authorization request.|
|`status`|*(Required.) * **[PreAuthorizationStatus](#preauthorizationstatus)**. The current status of the pre-authorization request.|

### PreAuthorizationStatus

Represents the current phase during the pre-authorization process.

``` plantuml

@startuml

state Declined

state Expired

state Finalized #line.bold

state Cancelled {

state PreApprovalCancelled

state PostApprovalCancelled

}

state Expired {

state PreApprovalExpired

state PostApprovalExpired

}

state Approved #line.bold

state Pending #line.bold

[*] -[bold]-> Pending : (Provider) Initiate

Pending --> Pending : [insufficient funds]\n(Client) Approve

Pending -[bold]-> Approved : [sufficient funds]\n(Client) Approve

Pending --> PreApprovalCancelled : (Provider) Cancel

Pending --> Declined : (Client) Decline

Pending --> PreApprovalExpired : (System) Expire

Approved -[bold]-> Finalized : (Provider) Finalize

Approved --> PostApprovalCancelled : (Provider) Cancel

Approved --> PostApprovalExpired : (System) Expire

Declined --> [*]

Cancelled --> [*]

PreApprovalExpired --> [*]

PostApprovalExpired --> [*]

Finalized -[bold]-> [*]

@enduml

```

|variant|description|
|-------|-----------|
|[Pending](#pending)|**string**: `pending`. Represents the period after the provider has initiated a pre-authorization request and is waiting for the client's approval.|
|[Approved](#approved)|**string**: `approved`. The client has approved the pre-authorization request, and the funds are now locked in and reserved for the transaction. The provider can now deliver the service, and upon completion, finalize the transaction.|
|[Declined](#declined)|**string**: `declined`. The client has declined the pre-authorization request. No funds are locked or transferred, and the transaction process ends.|
|[Cancelled](#cancelled)|**object**. The provider has chosen to cancel the pre-authorization request. This could be due to various reasons, such as service unavailability or a change in terms. Any locked funds are unlocked for the client, and the transaction process ends.|
|[Expired](#expired)|**object**. The pre-authorization request has not been finalized within a specified timeframe and has thus expired. Any locked funds are unlocked for the client, and the transaction process ends.|
|[Finalized](#finalized)|**string**: `finalized`. Upon successful provision of the service, the provider finalizes the transaction. The appropriate amount, which could be all or a portion of the locked funds, is transferred to the provider as payment. Any remaining funds are unlocked for the client, concluding the transaction process.|

### Timestamp

A point in time in nanosecond precision.

This type can represent times from 1970-01-01T00:00:00Z to 2554-07-21T23:34:33Z.

## Examples

``` # use cosmwasm_std::Timestamp; let ts = Timestamp::from_nanos(1_000_000_202); assert_eq!(ts.nanos(), 1_000_000_202); assert_eq!(ts.seconds(), 1); assert_eq!(ts.subsec_nanos(), 202);

let ts = ts.plus_seconds(2); assert_eq!(ts.nanos(), 3_000_000_202); assert_eq!(ts.seconds(), 3); assert_eq!(ts.subsec_nanos(), 202); ```



### Uint128

A string containing a 128-bit integer in decimal representation.

|type|
|----|
|**string**.|

### Uint64

A thin wrapper around u64 that is using strings for JSON encoding/decoding, such that the full u64 range can be used for clients that convert JSON numbers to floats, like JavaScript and jq.

# Examples

Use `from` to create instances of this and `u64` to get the value out:

``` # use cosmwasm_std::Uint64; let a = Uint64::from(42u64); assert_eq!(a.u64(), 42);

let b = Uint64::from(70u32); assert_eq!(b.u64(), 70); ```

|type|
|----|
|**string**.|

### WhereFilter

Represents the filters that can be applied when querying pre-authorization requests.

|variant|description|
|-------|-----------|
|[EqStatus](#eqstatus)|**object**. Filter by the status of the pre-authorization.|
|[EqProvider](#eqprovider)|**object**. Filter by the provider ID.|

### WhitelistBlacklistAddress

Defines the set of possible permissions that can be granted to a provider.

|property|description|
|----------|-----------|
|`blacklisted_addresses`|**array\|null**. A list of blacklisted addresses. Blacklisted addresses always take precedence over whitelisted addresses. If not provided, no addresses are blacklisted.|
|`whitelisted_addresses`|**array\|null**. A list of whitelisted addresses. If not provided, any address is allowed.|

### undefined

AtHeight will expire when `env.block.height` >= height

|property|description|
|----------|-----------|
|`at_height`|*(Required.) * **integer**. |