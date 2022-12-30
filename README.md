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

### Prerequisites

Be sure you have [Rust](https://www.rust-lang.org/tools/install) properly installed with [cargo-make](https://github.com/sagiegurari/cargo-make).

```sh
$ cargo make --version
cargo-make 0.35.13
```

(your version may differ and that is fine)

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

## You want to get involved? 😍

So you want to contribute? Great! ❤️ We appreciate any help you're willing to give. Don't hesitate to open issues and/or submit pull requests.

Please check out OKP4 health files:

- [Contributing](https://github.com/okp4/.github/blob/main/CONTRIBUTING.md)
- [Code of conduct](https://github.com/okp4/.github/blob/main/CODE_OF_CONDUCT.md)
