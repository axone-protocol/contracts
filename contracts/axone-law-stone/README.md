# Law Stone

## Overview

The `axone-law-stone` smart contract aims to provide GaaS (i.e. Governance as a Service) in any [Cosmos blockchains](https://cosmos.network/) using the [CosmWasm](https://cosmwasm.com/) framework and the [Logic](https://docs.axone.xyz/modules/next/logic) AXONE module.

This contract is built around a Prolog program describing the law by rules and facts. The law stone is immutable, this means it can only be questioned, there are no update mechanisms.

The `axone-law-stone` responsibility is to guarantee the availability of its rules in order to question them, but not to ensure the rules application.

To ensure reliability over time, the associated Prolog program is stored and pinned in a `axone-objectarium` contract. Moreover, all the eventual loaded files must be stored in a `axone-objectarium` contract as well, allowing the contract to pin them.

To be able to free the underlying resources (i.e. objects in `axone-objectarium`) if not used anymore, the contract admin can break the stone.

➡️ Checkout the [examples](https://github.com/axone-protocol/contracts/tree/main/contracts/axone-law-stone/examples/) for usage information.
