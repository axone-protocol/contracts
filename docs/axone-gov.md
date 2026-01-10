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

`constitution` is the Prolog program (UTF-8 bytes) that defines the governance rules. The contract validates that it provides the required predicates (`decide/2` and `decide/3`) during instantiation.

| variant        | description                                                                                                                                                                                                                                         |
| -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| InstantiateMsg | **object**. Instantiate message.<br /><br />`constitution` is the Prolog program (UTF-8 bytes) that defines the governance rules. The contract validates that it provides the required predicates (`decide/2` and `decide/3`) during instantiation. |

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

Return the stored governance constitution program.

| parameter      | description                |
| -------------- | -------------------------- |
| `constitution` | _(Required.) _ **object**. |

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

| property     | description                                                             |
| ------------ | ----------------------------------------------------------------------- |
| `governance` | _(Required.) _ **[Binary](#binary)**. Stored Prolog governance program. |

## Definitions

### Binary

A string containing Base64-encoded data.

| type        |
| ----------- |
| **string**. |

---

_Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `axone-gov.json` (`51c999a033214538`)_
