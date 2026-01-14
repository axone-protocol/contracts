---
name: deployment
description: Guide for deploying CosmWasm contracts using cw-orchestrator and Abstract SDK. Use when publishing contracts, deploying to networks, or managing local chains.
license: BSD-3-Clause
metadata:
  author: axone.xyz
  version: "1.0"
---

# Contract Deployment with cw-orchestrator

This skill helps you deploy Axone protocol smart contracts using cw-orchestrator and the Abstract SDK framework.

## When to use this skill

Use this skill when you need to:

- Deploy contracts to local, testnet, or mainnet
- Publish modules to Abstract's registry
- Install modules on Abstract Accounts
- Manage local development chains
- Configure deployment environments

## Environment Setup

### Configure Credentials

```bash
# Copy example environment file
cp .env.example .env

# Edit .env with your mnemonics
```

**`.env` configuration:**

```env
# Local development (safe to use defaults)
LOCAL_MNEMONIC="your local dev mnemonic"

# Testnet deployments
TEST_MNEMONIC="your testnet wallet mnemonic"

# Mainnet deployments (⚠️ keep private!)
MAIN_MNEMONIC="your mainnet wallet mnemonic"
```

### Supported Networks

| Network ID | Chain | Description |
| - | - | - |
| `local` | axone-localnet | Local Docker chain |
| `testnet` | axone-dentrite-1 | Axone testnet |
| `mainnet` | axone-1 | Axone mainnet |
| `axone-localnet` | axone-localnet | Alias for local |

Also supports standard cw-orch networks: `uni-6`, `osmo-test-5`, `juno-1`, etc.

## Deployment Workflow

### Step 1: Start Local Chain (for local development)

```bash
# Initialize chain (first time only)
cargo make chain-initialize

# Start the chain
cargo make chain-start

# View logs
cargo make chain-logs
```

### Step 2: Deploy Abstract Infrastructure

Deploy Abstract's core contracts (account factory, module factory, version control):

```bash
# Deploy to local
cargo make deploy-abstract local

# Deploy to testnet
cargo make deploy-abstract testnet

# Deploy to multiple networks
cargo make deploy-abstract local testnet
```

> **Note:** Only needed once per network.

### Step 3: Publish Your Contract

Upload WASM and register with Abstract's version control:

```bash
# Publish to local
cargo make deploy-contract axone-gov local

# Publish to testnet
cargo make deploy-contract axone-gov testnet

# Publish to multiple networks
cargo make deploy-contract axone-gov local testnet
```

This step:

1. Builds WASM if not present
2. Uploads the contract to the chain
3. Registers it with Abstract's module registry

### Step 4: Install on Abstract Account

Instantiate and install the module on an Abstract Account:

```bash
# Install on local
cargo make deploy-install axone-gov local

# Install on testnet  
cargo make deploy-install axone-gov testnet
```

This creates an Abstract Account (if needed) and installs your contract as a module.

## Quick Deployment (All-in-One)

For local development:

```bash
cargo make chain-start && \
cargo make deploy-abstract local && \
cargo make deploy-contract axone-gov local && \
cargo make deploy-install axone-gov local
```

## Chain Management

### Starting and Stopping

```bash
# Start chain (runs in Docker)
cargo make chain-start

# Stop chain
cargo make chain-stop

# Clean chain data (⚠️ irreversible)
cargo make clean-chain
```

### Interacting with Chain

```bash
# Execute arbitrary axoned commands
cargo make chain <command>

# Example: Check chain status
cargo make chain status

# Example: Query account
cargo make chain query bank balances <address>
```

### Contract Inspection

```bash
# List all deployed contracts
cargo make contract-list

# Inspect specific contract by code ID
cargo make contract-inspect <code_id>

# Query contract state
cargo make contract-query <address> '{"query_msg": {}}'

# Execute on contract
cargo make contract-execute <address> '{"execute_msg": {}}'
```

## Building for Deployment

```bash
# Development build
cargo make wasm

# Optimized release build (for production)
cargo make release-wasm

# Verify WASM validity
cargo make check-contracts
```

## Custom Deployment Scripts

For advanced deployment, use the binary scripts in each contract:

```rust
// contracts/<name>/src/bin/publish.rs
// contracts/<name>/src/bin/install.rs
```

Run with:

```bash
cargo make deploy-script publish axone-gov local
cargo make deploy-script install axone-gov local
```

## Troubleshooting

### "Chain not running"

```bash
cargo make chain-start
```

### "Abstract not deployed"

```bash
cargo make deploy-abstract <network>
```

### "WASM not found"

```bash
cargo make wasm
```

### "Insufficient funds"

For local: Balances are pre-configured.
For testnet: Use the Axone faucet.

## Deployment Checklist

- [ ] Environment file configured (`.env`)
- [ ] Chain running (for local)
- [ ] Abstract infrastructure deployed
- [ ] Contract WASM built
- [ ] Contract published to registry
- [ ] Module installed on Abstract Account
- [ ] Deployment verified with queries
