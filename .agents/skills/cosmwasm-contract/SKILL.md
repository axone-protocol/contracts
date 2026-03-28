---
name: cosmwasm-contract
description: Axone contract structure and Abstract SDK patterns. Use when scaffolding or refactoring contracts, deciding layer boundaries, wiring AppContract entrypoints, or adding module metadata and replies.
license: BSD-3-Clause
metadata:
  author: axone.xyz
  version: "1.0"
---

# CosmWasm Contract Structure

## Companion Skills

Use `rust-contract-domain-modeling` for invariant-heavy domain design, `api-design` for message surfaces, `api-doc-comments` for schema-facing documentation, `rust-testing` for tests, and `rust-quality-gates` for validation gates.

## Repository Structure

The repository has:

- a workspace-level `Makefile.toml` for shared tasks
- a contract-level `Makefile.toml` in each contract for local helpers
- one directory per contract under `contracts/`

## Minimal Contract Skeleton

Start from this minimal shape:

```text
contracts/<contract-name>/
├── Cargo.toml
├── Makefile.toml
├── README.md
├── metadata.json
├── src/
│   ├── lib.rs
│   ├── contract.rs
│   ├── msg.rs
│   ├── state.rs
│   ├── error.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── instantiate.rs
│   │   ├── execute.rs
│   │   ├── query.rs
│   │   └── migrate.rs
│   └── bin/
│       ├── schema.rs
│       ├── publish.rs
│       └── install.rs
└── tests/
    └── integration.rs
```

Add extra layers only when they clarify boundaries.

## Optional Layers Already Used In This Repo

Current repository layers include:

- `domain/` for explicit business concepts and invariants
- `services/` for orchestration and environment-aware composition
- `gateway/` for external module interaction and protocol I/O
- `queries/` for query string / payload builders
- `replies/` for reply handlers

`axone-gov` is the reference example for the richer layered shape.

## Layer Boundaries

- `msg.rs` defines the public contract surface.
- `contract.rs` wires the `AppContract` entrypoints and module metadata.
- `handlers/` should stay thin: decode messages, call domain/services, build responses.
- `domain/` should own invariants and canonical representations.
- `services/` should coordinate enriched flows or external calls without becoming a second handler layer.
- `gateway/` should isolate external query / module plumbing.
- `state.rs` should persist and reconstruct values, not silently redefine business rules.

## `lib.rs` Pattern

Keep module identity constants explicit:

```rust
pub const AXONE_NAMESPACE: &str = "axone";
pub const CONTRACT_NAME: &str = "my-contract";
pub const CONTRACT_ID: &str = const_format::concatcp!(AXONE_NAMESPACE, ":", CONTRACT_NAME);
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
```

## `contract.rs` Pattern

The baseline `AppContract` wiring looks like this:

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

Repository-specific refinements:

- Use `Some(APP_METADATA_URL)` when the contract publishes metadata from a tagged GitHub URL.
- Add `.with_replies(...)` when the contract has reply-driven flows.
- Keep `.with_dependencies(&[])` explicit unless real module dependencies exist.
- Add the non-wasm `DependencyCreation` impl when the interface needs the standard installation path used in this repo.

## Message Wiring

Use `abstract_app::app_msg_types!(...)` in `msg.rs` when following the repository's Abstract app pattern.

Keep message docs and response typing aligned with:

- `api-design`
- `api-doc-comments`

## Error Design

Error enums should include the standard conversion layers used by the contract and then domain-specific errors.

Do not hide domain invariants in generic `StdError` messages if a dedicated error variant would make the failure explicit.

## Replies and Metadata

Two patterns matter in this repository:

- `metadata.json` is part of the contract deliverable and should stay aligned with the module identity.
- reply handlers are normal when contract setup involves follow-up callbacks or submessage completion.

Treat both as first-class contract structure, not optional afterthoughts.

## Construction Rules

- Put invariant checks close to domain object creation.
- Keep handler branches short; move validation-heavy logic out of handlers.
- Prefer explicit constructors such as `new`, `try_new`, or `from_state`.
- Reuse canonical domain types rather than passing partially validated strings across layers.
