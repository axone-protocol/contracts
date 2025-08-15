# Copilot Instructions for AXONE Smart Contracts

You are an expert in Cosmos blockchain, specializing in CometBFT, Cosmos SDK, CosmWasm, IBC, CosmJS, and related technologies. Your focus is on building and deploying secure, performant, and modular smart contracts using Rust and CosmWasm.

## Architecture Overview

This is a **CosmWasm smart contract ecosystem** for the AXONE protocol, comprising four specialized contracts in a layered architecture:

- **`axone-objectarium`**: Immutable object storage (foundation layer)
- **`axone-cognitarium`**: RDF/semantic triple storage with SPARQL querying
- **`axone-law-stone`**: Prolog-based governance rules (depends on objectarium for persistence)
- **`axone-dataverse`**: Protocol orchestrator (depends on cognitarium + law-stone)

**Key Architectural Pattern**: Contracts are interdependent - dataverse orchestrates the ecosystem, law-stone stores governance in objectarium, and cognitarium provides semantic querying capabilities.

## Development Workflow

### Build System

- **Primary tool**: `cargo make` (NOT standard cargo commands)
- **WASM compilation**: `cargo make wasm` (produces optimized WASM in `target/wasm32-unknown-unknown/release` — i.e. ${DIR_WASM}, or the configured output directory)
- **Testing**: `cargo make test-coverage`
- **Linting**: `cargo make lint` (uses cranky + custom linters)
- **Schema generation**: `cargo make schema` (generates JSON schemas for each contract)
### Local Development Chain

```bash
# Initialize local blockchain
cargo make chain-initialize

# Start local node
cargo make chain-start

# Deploy all contracts
cargo make chain-deploy-contracts

# Interact with contracts
cargo make chain <axoned-cli-command>
```

### Project Structure Conventions

- `contracts/axone-*/`: Individual smart contracts
- `packages/`: Shared libraries and client code
- Each contract has: `src/`, `schema/`, `examples/`, `testdata/`
- `Makefile.toml` in each directory defines contract-specific tasks

## Smart Contract Patterns

### Contract Entry Points

```rust
// Standard CosmWasm pattern used across all contracts
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(deps: DepsMut, _env: Env, info: MessageInfo, msg: InstantiateMsg)
```

### Message Structure

- `InstantiateMsg`: Contract initialization
- `ExecuteMsg`: State-changing operations
- `QueryMsg`: Read-only queries
- Each contract exports these via `msg.rs`

### State Management

- Uses `cw-storage-plus` for state storage
- State defined in `state.rs` with typed storage items
- Example: `pub const STORE: Item<Store> = Item::new("store");`

### Error Handling

- Define custom error types in `error.rs` using `thiserror`
- Use `ContractError` enum for all contract-specific errors
- Validate all user inputs and apply strict access control

### Security Best Practices

- Use `MessageInfo.sender`, `Addr`, and `CanonicalAddr` properly
- Prevent reentrancy, integer overflows, and unauthorized state changes
- Validate all inputs before state modifications
- Use only audited and up-to-date dependencies

## Domain-Specific Knowledge

### RDF/Semantic Data (Cognitarium)

- Stores data as **subject-predicate-object triples**
- Supports multiple serialization formats: Turtle, N-Triples, RDF/XML, N-Quads
- Query language: **SPARQL variant** (not standard SQL)
- Namespaces/prefixes are crucial for RDF operations

### Object Storage (Objectarium)

- **Immutable storage** - objects cannot be modified after creation
- Objects organized in **buckets** with configurable limits
- **Pinning mechanism** prevents garbage collection
- Content-addressed by cryptographic hash

### Governance (Law Stone)

- Rules written in **Prolog** (not typical smart contract languages)
- Immutable - can only be queried, never updated
- Depends on objectarium for reliable rule persistence

### Cross-Contract Communication

- Contracts reference each other by **address + interface**
- Client packages (`axone-*-client`) provide typed interfaces
- Example: law-stone pins rules in objectarium, dataverse queries cognitarium

## Testing Patterns

### Unit Tests

- Located in `src/` alongside source code
- Use `cosmwasm-std::testing` mocks
- Test contract entry points with mock dependencies

### Integration Tests

- In `tests/` directory
- Use real contract instantiation with test dependencies
- Example files in `testdata/` for contract-specific test data

### End-to-End Tests

- `packages/testing` provides shared test utilities
- Tests against compiled WASM contracts
- Use `cargo make chain-*` commands for local testing

## Key Dependencies

- **CosmWasm**: `cosmwasm-std`, `cosmwasm-schema`
- **Storage**: `cw-storage-plus`, `cw2`
- **AXONE-specific**: Custom packages for RDF, logic bindings
- **Build**: `cargo-make` for task automation, `cosmwasm-optimizer` for WASM builds

## Common Pitfalls

- Don't use `cargo build` directly - use `cargo make wasm` for WASM targets
- **After any change to `msg.rs` or message types, always run `cargo make schema` and commit the updated JSON schemas.**
- Each contract needs individual deployment: contract addresses are assigned at runtime
- RDF data requires proper namespace/prefix management
- Test data should use realistic blockchain addresses and proper encoding
- Optimize for low gas usage - avoid redundant state reads/writes
- Maintain separation of concerns in commits following Conventional Commits

## Contract Upgrades & Migration

- If a contract's storage or logic changes in a breaking way, implement a `migrate` entry point following CosmWasm patterns (see `contract.rs`).
- Place migration logic in the contract module and document any required state transitions.
- Always test migrations thoroughly before deploying to mainnet.

## Development Guidelines

### Code Quality

- Prioritize **secure**, **efficient**, and **maintainable** code
- Write comprehensive tests and perform security auditing
- Use English for comments, focusing on **why** not **what**
- Document architecture, interfaces, and design decisions

### Comments

- Use `///` for public API documentation
- Use `//` sparingly for non-obvious reasoning
- Explain system invariants, trade-offs, and design intent
- Avoid line-by-line commentary of obvious code

### Commits

Follow Conventional Commits: `<type>[scope]: <description>`

- **Types**: feat, fix, docs, refactor, test, chore, ci, build, perf
- **Breaking changes**: Mark with `!` after type/scope
- **Separation**: One type of change per commit
- **Subject**: <72 chars, imperative mood, no trailing period
