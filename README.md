# 📜 AXONE Smart Contracts

> [CosmWasm](https://cosmwasm.com) contracts for the [AXONE network](http://axone.xyz).

[![build](https://img.shields.io/github/actions/workflow/status/axone-protocol/contracts/build.yml?label=build&style=for-the-badge&logo=github)](https://github.com/axone-protocol/contracts/actions/workflows/build.yml)
[![lint](https://img.shields.io/github/actions/workflow/status/axone-protocol/contracts/lint.yml?label=lint&style=for-the-badge&logo=github)](https://github.com/axone-protocol/contracts/actions/workflows/lint.yml)
[![test](https://img.shields.io/github/actions/workflow/status/axone-protocol/contracts/test.yml?label=test&style=for-the-badge&logo=github)](https://github.com/axone-protocol/contracts/actions/workflows/test.yml)
[![coverage](https://img.shields.io/codecov/c/github/axone-protocol/contracts?style=for-the-badge)](https://app.codecov.io/gh/axone-protocol/contracts)

[![built with cargo-make](https://img.shields.io/badge/build_with-cargo_make-d39f59?style=for-the-badge)](https://sagiegurari.github.io/cargo-make)
[![conventional commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg?style=for-the-badge&logo=conventionalcommits)](https://conventionalcommits.org)
[![semantic-release](https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release-e10079.svg?style=for-the-badge)](https://github.com/semantic-release/semantic-release)
[![license](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg?style=for-the-badge)](https://opensource.org/licenses/BSD-3-Clause)

## ✨ Smart Contracts

This repository hosts the smart contracts at the heart of the AXONE ecosystem, powering on-chain governance and resource management on the [AXONE network](http://axone.xyz).

Built on **[CosmWasm](https://cosmwasm.com/)** and the **[Abstract SDK](https://abstract.money/)**, these contracts constitute the **functional core** of the protocol. They act as the on-chain operating system, responsible for **orchestrating off-chain resources**, **empowering sovereign agents**, and **enforcing verifiable governance**.

This foundation allows AXONE to go beyond simple transactions, enabling the execution of complex agreements and logic-based interactions directly on the blockchain.

### 🏛️ Governance

> Smart Contracts designed to enable on-chain decision making, rule enforcement and organizational structure.

<table>
  <tr>
    <th rowspan="4" width="30%">
      <div align="center">
        <img alt="axone-gov logo" src="contracts/axone-gov/axone-gov-card.webp" width="150px"/>
        <b>axone-gov</b>
      </div>
      <br/>
      <sub><a href="contracts/axone-gov/README.md">→ Tech documentation</a></sub>
    </th>
  </tr>
  <tr>
    <td>
      <a href="#-maturity">
        <img alt="status: intermediate" src="https://img.shields.io/badge/status-🐣-black?style=for-the-badge" />
      </a>
      <a href="https://crates.io/crates/axone-gov" target="_blank">
        <img alt="crates" src="https://img.shields.io/crates/v/axone-gov.svg?style=for-the-badge&color=orange"/>
      </a>
    </td>
  </tr>
  <tr>
    <td>
      <p>
        The <b>Governance Engine</b> of the protocol. It attaches governance capabilities to any resource represented by an <a href="https://docs.abstract.money/">Abstract Account (AA)</a>.
      </p>
      <p>
        It utilizes <b>Prolog constitutions</b> to strictly define and enforce governance rules, allowing for highly flexible and programmable organizational logic.
      </p>
    </td>
  </tr>
  <tr>
    <td>
      <img alt="type: governance" src="https://img.shields.io/badge/type-🏛️%20governance-darkslateblue?style=for-the-badge" />
      <a href="https://en.wikipedia.org/wiki/Prolog" target="_blank">
        <img alt="kind: logic" src="https://img.shields.io/badge/kind-🧠%20logic-moccasin?style=for-the-badge" />
      </a>
      <img alt="state: mutable" src="https://img.shields.io/badge/state-✏️%20mutable-lavender?style=for-the-badge" />
    </td>
  </tr>
</table>

## 🥚 Maturity

The maturity of each contract is indicated by the following emojis.

### 💥 - Genesis Stage

This stage represents the initial spark or idea that leads to the development of the Smart Contract.

### 🥚 - Initial Stage

The Smart Contract is still in its infancy, with basic functionalities just being implemented. It is still under development, and its evolution may lead to breaking changes. It is not recommended for production use.

### 🐣 - Intermediate Stage

The Smart Contract has undergone a number of tests and improvements, and presents a coherent functional package, but may still lack some advanced features or optimizations. It can be used in production in testnets.

### 🐥 - Mature Stage

The Smart Contract is fully developed and tested. It's considered safe for production use, offering robust features and optimized performance.

## 🗂 Directory Structure

The project is structured around a set of Rust workspaces, each defining a Smart Contract.

```text
.
├── contracts
│   └── <contract>
│       ├── Cargo.toml
│       ├── examples
│       ├── schema
│       └── src
├── packages
│   └── <package>
│       ├── Cargo.toml
│       ├── examples
│       └── src
└─── Cargo.toml
```

## 🏗 Build

### 🔨 Pre-requisites

Be sure you have the following tools installed:

- [Rust](https://www.rust-lang.org/tools/install) v1.81 or higher
- [cargo-make](https://github.com/sagiegurari/cargo-make) v0.36.3 or higher
- [Docker](https://docs.docker.com/get-docker/)
- [jq](https://stedolan.github.io/jq/download/) v1.6 or higher
- [npx](https://www.npmjs.com/package/npx) v8.19.2 or higher

And the following common [GNU Core utilities](https://en.wikipedia.org/wiki/List_of_GNU_Core_Utilities_commands):

- [shasum](https://linuxhint.com/shasum-linux/) v6.02 or higher
- [sed](https://www.gnu.org/software/sed/) v4.8 or higher

### 🔧 Environment setup

For deployment to networks (testnet/mainnet), copy the example environment file and configure your credentials:

```sh
cp .env.example .env
```

Then edit `.env` and add your deployment wallet mnemonics:

```env
# For testnet deployments
TEST_MNEMONIC="your testnet wallet mnemonic here"

# For mainnet deployments (⚠️ keep this private!)
MAIN_MNEMONIC="your mainnet wallet mnemonic here"
```

For local development, the default `LOCAL_MNEMONIC` in `.env.example` is safe to use.

### �🛠️ Available Tasks

The project uses [cargo-make](https://github.com/sagiegurari/cargo-make) to manage common development tasks. Here are the main tasks available:

<!-- TASKS -->
```text
Build
----------
build - No Description.
release-wasm - Build optimized wasm using CosmWasm optimizer and provide checksums
schema - Generate JSON schemas for all contracts.
wasm - No Description.

Chain Management
----------
chain - Run the axoned CLI using the chain's home directory under a Docker container.
chain-add-keys - Add a set of predefined keys (recovered from the seed phrases) to the chain.
chain-init-folder - Initialize deploy folder to make sure scripts have the right permission (needed for linux)
chain-initialize - Initialize the chain with a validator's key and a set of predefined keys. ⚠️ The home directory is cleaned before.
chain-logs - Show the chain's container logs.
chain-start - Run the full node axoned application using the chain's home directory under a Docker container.
chain-stop - Stop the chain's container.

Cleanup
----------
clean - Clean all artifacts (cargo, docs, and chain).
clean-cargo - Clean cargo build artifacts.
clean-chain - Clean the chain data (⚠️ definitively).
clean-docs - Clean documentation folder.

Code Quality
----------
format - Format all files (Rust and TOML).
format-rust - Format rust sources files. (rustfmt provided by rust-toolchain.toml)
format-toml - Format toml file
lint - Check all linting (Rust, Cargo, and TOML).
lint-cargo - Check all Cargo linting (toml files and dependencies).
lint-cargo-deps - Check for unused dependencies.
lint-cargo-toml - Check lint of all Cargo.toml files.
lint-cargo-toml-file - Check lint of the given toml file
lint-rust - Check all Rust linting (formatting and clippy).
lint-rust-clippy - Check lint of all sources files (clippy via rust-toolchain.toml).
lint-rust-format - Check formatting and derives order (rustfmt via rust-toolchain.toml).
lint-toml - Check lint of all toml files.

Contract Inspection
----------
contract-inspect - Inspect a specific contract deployed to the chain.
contract-list - List all the contracts deployed to the chain.

Contract Interaction
----------
contract-execute - Execute a command on a specific contract. The contract must be already deployed and instantiated.
contract-query - Query a specific contract. The contract must be already deployed and instantiated.

Deployment
----------
deploy-abstract - Deploy Abstract infrastructure to specified networks. Usage: cargo make deploy-abstract <network-ids...>
deploy-contract - Publish a contract to Abstract on specified networks. Usage: cargo make deploy-contract <contract-name> <network-ids...>
deploy-install - Install a module on an Abstract Account. Usage: cargo make deploy-install <contract-name> <network-ids...>
deploy-script - Run a contract deployment script. Usage: cargo make deploy-script <script> <package> <network-ids...>

Development Tools
----------
install-cargo-binstall - Install cargo-binstall if not already available
install-cargo-hack - No Description.
install-cargo-machete - No Description.
install-cargo-sort-derives - No Description.
install-cargo-toml-lint - No Description.
install-cargo-workspaces - No Description.
install-cosmwasm-check - No Description.
install-cranky - No Description.
install-dev-tools - Install all required development tools.
install-llvm-cov - No Description.
install-taplo-cli - No Description.

Documentation
----------
docs - Generate documentation
readme - Update README with help output

Help
----------
help - Display available tasks [aliases: default]

Publishing
----------
publish-crates - Publish all crates to the registry. Requires CARGO_REGISTRY_TOKEN to be set.

Testing
----------
test - Run all tests.
test-coverage - Run tests with coverage reporting.
test-unit - Run all unit tests.

Verification
----------
check - Check all requirements (prerequisites and contracts).
check-awk - Check awk is installed
check-contracts - Check WASM contracts validity.
check-jq - Check jq is installed (version 1.7 or higher, but below 2.0)
check-npx - Check npx is installed
check-perl - Check perl is installed
check-prerequisites - Check all the prerequisites are installed.
```
<!-- TASKS -->

### 🔧 Compiling Smart Contracts to Wasm

To compile the Smart Contracts to Wasm, just invoke the `wasm` goal of the makefile:

```sh
cargo make wasm
```

This will install the rust Wasm toolchain, if not already done, and start the process for compiling the Smart Contracts
to Wasm.

## ✅ Test Smart Contracts

The Smart Contracts are under unit testing. The tests can be launched with the following invocation:

```sh
cargo make test-coverage
```

## 🏓 Play with the Smart Contracts

The project comes with a set of convenient tasks to manage the Smart Contracts and the blockchain.

### 🚀 Initialize the chain

To initialize the chain, just run:

```sh
cargo make chain-initialize
```

This will initialize the chain's home directory and create a validator's key and a set of predefined keys
(recovered from the seed phrases).

### 🟢 Start the chain

To start the chain, just run:

```sh
cargo make chain-start
```

Note: the default Docker image used by the tasks points to the latest released `axoned` version configured in `Makefile.toml`.

To temporarily run a different `axoned` image (for example to test a newer release), set the variable when invoking `cargo make`:

```sh
# override just for this run
cargo make --env DOCKER_IMAGE_AXONEPROTOCOL_AXONED=axoneprotocol/axoned:13.1.0 chain-start
```

This will start the chain's container and run the full node `axoned` binary inside that image.

### 🔍 Viewing chain logs

Run this to follow the chain container logs in real time:

```sh
cargo make chain-logs
```

### 🛳 Deploy the Smart Contracts

The Smart Contracts in this repository are designed to work with the [Abstract framework](https://abstract.money/), which provides a modular application layer for Cosmos chains. The deployment process involves three main steps:

**Prerequisites:** Make sure you've configured your `.env` file with the appropriate mnemonics for your target network (see [Environment Setup](#-environment-setup)).

#### 1️⃣ Deploy Abstract Infrastructure

First, deploy the Abstract framework infrastructure (account factory, module factory, version control, etc.) to your target network:

```sh
cargo make deploy-abstract local
```

This command deploys the entire Abstract infrastructure to the specified network. You only need to do this once per network.

**Supported networks:** `local`, `testnet`, `mainnet`, `axone-localnet`, `axone-dentrite-1`, `axone-1`.

#### 2️⃣ Publish Your Contracts

Once the infrastructure is deployed, publish your smart contracts to Abstract's on-chain registry:

```sh
cargo make deploy-contract axone-gov local
```

This uploads your contract's WASM binary and registers it with Abstract's version control system. The contract becomes available for installation but is not yet instantiated.

You can publish to multiple networks at once:

```sh
cargo make deploy-contract axone-gov local testnet
```

#### 3️⃣ Install on an Abstract Account

Finally, install and instantiate your contract on an Abstract Account (which acts as a modular smart contract wallet):

```sh
cargo make deploy-install axone-gov local
```

This creates an Abstract Account (if needed) and installs your contract as a module on that account. The contract is now fully deployed and operational.

#### 🎯 All-in-One Deployment

For local development, you can chain these commands together:

```sh
cargo make chain-start && \
cargo make deploy-abstract local && \
cargo make deploy-contract axone-gov local && \
cargo make deploy-install axone-gov local
```

**Note:** Contracts must be compiled first. If WASM files are not found, the `deploy-contract` task will automatically build them. See the [Build](#-build) section for more details.

Now, you can interact with the deployed Smart Contracts and test them out.

### 🎮 Free execution of the CLI command

You can freely interact with the local chain by executing the following CLI command. This will execute the `axoned`
binary
inside a Docker container with the `--home` argument pointing to the chain's home directory and using the same network
as
the chain's container. The arguments passed to the command will be directly passed to the `axoned` binary.

```sh
cargo make chain <command>
```

For example, to check the status of the chain, just run:

```sh
cargo make chain status
```

### 🔴 Stop the chain

To stop the chain, just run:

```sh
cargo make chain-stop
```

### 🧼 Clean the chain

To clean the chain, just run:

```sh
cargo make clean-chain
```

⚠️ Please be cautious when running this command as it will completely clean the chain's home directory and the action is
irreversible.

## 📚 Documentation

The documentation of the Smart Contracts must be committed to the repository. The documentation is generated from the
Smart Contracts' schema.

To generate the documentation follow the steps below.

### 🔨 Documentation Pre-requisites

Be sure you have the following tools installed:

- [Yarn](https://classic.yarnpkg.com/en/docs/install) v1.22.10 or higher

Then, install the dependencies:

```sh
yarn global add @adobe/jsonschema2md@7.1.5
```

### 🖨 Generate the documentation

To generate the documentation, just run:

```sh
cargo make schema
cargo make docs
```

You'll find the generated documentation under the `docs` folder.

### 🗒 Commit the documentation

When developing a new contract, you should commit the generated documentation to the repository. For this, generate the
documentation and commit the changes:

```sh
git commit -am "docs: update generated documentation"
```

## 🍀 Quality assurance approach

⛓ - The enforcement of stringent rules, monitored by a linter ([Clippy](https://github.com/rust-lang/rust-clippy)) within the Github CI environment.

🤖 - A high level of code coverage through systematic unit testing.

💫 - Future considerations for additional testing approaches, such as fuzzy testing or end-to-end testing, to further enhance quality.

## 🛡️ Audit

| Date | Auditor | Version | Report |
|---|---|---|---|
| 2024/08/08 | [BlockApex](https://blockapex.io/) | [0cae9ec (v6.0.0)](https://github.com/axone-protocol/contracts/tree/0cae9ecf24c4ded86abecd34aec2303e82413672) | [Axone Smart Contract Audit Report.pdf](https://github.com/BlockApex/Audit-Reports/blob/15d8765ac45b4a83bb2f1446fc9bf869c123f8d2/Axone%20Smart%20Contract%20Audit%20Report.pdf)|

## 📘 Resources

- [CosmWasm Docs](https://docs.cosmwasm.com/)
- [AXONE Whitepaper](https://docs.axone.xyz/whitepaper/abstract)
- [AXONE Blockchain](https://github.com/axone-protocol/axoned)

## 🧭 Previous implementation

Looking for the earlier contracts? The last archived release is available here:
👉 [v8.0.0 release](https://github.com/axone-protocol/contracts/releases/v8.0.0).

## You want to get involved? 😍

So you want to contribute? Great! ❤️ We appreciate any help you're willing to give. Don't hesitate to open issues and/or
submit pull requests.

We believe that collaboration is key to the success of the AXONE project. Join our Community discussions on the [Community Repository](https://github.com/axone-protocol/community) to:

- Engage in conversations with peers and experts.
- Share your insights and experiences with AXONE.
- Learn from others and expand your knowledge of the protocol.

The Community Repository serves as a hub for discussions, questions, and knowledge-sharing related to AXONE. We encourage you to actively participate and contribute to the growth of our community.

Please check out AXONE health files:

- [Contributing](https://github.com/axone-protocol/.github/blob/main/CONTRIBUTING.md)
- [Code of conduct](https://github.com/axone-protocol/.github/blob/main/CODE_OF_CONDUCT.md)
