# Law Stone

## Overview

The `okp4-law-stone` smart contract aims to provide GaaS (i.e. Governance as a Service) in any [Cosmos blockchains](https://cosmos.network/) using the [CosmWasm](https://cosmwasm.com/) framework and the [Logic](https://docs.okp4.network/modules/next/logic) OKP4 module.

This contract is built around a Prolog program describing the law by rules and facts. The law stone is immutable, this means it can only been questioned, there is no update mechanisms.

The `okp4-law-stone` responsibility is to guarantee the availability of its rules in order to question them, but not to ensure the rules application.

To ensure reliability over time, the associated Prolog program is stored and pinned in a `okp4-objectarium` contract. Moreover, all the eventual loaded files must be stored in a `okp4-objectarium` contract as well, allowing the contract to pin them.

To be able to free the underlying resources (i.e. objects in `okp4-objectarium`) if not used anymore, the contract admin can break the stone.

➡️ Checkout the [examples](https://github.com/okp4/contracts/tree/main/contracts/okp4-law-stone/examples/) for usage information.

## InstantiateMsg

Instantiate message

|parameter|description|
|----------|-----------|
|`program`|*(Required.) * **[Binary](#binary)**. The Prolog program carrying law rules and facts.|
|`storage_address`|*(Required.) * **string**. The `okp4-objectarium` contract address on which to store the law program.|

## ExecuteMsg

Execute messages

### ExecuteMsg::BreakStone

Break the stone making this contract unusable, by clearing all the related resources: - Unpin all the pinned objects on `okp4-objectarium` contracts, if any. - Forget the main program (i.e. or at least unpin it). Only the contract admin is authorized to break it, if any. If already broken, this is a no-op.

|literal|
|-------|
|`"break_stone"`|

## QueryMsg

Query messages

### QueryMsg::Ask

If not broken, ask the logic module the provided query with the law program loaded.

|parameter|description|
|----------|-----------|
|`ask`|*(Required.) * **object**. |
|`ask.query`|*(Required.) * **string**. |

### QueryMsg::Program

If not broken, returns the law program location information.

|literal|
|-------|
|`"program"`|

## Responses

### ask



|property|description|
|----------|-----------|
|`answer`|**[Answer](#answer)\|null**. |
|`gas_used`|*(Required.) * **integer**. |
|`height`|*(Required.) * **integer**. |

### program

ProgramResponse carry elements to locate the program in a `okp4-objectarium` contract.

|property|description|
|----------|-----------|
|`object_id`|*(Required.) * **string**. The program object id in the `okp4-objectarium` contract.|
|`storage_address`|*(Required.) * **string**. The `okp4-objectarium` contract address on which the law program is stored.|

## Definitions

### Answer



|property|description|
|----------|-----------|
|`has_more`|*(Required.) * **boolean**. |
|`results`|*(Required.) * **Array&lt;[Result](#result)&gt;**. |
|`success`|*(Required.) * **boolean**. |
|`variables`|*(Required.) * **Array&lt;string&gt;**. |

### Binary

A string containing Base64-encoded data.

|type|
|----|
|**string**.|

### Result



|property|description|
|----------|-----------|
|`substitutions`|*(Required.) * **Array&lt;[Substitution](#substitution)&gt;**. |

### Substitution



|property|description|
|----------|-----------|
|`term`|*(Required.) * **object**. |
|`variable`|*(Required.) * **string**. |

### Term



|property|description|
|----------|-----------|
|`arguments`|*(Required.) * **Array&lt;[Term](#term)&gt;**. |
|`name`|*(Required.) * **string**. |