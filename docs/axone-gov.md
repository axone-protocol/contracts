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

App instantiate message

| variant        | description                         |
| -------------- | ----------------------------------- |
| InstantiateMsg | **object**. App instantiate message |

## ExecuteMsg

App execute messages

### ExecuteMsg::update_config

| parameter       | description                |
| --------------- | -------------------------- |
| `update_config` | _(Required.) _ **object**. |

## QueryMsg

App query messages

### QueryMsg::config

| parameter | description                |
| --------- | -------------------------- |
| `config`  | _(Required.) _ **object**. |

### QueryMsg::constitution

| parameter      | description                |
| -------------- | -------------------------- |
| `constitution` | _(Required.) _ **object**. |

## MigrateMsg

### MigrateMsg::MigrateMsg

| parameter | description |
| --------- | ----------- |

## Responses

### config

| type        |
| ----------- |
| **object**. |

### constitution

| property     | description                |
| ------------ | -------------------------- |
| `governance` | _(Required.) _ **string**. |

## Definitions

### Binary

A string containing Base64-encoded data.

| type        |
| ----------- |
| **string**. |

---

_Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `axone-gov.json` (`c0f351e4f83ae34d`)_
