# okp4-pay-secure

A [CosmWasm](https://cosmwasm.com/) smart contract designed to facilitate secure, pre-authorized token transactions between clients and providers, akin to credit card pre-authorizations in traditional finance systems.

## Purpose

The primary objective of this smart contract is to provide a robust pre-authorization framework for token-based transactions. It allows clients to earmark a specific amount of tokens in favor of a provider without actually transferring them, thereby confirming the client's ability to cover the transaction cost.

## Rationale

In scenarios where a provider offers services on a deferred payment basis, there's a need for a mechanism that not only verifies but also ensures that the client has adequate funds to fulfill the payment obligation. This smart contract addresses this by "reserving" a specified amount of tokens in the client's account. These reserved funds are deducted from the available balance but remain in the client's account until the provider initiates the final settlement. At that point, the reserved tokens are transferred to the designated recipient, which could be the provider's account or an optional intermediary account, such as an escrow service.

## Features

The smart contract offers two distinct categories of features:

- [Smart Contract Account (SCA)](https://blog.ambire.com/eoas-vs-smart-contract-accounts/) Capabilities: The smart contract is under the ownership and control of the holder. Similar to an Externally Owned Account (EOA), the holder has the ability to deposit, withdraw, and transfer funds to and from this account.
- Pre-Authorization Features: A provider has the option to pre-authorize a specific amount of tokens along with designating a recipient account for a predetermined duration, after which the pre-authorization will expire. This designated recipient account can serve as an intermediary for holding the pre-authorized funds, such as an escrow service.

## Terminology

### Funds

Tokens allocated in the Smart Contract associated with a Holder's profile.

### Locked Funds

A specific portion of funds retained within the smart contract during the pre-authorization phase initiated by the Provider. These funds act as a commitment from the Client for the execution of a service and remain inaccessible until the pre-authorization concludes, is revoked or expires.

### Available Funds

Funds present in the Holder's smart contract account that aren't tied to any pre-authorization. The Contract Holder has full autonomy over these funds, allowing them to utilize, transfer, or retrieve them as they see fit.

### Pre-authorization

A procedure set in motion by the Provider to confirm the financial capability of the Client's account. This entails momentarily reserving a defined sum of funds as an assurance for service payment. These reserved funds stay in this condition until the service transaction finalizes, the pre-authorization concludes, is revoked or expires.

## Stakeholders

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

### Sender

The entity that initiates transactions to transfer funds to the Smart Contract.

### Receiver

The entity that receives funds from the Smart Contract.

### Holder

The entity that possesses the Smart Contract and controls the funds stored within it.

### Client

The Holder who aims to acquire a service, securing a specific token quantity as a pre-approval for the Provider.

### Provider

An entity that offers a service and seeks a token pre-authorization before delivering the said service.
