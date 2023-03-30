# okp4-law-stone Schema

```txt
undefined
```

# Law Stone

## Overview

The `okp4-law-stone` smart contract aims to provide GaaS (i.e. Governance as a Service) in any [Cosmos blockchains](https://cosmos.network/) using the [CosmWasm](https://cosmwasm.com/) framework and the [Logic](https://docs.okp4.network/modules/next/logic) OKP4 module.

This contract is built around a Prolog program describing the law by rules and facts. The law stone is immutable, this means it can only been questioned, there is no update mechanisms.

The `okp4-law-stone` responsibility is to guarantee the availability of its rules in order to question them, but not to ensure the rules application.

To ensure reliability over time, the associated Prolog program is stored and pinned in a `okp4-objectarium` contract. Moreover, all the eventual loaded files must be stored in a `okp4-objectarium` contract as well, allowing the contract to pin them.

To be able to free the underlying resources (i.e. objects in `okp4-objectarium`) if not used anymore, the contract admin can break the stone.

➡️ Checkout the [examples](https://github.com/okp4/contracts/tree/main/contracts/okp4-law-stone/examples/) for usage information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                               |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json](schema/okp4-law-stone.json "open original schema") |

## okp4-law-stone Type

unknown ([okp4-law-stone](okp4-law-stone.md))
