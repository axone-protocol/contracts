---
name: api-design
description: Best practices for designing CosmWasm smart contract APIs. Use when defining message types, designing execute/query interfaces, or optimizing API ergonomics.
license: BSD-3-Clause
metadata:
  author: axone.xyz
  version: "1.0"
---

# CosmWasm API Design Best Practices

This skill helps you design clean, minimal, and well-documented APIs for CosmWasm smart contracts.

## When to use this skill

Use this skill when you need to:

- Define message types (InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)
- Design execute or query interfaces
- Document APIs for schema generation
- Apply serde patterns for optional fields
- Ensure API consistency across contracts

## Core Principles

1. **Minimalism** - Include only what's necessary; avoid bloated APIs
2. **Clarity** - Names should be self-documenting
3. **Consistency** - Follow established patterns across all contracts
4. **Documentation** - Every public type and field must have doc comments

## Message Type Patterns

### InstantiateMsg

```rust
/// Contract instantiation message
#[cosmwasm_schema::cw_serde]
#[derive(Default)]
pub struct MyContractInstantiateMsg {
    /// Optional configuration parameter with sensible default
    #[serde(default)]
    pub some_config: Option<String>,
}
```

**Guidelines:**

- Derive `Default` when possible for easier testing
- Use `#[serde(default)]` for optional fields
- Keep required fields minimal
- Document each field

### ExecuteMsg

```rust
/// Contract execute messages
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum MyContractExecuteMsg {
    /// Update the contract configuration
    UpdateConfig {
        /// New admin address (optional)
        new_admin: Option<String>,
    },
    /// Process an action with the given parameters
    ProcessAction {
        /// Unique identifier for the action
        action_id: String,
        /// Amount to process
        amount: Uint128,
    },
}
```

**Guidelines:**

- Use verb-based names (Update, Process, Create, Remove)
- Group related parameters in structs if >3 fields
- Document each variant AND each field
- Derive `ExecuteFns` for cw-orch integration

### QueryMsg

```rust
/// Contract query messages
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum MyContractQueryMsg {
    /// Get the current configuration
    #[returns(ConfigResponse)]
    Config {},
    
    /// Get item by ID
    #[returns(ItemResponse)]
    Item {
        /// The item identifier
        id: String,
    },
    
    /// List all items with pagination
    #[returns(ItemsResponse)]
    Items {
        /// Start after this ID for pagination
        start_after: Option<String>,
        /// Maximum number of items to return
        limit: Option<u32>,
    },
}
```

**Guidelines:**

- Always include `#[returns(ResponseType)]` attribute
- Use noun-based names for queries
- Include pagination for list queries (`start_after`, `limit`)
- Derive `QueryFns` and `QueryResponses`

### Response Types

```rust
#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {
    /// Current admin address
    pub admin: Addr,
    /// Whether the contract is paused
    pub paused: bool,
}

#[cosmwasm_schema::cw_serde]
pub struct ItemsResponse {
    /// List of items
    pub items: Vec<ItemInfo>,
}
```

**Guidelines:**

- Response types should mirror what clients need
- Use specific types (Addr, Uint128) not strings
- Document all fields

## Abstract SDK Integration

Use the `app_msg_types!` macro to generate wrapper types:

```rust
use crate::contract::MyContract;
use cosmwasm_schema::QueryResponses;

// Generates ExecuteMsg, QueryMsg, InstantiateMsg wrappers
abstract_app::app_msg_types!(MyContract, MyContractExecuteMsg, MyContractQueryMsg);
```

## Documentation Standards

### Rust Doc Comments

```rust
/// Brief one-line description of the variant.
/// 
/// Optional longer description that explains:
/// - When to use this
/// - Side effects
/// - Related messages
/// 
/// # Errors
/// 
/// Returns `ContractError::Unauthorized` if caller is not admin.
```

### Field Documentation

Every field must have a doc comment:

- Describe what the field represents
- Mention default values if applicable
- Note any constraints (min/max values, format)

## Serde Patterns

### Optional Fields with Defaults

```rust
#[serde(default)]
pub optional_field: Option<String>,

#[serde(default = "default_limit")]
pub limit: u32,

fn default_limit() -> u32 {
    10
}
```

### Flatten for Nested Configs

```rust
#[cosmwasm_schema::cw_serde]
pub struct InstantiateMsg {
    #[serde(flatten)]
    pub base_config: BaseConfig,
    pub custom_field: String,
}
```

### Rename for JSON Clarity

```rust
#[serde(rename = "owner")]
pub owner_addr: Addr,
```

## Checklist for New APIs

- [ ] All message variants have doc comments
- [ ] All struct fields have doc comments
- [ ] Query messages have `#[returns(...)]` attribute
- [ ] Execute messages describe potential errors
- [ ] Pagination is included for list queries
- [ ] Optional fields use `Option<T>` or `#[serde(default)]`
- [ ] Response types are defined for all queries
- [ ] Types use `Addr`/`Uint128` not raw strings/numbers
