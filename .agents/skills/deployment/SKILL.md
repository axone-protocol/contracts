---
name: deployment
description: Axone deployment workflows with cargo-make, cw-orch, and Abstract. Use when publishing modules, installing them on accounts, running local chain tasks, or inspecting deployments.
license: BSD-3-Clause
metadata:
  author: axone.xyz
  version: "1.0"
---

# Deployment

## Rule

Prefer repository tasks over ad hoc deployment commands.

## Environment Setup

### Configure Credentials

```bash
cp .env.example .env
```

Then set the relevant mnemonic values in `.env`.

The repository already defines:

- `LOCAL_MNEMONIC` for local development
- `TEST_MNEMONIC` for testnet deployment
- `MAIN_MNEMONIC` for mainnet deployment

## Supported Network IDs

Primary repository aliases:

- `local`
- `testnet`
- `mainnet`
- `axone-localnet`
- `axone-dendrite-2`
- `axone-1`

Other cw-orch-supported networks may also work when the scripts support them.

## Local Chain Operations

Use these tasks for local chain lifecycle:

```bash
cargo make chain-initialize
cargo make chain-start
cargo make chain-logs
cargo make chain-stop
cargo make clean-chain
```

Repository note:

- many deployment and inspection tasks already call `chain-start` automatically for local targets
- do not duplicate that manually unless you specifically need the chain running ahead of time

## Abstract Infrastructure

Deploy Abstract core contracts with:

```bash
cargo make deploy-abstract local
cargo make deploy-abstract testnet
```

Inspect the resulting deployment info with:

```bash
cargo make deploy-abstract-info local
cargo make deploy-abstract-info testnet
```

Treat `deploy-abstract-info` as the normal post-deploy verification step for infrastructure.

## Publishing a Contract

Publish a module with:

```bash
cargo make deploy-contract axone-gov local
cargo make deploy-contract axone-gov testnet
```

Repository behavior of `deploy-contract`:

- builds WASM if no suitable artifact is found
- copies built `.wasm` files into `artifacts/` for cw-orch lookup
- runs the contract publish script through `cargo make deploy-script publish ...`

Use `cargo make release-wasm` only when you explicitly need optimized production artifacts and checksums. The normal publish flow uses `cargo make wasm` when it needs to build.

## Installing a Module

Install a module on an Abstract account with:

```bash
cargo make deploy-install axone-gov local
cargo make deploy-install axone-gov testnet
```

The repository task also supports additional CLI-style arguments after the contract name. Keep using `cargo make deploy-install ...` instead of calling the binary directly unless you have a reason to bypass the task wrapper.

## Inspection and Debugging

Use the repository inspection tasks instead of raw chain commands when possible:

```bash
cargo make contract-list
cargo make contract-inspect <code_id>
cargo make contract-query <address> '{"query_msg": {}}'
cargo make contract-execute <address> '{"execute_msg": {}}'
```

For lower-level chain access:

```bash
cargo make chain status
cargo make chain query bank balances <address>
```

## Custom Deployment Scripts

The repository keeps publish and install entrypoints in each contract:

```text
contracts/<name>/src/bin/publish.rs
contracts/<name>/src/bin/install.rs
```

Use the wrapper task when running them:

```bash
cargo make deploy-script publish axone-gov local
cargo make deploy-script install axone-gov local
```

## Practical Order Of Operations

For a normal local flow:

1. `cargo make deploy-abstract local`
2. `cargo make deploy-abstract-info local`
3. `cargo make deploy-contract <contract> local`
4. `cargo make deploy-install <contract> local`
5. verify with `contract-query`, `contract-inspect`, or targeted chain queries
