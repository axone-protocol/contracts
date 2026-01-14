---
name: documentation
description: Guide for generating and maintaining contract documentation from JSON schemas. Use when creating schemas, generating docs, or documenting contract APIs.
license: BSD-3-Clause
metadata:
  author: axone.xyz
  version: "1.0"
---

# Contract Documentation Generation

This skill helps you generate and maintain documentation for Axone protocol smart contracts from JSON schemas.

## When to use this skill

Use this skill when you need to:

- Generate JSON schemas from message types
- Create markdown documentation from schemas
- Document contract APIs
- Update documentation after contract changes
- Follow documentation best practices

## Documentation Pipeline

```text
Rust Types → JSON Schema → Markdown Docs
   ↓              ↓              ↓
 msg.rs    module-schema.json   docs/*.md
```

### Step 1: Generate Schemas

```bash
cargo make schema
```

This runs the schema binary for each contract, producing:

```text
contracts/<name>/schema/module-schema.json
```

### Step 2: Generate Documentation

```bash
cargo make docs
```

This:

1. Collects schemas from all contracts
2. Processes them with `@fadroma/schema`
3. Formats output with Prettier
4. Outputs to `docs/` directory

### One Command

```bash
cargo make docs  # Runs schema generation automatically
```

## Schema Generation

### Binary Setup

Each contract has a schema binary at `src/bin/schema.rs`:

```rust
use cosmwasm_schema::write_api;
use my_contract::msg::{
    ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg,
};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
        migrate: MigrateMsg,
    }
}
```

### Running for Single Contract

```bash
cd contracts/axone-gov
cargo make schema
```

## Writing Documentation-Ready Code

### Message Documentation

Every message type and field must have doc comments:

```rust
/// Contract execution messages for the governance module.
/// 
/// These messages modify contract state and require authorization.
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    /// Create a new proposal for voting.
    /// 
    /// Only accounts with proposal rights can call this.
    /// The proposal enters the voting period immediately.
    CreateProposal {
        /// Title of the proposal (max 256 chars)
        title: String,
        
        /// Detailed description of what the proposal does
        description: String,
        
        /// Messages to execute if the proposal passes
        msgs: Vec<CosmosMsg>,
    },
    
    /// Cast a vote on an active proposal.
    Vote {
        /// ID of the proposal to vote on
        proposal_id: u64,
        
        /// The vote choice
        vote: VoteOption,
    },
}
```

### Response Types

```rust
/// Response for the Config query.
/// 
/// Contains the current configuration of the governance module.
#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {
    /// Address of the governance admin
    pub admin: Addr,
    
    /// Minimum voting period in seconds
    pub voting_period: u64,
    
    /// Quorum percentage required for proposals (0-100)
    pub quorum: u8,
}
```

### Enum Variants

```rust
/// Options for casting a vote.
#[cosmwasm_schema::cw_serde]
pub enum VoteOption {
    /// Vote in favor of the proposal
    Yes,
    
    /// Vote against the proposal  
    No,
    
    /// Abstain from voting but count toward quorum
    Abstain,
    
    /// Strong opposition that vetoes the proposal
    NoWithVeto,
}
```

## Schema Structure

The generated `module-schema.json` follows this structure:

```json
{
  "contract_name": "axone-gov",
  "contract_version": "8.0.0",
  "idl_version": "1.0.0",
  "instantiate": { ... },
  "execute": { ... },
  "query": { ... },
  "migrate": { ... },
  "responses": { ... }
}
```

## Documentation Best Practices

### 1. Be Concise but Complete

```rust
// ✅ Good
/// Create a new governance proposal for community voting.

// ❌ Too brief
/// Create proposal.

// ❌ Too verbose
/// This function is used to create a new governance proposal 
/// that will be submitted to the community for voting purposes.
```

### 2. Document Constraints

```rust
/// Title of the proposal.
/// 
/// Must be between 1 and 256 characters.
/// Cannot contain newlines.
pub title: String,
```

### 3. Document Default Values

```rust
/// Maximum number of items to return.
/// 
/// Defaults to 10 if not specified. Maximum allowed is 100.
#[serde(default = "default_limit")]
pub limit: Option<u32>,
```

### 4. Cross-Reference Related Items

```rust
/// Execute a passed proposal.
/// 
/// Can only be called after the proposal passes.
/// See [`CreateProposal`] for proposal creation.
/// See [`Vote`] for casting votes.
ExecuteProposal { proposal_id: u64 },
```

## Commit Documentation Changes

When updating contracts, always regenerate and commit docs:

```bash
cargo make docs
git add docs/
git commit -m "docs: update generated documentation"
```

## Documentation Checklist

- [ ] All message types have doc comments
- [ ] All struct fields are documented
- [ ] Constraints are documented (min/max, format)
- [ ] Default values are noted
- [ ] Error conditions are described
- [ ] Schemas generate without errors (`cargo make schema`)
- [ ] Docs generate without errors (`cargo make docs`)
- [ ] Changes committed with conventional commit message
