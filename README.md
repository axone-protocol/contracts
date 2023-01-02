# 📜 OKP4 Smart Contracts

> [CosmWasm](https://cosmwasm.com) contracts for the [OKP4 network](http://okp4.network).

[![release](https://img.shields.io/github/v/release/okp4/contracts?style=for-the-badge&logo=github)](https://github.com/okp4/contracts/releases)
[![build](https://img.shields.io/github/actions/workflow/status/okp4/contracts/build.yml?label=build&style=for-the-badge&logo=github)](https://github.com/okp4/contracts/actions/workflows/build.yml)
[![lint](https://img.shields.io/github/actions/workflow/status/okp4/contracts/lint.yml?label=lint&style=for-the-badge&logo=github)](https://github.com/okp4/contracts/actions/workflows/lint.yml)
[![test](https://img.shields.io/github/actions/workflow/status/okp4/contracts/test.yml?label=test&style=for-the-badge&logo=github)](https://github.com/okp4/contracts/actions/workflows/test.yml)
[![conventional commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg?style=for-the-badge&logo=conventionalcommits)](https://conventionalcommits.org)
[![license](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg?style=for-the-badge)](https://opensource.org/licenses/BSD-3-Clause)

## ✨ Smart Contracts

This repository contains the Smart Contracts deployed on the [OKP4 network](http://okp4.network) and running on [CosmWasm](https://cosmwasm.com).

The list of currently implemented contracts is as follows:

- [cw-template](./contracts/cw-template/README.md): base smart contract to start coding in the OKP4 blockchain.

## 🗂 Directory Structure

The project is structured around a set of Rust workspaces, each defining a smart contract.

```text
.
├── contracts
│   └── <contract>
│       ├── Cargo.toml
│       ├── examples
│       ├── schema
│       └── src
└─── Cargo.toml
```

## 🏗 Build

### Pre-requisites

Be sure you have the following tools installed:

- [Rust](https://www.rust-lang.org/tools/install) v1.63.0 or higher
- [cargo-make](https://github.com/sagiegurari/cargo-make) v0.36.3 or higher
- [Docker](https://docs.docker.com/get-docker/)

### Compiling Smart Contracts to Wasm

To compile the Smart Contracts to Wasm, just invoke the `wasm` goal of the makefile:

```sh
cargo make wasm
```

This will install the rust Wasm toolchain, if not already done, and start the process for compiling the Smart Contracts to Wasm.

### Test Smart Contracts

The Smart Contracts are under unit testing. The tests can be launched with the following invocation:

```sh
cargo make test-coverage
```

## 🏓 Play with the Smart Contracts

The project comes with a set of convenient tasks to manage the Smart Contracts and the blockchain.
To see the list of available tasks, run the following command:

```sh
cargo make --list-all-steps | grep chain- | sort
```

The list of available tasks is as follows:

```text
chain-add-keys - Add a set of predefined keys (recovered from the seed phrases) to the chain.
chain-clean - Clean the chain data (⚠️ definitively)
chain-deploy-contract - Deploy a specific contract to the chain. The contract must be compiled and the wasm file must be present in the artifacts directory (under target/wasm32-unknown-unknown/...).
chain-deploy-contracts - Deploy all the available contracts to the chain (under target/wasm32-unknown-unknown/...).
chain-init-folder - Initialize deploy folder to make sure scripts have the right permission (needed for linux)
chain-initialize - Initialize the chain with a validator's key and a set of predefined keys. ⚠️ The home directory is cleaned before.
chain-inspect-contract - Inspect a specific contract deployed to the chain.
chain-list-contracts - List all the contracts deployed to the chain.
chain-logs - Show the chain's container logs.
chain-start - Run the full node wasmd application using the chain's home directory under a Docker container.
chain-stop - Stop the chain's container.
```

## Initialize the chain

To initialize the chain, just run:

```sh
cargo make chain-initialize
```

This will initialize the chain's home directory and create a validator's key and a set of predefined keys
(recovered from the seed phrases).

## Start the chain

To start the chain, just run:

```sh
cargo make chain-start
```

This will start the chain's container and run the full node wasmd application.

You can check the chain's logs with:

```sh
cargo make chain-logs
```

## Deploy the Smart Contracts

To deploy the Smart Contracts, just run:

```sh
cargo make chain-deploy-contracts
```

This will deploy all the available contracts to the chain. For this, the contracts must be compiled and the wasm files
must be present in the artifacts directory. See the [Build](#-build) section for more details.

Now, you can interact with the deployed smart contracts and test them out.

## Stop the chain

To stop the chain, just run:

```sh
cargo make chain-stop
```

## Clean the chain

To clean the chain, just run:

```sh
cargo make chain-clean
```

Please be cautious when running this command as it will completely clean the chain's home directory and the action is
irreversible.

## Resources

- [CosmWasm Docs](https://docs.cosmwasm.com/)
- [OKP4 Whitepaper](https://docs.okp4.network/whitepaper/abstract)
- [OKP4 Blockchain](https://githhub.com/okp4/okp4d)

## You want to get involved? 😍

So you want to contribute? Great! ❤️ We appreciate any help you're willing to give. Don't hesitate to open issues and/or submit pull requests.

Please check out OKP4 health files:

- [Contributing](https://github.com/okp4/.github/blob/main/CONTRIBUTING.md)
- [Code of conduct](https://github.com/okp4/.github/blob/main/CODE_OF_CONDUCT.md)
