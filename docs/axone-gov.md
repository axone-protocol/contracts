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

### ExecuteMsg::increment

Increment count by 1

| parameter   | description                |
| ----------- | -------------------------- |
| `increment` | _(Required.) _ **object**. |

### ExecuteMsg::reset

Admin method - reset count

| parameter     | description                                         |
| ------------- | --------------------------------------------------- |
| `reset`       | _(Required.) _ **object**.                          |
| `reset.count` | _(Required.) _ **integer**. Count value after reset |

## QueryMsg

App query messages

### QueryMsg::config

| parameter | description                |
| --------- | -------------------------- |
| `config`  | _(Required.) _ **object**. |

### QueryMsg::count

| parameter | description                |
| --------- | -------------------------- |
| `count`   | _(Required.) _ **object**. |

## MigrateMsg

### MigrateMsg::MigrateMsg

| parameter | description |
| --------- | ----------- |

## Responses

### config

| type        |
| ----------- |
| **object**. |

### count

| property | description                 |
| -------- | --------------------------- |
| `count`  | _(Required.) _ **integer**. |

---

_Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `axone-gov.json` (`68d5c80810cc40b0`)_
