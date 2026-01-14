---
name: cosmwasm-contract
description: Guide for developing CosmWasm smart contracts using the Abstract SDK. Use when creating new contracts, implementing handlers, or working with contract structure.
license: BSD-3-Clause
metadata:
  author: axone.xyz
  version: "1.0"
---

# CosmWasm Smart Contract Development with Abstract SDK

This skill helps you develop CosmWasm smart contracts for the Axone protocol using the Abstract SDK framework.

## When to use this skill

Use this skill when you need to:

- Create a new smart contract from scratch
- Implement contract handlers (instantiate, execute, query, migrate)
- Define contract state and storage
- Work with the Abstract `AppContract` pattern
- Define error types for contracts

## Contract Structure

Every contract follows this standard structure:

```text
contracts/<contract-name>/
├── Cargo.toml           # Dependencies and features
├── Makefile.toml        # Contract-specific tasks
├── README.md            # Contract documentation
├── metadata.json        # Abstract module metadata
├── schema/              # Generated JSON schemas
├── src/
│   ├── lib.rs           # Public exports and constants
│   ├── contract.rs      # AppContract type and entry points
│   ├── msg.rs           # Message types (Instantiate, Execute, Query, Migrate)
│   ├── state.rs         # State storage definitions
│   ├── error.rs         # Error types
│   ├── handlers/        # Handler implementations
│   │   ├── mod.rs
│   │   ├── instantiate.rs
│   │   ├── execute.rs
│   │   ├── query.rs
│   │   └── migrate.rs
│   ├── replies/         # Reply handlers (optional)
│   └── bin/             # Binary scripts (schema, publish, install)
└── tests/
    └── integration.rs   # Integration tests
```

## Key Patterns

### 1. Module ID and Namespace (lib.rs)

```rust
pub const AXONE_NAMESPACE: &str = "axone";
pub const CONTRACT_NAME: &str = "my-contract";
pub const CONTRACT_ID: &str = const_format::concatcp!(AXONE_NAMESPACE, ":", CONTRACT_NAME);
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
```

### 2. AppContract Definition (contract.rs)

```rust
use abstract_app::AppContract;

pub type MyContract = AppContract<
    MyContractError,
    MyContractInstantiateMsg,
    MyContractExecuteMsg,
    MyContractQueryMsg,
    MyContractMigrateMsg,
>;

const APP: MyContract = MyContract::new(CONTRACT_ID, APP_VERSION, None)
    .with_instantiate(handlers::instantiate_handler)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler)
    .with_migrate(handlers::migrate_handler)
    .with_dependencies(&[]);

#[cfg(feature = "export")]
abstract_app::export_endpoints!(APP, MyContract);

abstract_app::cw_orch_interface!(APP, MyContract, MyContractInterface);
```

### 3. Handler Functions

```rust
// handlers/execute.rs
use crate::contract::{MyContract, MyContractResult};
use crate::msg::MyContractExecuteMsg;
use abstract_app::traits::AbstractNameService;

pub fn execute_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    app: MyContract,
    msg: MyContractExecuteMsg,
) -> MyContractResult {
    match msg {
        MyContractExecuteMsg::DoSomething { param } => do_something(deps, env, info, app, param),
    }
}
```

### 4. State Management (state.rs)

```rust
use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("config");
pub const ITEMS: Map<&str, ItemData> = Map::new("items");
```

### 5. Error Types (error.rs)

```rust
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum MyContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Abstract(#[from] AbstractError),

    #[error("{0}")]
    AbstractSdk(#[from] AbstractSdkError),

    #[error("{0}")]
    DappError(#[from] AppError),

    // Custom errors
    #[error("Unauthorized")]
    Unauthorized {},
}
```

## Building and Testing

```bash
# Build the contract
cargo make build

# Build WASM
cargo make wasm

# Run tests
cargo make test-unit

# Generate schemas
cargo make schema
```

## Best Practices

1. **Use Abstract SDK features** - Leverage `AbstractNameService`, `Bank`, and other SDK modules
2. **Keep handlers focused** - Each handler function should do one thing well
3. **Document all messages** - Every message variant needs doc comments for schema generation
4. **Derive traits consistently** - Use `cw_serde`, `ExecuteFns`, `QueryFns`
5. **Test error cases** - Cover both success and failure paths in tests
