# Law Stone

## Overview

The `okp4-law-stone` smart contract aims to provide GaaS (i.e. Governance as a Service) in any [Cosmos blockchains](https://cosmos.network/) using the [CosmWasm](https://cosmwasm.com/) framework and the [Logic](https://docs.okp4.network/modules/next/logic) OKP4 module.

This contract is built around a Prolog program describing the law by rules and facts. The law stone is immutable, this means it can only be questioned, there are no update mechanisms.

The `okp4-law-stone` responsibility is to guarantee the availability of its rules in order to question them, but not to ensure the rules application.

To ensure reliability over time, the associated Prolog program is stored and pinned in a `okp4-objectarium` contract. Moreover, all the eventual loaded files must be stored in a `okp4-objectarium` contract as well, allowing the contract to pin them.

To be able to free the underlying resources (i.e. objects in `okp4-objectarium`) if not used anymore, the contract admin can break the stone.

➡️ Checkout the [examples](https://github.com/okp4/contracts/tree/main/contracts/okp4-law-stone/examples/) for usage information.

## InstantiateMsg

Instantiate message

| parameter         | description                                                                                           |
| ----------------- | ----------------------------------------------------------------------------------------------------- |
| `program`         | _(Required.) _ **[Binary](#binary)**. The Prolog program carrying law rules and facts.                |
| `storage_address` | _(Required.) _ **string**. The `okp4-objectarium` contract address on which to store the law program. |

## ExecuteMsg

Execute messages

### ExecuteMsg::BreakStone

Break the stone making this contract unusable, by clearing all the related resources: - Unpin all the pinned objects on `okp4-objectarium` contracts, if any. - Forget the main program (i.e. or at least unpin it). Only the contract admin is authorized to break it, if any. If already broken, this is a no-op.

| literal         |
| --------------- |
| `"break_stone"` |

## QueryMsg

Query messages

### QueryMsg::Ask

If not broken, ask the logic module the provided query with the law program loaded.

| parameter   | description                |
| ----------- | -------------------------- |
| `ask`       | _(Required.) _ **object**. |
| `ask.query` | _(Required.) _ **string**. |

### QueryMsg::Program

If not broken, returns the law program location information.

| literal     |
| ----------- |
| `"program"` |

### QueryMsg::ProgramCode

ProgramCode returns the law program code.

| literal          |
| ---------------- |
| `"program_code"` |

## Responses

### ask

| property   | description                  |
| ---------- | ---------------------------- |
| `answer`   | **[Answer](#answer)\|null**. |
| `gas_used` | _(Required.) _ **integer**.  |
| `height`   | _(Required.) _ **integer**.  |

### program

ProgramResponse carry elements to locate the program in a `okp4-objectarium` contract.

| property          | description                                                                                            |
| ----------------- | ------------------------------------------------------------------------------------------------------ |
| `object_id`       | _(Required.) _ **string**. The program object id in the `okp4-objectarium` contract.                   |
| `storage_address` | _(Required.) _ **string**. The `okp4-objectarium` contract address on which the law program is stored. |

### program_code

Binary is a wrapper around Vec&lt;u8&gt; to add base64 de/serialization with serde. It also adds some helper methods to help encode inline.

This is only needed as serde-json-\{core,wasm\} has a horrible encoding for Vec&lt;u8&gt;. See also &lt;https://github.com/CosmWasm/cosmwasm/blob/main/docs/MESSAGE_TYPES.md&gt;.

| type        |
| ----------- |
| **string**. |

## Definitions

### Answer

| property    | description                                        |
| ----------- | -------------------------------------------------- |
| `has_more`  | _(Required.) _ **boolean**.                        |
| `results`   | _(Required.) _ **Array&lt;[Result](#result)&gt;**. |
| `success`   | _(Required.) _ **boolean**.                        |
| `variables` | _(Required.) _ **Array&lt;string&gt;**.            |

### Binary

A string containing Base64-encoded data.

| type        |
| ----------- |
| **string**. |

### Result

| property        | description                                                    |
| --------------- | -------------------------------------------------------------- |
| `substitutions` | _(Required.) _ **Array&lt;[Substitution](#substitution)&gt;**. |

### Substitution

| property   | description                |
| ---------- | -------------------------- |
| `term`     | _(Required.) _ **object**. |
| `variable` | _(Required.) _ **string**. |

### Term

| property    | description                                    |
| ----------- | ---------------------------------------------- |
| `arguments` | _(Required.) _ **Array&lt;[Term](#term)&gt;**. |
| `name`      | _(Required.) _ **string**.                     |

---

_Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `okp4-law-stone.json` (`092608edf6c36d25`)_
