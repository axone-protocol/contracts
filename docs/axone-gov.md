# AXONE Governance Contract

A governance contract built using the Abstract SDK for the AXONE protocol.

This contract provides governance capabilities built on top of the Abstract SDK framework, integrating with the AXONE ecosystem.

## Features

- Governance functionality using Abstract SDK
- Integration with AXONE protocol
- CosmWasm-based smart contract

## Usage

This contract can be instantiated and used within the AXONE blockchain network to provide governance capabilities.

## Development

This contract follows AXONE development patterns and uses the cargo-make build system. Use `cargo make` commands for building, testing, and schema generation.

## InstantiateMsg

Instantiate message.

This contract stores a governance constitution as a Prolog program on the resource AA. The constitution defines governance rules using Prolog predicates. The `constitution` field must contain a UTF-8 encoded Prolog program.

During instantiation, the contract validates that the constitution defines the required predicates: - `decide/2` which takes a `Case` argument and returns a verdict term. - `decide/3` which takes a `Case` argument and returns both a verdict and a motivation term.

The `decide/2` predicate returns a verdict Prolog term indicating the decision outcome. The `decide/3` predicate returns both a verdict and a motivation term providing reasoning for the decision.

| variant        | description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| InstantiateMsg | **object**. Instantiate message.<br /><br />This contract stores a governance constitution as a Prolog program on the resource AA. The constitution defines governance rules using Prolog predicates. The `constitution` field must contain a UTF-8 encoded Prolog program.<br /><br />During instantiation, the contract validates that the constitution defines the required predicates: - `decide/2` which takes a `Case` argument and returns a verdict term. - `decide/3` which takes a `Case` argument and returns both a verdict and a motivation term.<br /><br />The `decide/2` predicate returns a verdict Prolog term indicating the decision outcome. The `decide/3` predicate returns both a verdict and a motivation term providing reasoning for the decision. |

## ExecuteMsg

Execute messages.

### ExecuteMsg::no_op

No-op execute message

| parameter | description                |
| --------- | -------------------------- |
| `no_op`   | _(Required.) _ **object**. |

## QueryMsg

Query messages.

### QueryMsg::constitution

Return the stored governance constitution program bytes.

| parameter      | description                |
| -------------- | -------------------------- |
| `constitution` | _(Required.) _ **object**. |

### QueryMsg::constitution_status

Return the stored constitution status metadata.

| parameter             | description                |
| --------------------- | -------------------------- |
| `constitution_status` | _(Required.) _ **object**. |

### QueryMsg::decide

Decide a case using the constitution's `decide/2` or `decide/3` predicate.

The `case` parameter is a Prolog dict term string that represents the decision context. This string is passed as the `Case` argument to the `decide` predicate.

Example of a case dict: `ctx{action:read, user:"did:example:123", object:"obj:42"}`

The `verdict` returned is an arbitrary Prolog term (which can be an atom or a compound term, e.g., `permitted` or `pay(user_1)`), representing the decision outcome.

If `motivated` is true, the contract calls `decide/3` and returns both `verdict` and `motivation`. The `motivation` is a Prolog term that provides reasoning behind the decision.

| parameter          | description                 |
| ------------------ | --------------------------- |
| `decide`           | _(Required.) _ **object**.  |
| `decide.case`      | _(Required.) _ **string**.  |
| `decide.motivated` | _(Required.) _ **boolean**. |

## MigrateMsg

Migrate message.

Reserved for future migrations.

### MigrateMsg::MigrateMsg

Migrate message.

Reserved for future migrations.

| parameter | description |
| --------- | ----------- |

## Responses

### constitution

Response returned by `QueryMsg::Constitution`.

| property     | description                                                                                    |
| ------------ | ---------------------------------------------------------------------------------------------- |
| `governance` | _(Required.) _ **[Binary](#binary)**. The stored Prolog governance constitution program bytes. |

### constitution_status

Response returned by `QueryMsg::ConstitutionStatus`.

| property                | description                                                                                                                                                                                               |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `constitution_hash`     | _(Required.) _ **[Binary](#binary)**. The stored constitution hash (32 bytes).                                                                                                                            |
| `constitution_revision` | _(Required.) _ **integer**. The stored constitution revision number.<br /><br />Revision starts at `0` for the initial constitution and is incremented by `1` on each successful constitutional revision. |

### decide

Response returned by `QueryMsg::Decide`.

| property     | description                                                                              |
| ------------ | ---------------------------------------------------------------------------------------- |
| `motivation` | **string\|null**. Optional motivation term returned as the third argument by `decide/3`. |
| `verdict`    | _(Required.) _ **string**. The decision verdict as a Prolog term string.                 |

## Definitions

### Binary

A string containing Base64-encoded data.

| type        |
| ----------- |
| **string**. |

---

_Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `axone-gov.json` (`b34bf756c89335fc`)_
