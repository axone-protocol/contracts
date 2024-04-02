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

Submits a Prolog query string to the `Logic` module, evaluating it against the law program associated with this contract.

If the law stone is broken the query returns a response with the error `error(system_error(broken_law_stone),root)` set in the `answer` field.

| parameter   | description                |
| ----------- | -------------------------- |
| `ask`       | _(Required.) _ **object**. |
| `ask.query` | _(Required.) _ **string**. |

### QueryMsg::Program

Retrieves the location metadata of the law program bound to this contract.

This includes the contract address of the `objectarium` and the program object ID, where the law program's code can be accessed.

| literal     |
| ----------- |
| `"program"` |

### QueryMsg::ProgramCode

Fetches the raw code of the law program tied to this contract.

If the law stone is broken, the query may fail if the program is no longer available in the `Objectarium`.

| literal          |
| ---------------- |
| `"program_code"` |

## Responses

### ask

| property      | description                  |
| ------------- | ---------------------------- |
| `answer`      | **[Answer](#answer)\|null**. |
| `gas_used`    | _(Required.) _ **integer**.  |
| `height`      | _(Required.) _ **integer**.  |
| `user_output` | **string\|null**.            |

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
| `variables` | _(Required.) _ **Array&lt;string&gt;**.            |

### Binary

A string containing Base64-encoded data.

| type        |
| ----------- |
| **string**. |

### Result

| property        | description                                                    |
| --------------- | -------------------------------------------------------------- |
| `error`         | **string\|null**.                                              |
| `substitutions` | _(Required.) _ **Array&lt;[Substitution](#substitution)&gt;**. |

### Substitution

| property     | description                |
| ------------ | -------------------------- |
| `expression` | _(Required.) _ **string**. |
| `variable`   | _(Required.) _ **string**. |

---

_Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `okp4-law-stone.json` (`1c8e531eb7eea016`)_
