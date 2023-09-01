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
