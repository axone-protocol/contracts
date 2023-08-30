# 📜 OKP4 Smart Contracts

> [CosmWasm](https://cosmwasm.com) contracts for the [OKP4 network](http://okp4.network).

[![release](https://img.shields.io/github/v/release/okp4/contracts?style=for-the-badge&logo=github)](https://github.com/okp4/contracts/releases)
[![build](https://img.shields.io/github/actions/workflow/status/okp4/contracts/build.yml?label=build&style=for-the-badge&logo=github)](https://github.com/okp4/contracts/actions/workflows/build.yml)
[![lint](https://img.shields.io/github/actions/workflow/status/okp4/contracts/lint.yml?label=lint&style=for-the-badge&logo=github)](https://github.com/okp4/contracts/actions/workflows/lint.yml)
[![test](https://img.shields.io/github/actions/workflow/status/okp4/contracts/test.yml?label=test&style=for-the-badge&logo=github)](https://github.com/okp4/contracts/actions/workflows/test.yml)
[![coverage](https://img.shields.io/codecov/c/github/okp4/contracts?style=for-the-badge)](https://app.codecov.io/gh/okp4/contracts)
[![conventional commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg?style=for-the-badge&logo=conventionalcommits)](https://conventionalcommits.org)
[![semantic-release](https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release-e10079.svg?style=for-the-badge)](https://github.com/semantic-release/semantic-release)
[![license](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg?style=for-the-badge)](https://opensource.org/licenses/BSD-3-Clause)

## ✨ Smart Contracts

This repository hosts Smart Contracts that are deployed on the [OKP4 network](http://okp4.network). But they are compatible with any
[Cosmos blockchains](https://cosmos.network/) that uses the [CosmWasm](https://cosmwasm.com/) framework.

### 🗄️ Storage oriented Smart Contracts

<table>
  <tr>
    <th>contract</th>
    <th>kind</th>
    <th>state</th>
    <th>description</th>
    <th>status</th>
  </tr>
  <tr>
    <td>
      <img src="etc/objectarium.webp" width="150" alt="objectarium" />
      <br />
      <a href="contracts/okp4-objectarium/README.md">objectarium</a>
    </td>
    <td><a href="https://en.wikipedia.org/wiki/Object_storage">object</a></td>
    <td><code>immutable</code></td>
    <td>Persists unstructured data on-chain.</td>
    <td>✅</td>
  </tr>
  <tr>
    <td>
      <img src="etc/cognitarium.webp" width="150" alt="cognitarium" />
      <br />
      <a href="contracts/okp4-cognitarium/README.md">cognitarium</a>
    </td>
    <td><a href="https://en.wikipedia.org/wiki/Triplestore">semantic</a></td>
    <td><code>mutable</code></td>
    <td>Persists semantic data on-chain.</td>
    <td>🚧</td>
  </tr>
</table>

### ⚖️ Sovereignty oriented Smart Contracts

<table>
  <tr>
    <th>contract</th>
    <th>state</th>
    <th>description</th>
    <th>status</th>
  </tr>
  <tr>
    <td>
     <img src="etc/law-stone.webp" width="150" alt="law-stone" />
     <br />
        <a href="contracts/okp4-law-stone/README.md">law&#8209;stone</a>
    </td>
    <td><code>immutable</code></td>
    <td>Interprets governances expressed as <a href="https://en.wikipedia.org/wiki/Prolog">Prolog</a> program.</td>
    <td>✅</td>
  </tr>
</table>

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
├── packages
│   └── <package>
│       ├── Cargo.toml
│       ├── examples
│       └── src
└─── Cargo.toml
```

## 🏗 Build

### Pre-requisites

Be sure you have the following tools installed:

- [Rust](https://www.rust-lang.org/tools/install) v1.69 or higher
- [cargo-make](https://github.com/sagiegurari/cargo-make) v0.36.3 or higher
- [Docker](https://docs.docker.com/get-docker/)
- [jq](https://stedolan.github.io/jq/download/) v1.6 or higher
- [npx](https://www.npmjs.com/package/npx) v8.19.2 or higher

And the following common [GNU Core utilities](https://en.wikipedia.org/wiki/List_of_GNU_Core_Utilities_commands):

- [shasum](https://linuxhint.com/shasum-linux/) v6.02 or higher
- [sed](https://www.gnu.org/software/sed/) v4.8 or higher

### Compiling Smart Contracts to Wasm

To compile the Smart Contracts to Wasm, just invoke the `wasm` goal of the makefile:

```sh
cargo make wasm
```

This will install the rust Wasm toolchain, if not already done, and start the process for compiling the Smart Contracts
to Wasm.

### Test Smart Contracts

The Smart Contracts are under unit testing. The tests can be launched with the following invocation:

```sh
cargo make test-coverage
```

## 🏓 Play with the Smart Contracts

The project comes with a set of convenient tasks to manage the Smart Contracts and the blockchain.
To see the list of available tasks, run the following command:

```sh
cargo make --list-all-steps | grep chain | sort | sed -r 's/chain(-?[a-zA-Z\-]*)/- `chain\1`/'
```

The list of available tasks for managing the chain is as follows:

- `chain` - Run the okp4d CLI using the chain's home directory under a Docker container.
- `chain-add-keys` - Add a set of predefined keys (recovered from the seed phrases) to the chain.
- `chain-clean` - Clean the chain data (⚠️ definitively)
- `chain-deploy-contract` - Deploy a specific contract to the chain. The contract must be compiled and the wasm file must be present in the artifacts directory (under target/wasm32-unknown-unknown/...).
- `chain-deploy-contracts` - Deploy all the available contracts to the chain (under target/wasm32-unknown-unknown/...).
- `chain-execute-contract` - Execute a command on a specific contract to the chain. The contract must be already deployed and instantiated.
- `chain-init-folder` - Initialize deploy folder to make sure scripts have the right permission (needed for linux)
- `chain-initialize` - Initialize the chain with a validator's key and a set of predefined keys. ⚠️ The home directory is cleaned before.
- `chain-inspect-contract` - Inspect a specific contract deployed to the chain.
- `chain-instantiate-contract` - Instantiate a specific contract to the chain. The contract must be already deployed.
- `chain-list-contracts` - List all the contracts deployed to the chain.
- `chain-logs` - Show the chain's container logs.
- `chain-query-contract` - Query a specific contract to the chain. The contract must be already deployed and instantiated.
- `chain-start` - Run the full node okp4d application using the chain's home directory under a Docker container.
- `chain-stop` - Stop the chain's container.

### Initialize the chain

To initialize the chain, just run:

```sh
cargo make chain-initialize
```

This will initialize the chain's home directory and create a validator's key and a set of predefined keys
(recovered from the seed phrases).

### Start the chain

To start the chain, just run:

```sh
cargo make chain-start
```

This will start the chain's container and run the full node wasmd application.

You can check the chain's logs with:

```sh
cargo make chain-logs
```

### Deploy the Smart Contracts

To deploy the Smart Contracts, just run:

```sh
cargo make chain-deploy-contracts
```

This will deploy all the available contracts to the chain. For this, the contracts must be compiled and the wasm files
must be present in the artifacts directory. See the [Build](#-build) section for more details.

Now, you can interact with the deployed smart contracts and test them out.

### Free execution of the CLI command

You can freely interact with the local chain by executing the following CLI command. This will execute the `okp4d`
binary
inside a Docker container with the `--home` argument pointing to the chain's home directory and using the same network
as
the chain's container. The arguments passed to the command will be directly passed to the `okp4d` binary.

```sh
cargo make chain <command>
```

For example, to check the status of the chain, just run:

```sh
cargo make chain status
```

### Stop the chain

To stop the chain, just run:

```sh
cargo make chain-stop
```

### Clean the chain

To clean the chain, just run:

```sh
cargo make chain-clean
```

⚠️ Please be cautious when running this command as it will completely clean the chain's home directory and the action is
irreversible.

## Develop

### Smart Contracts scaffolding

When developing a new Smart Contract, you can use the scaffolding to generate the Smart Contract's code.

#### Pre-requisites

Be sure you have the following tools installed:

- [ffizer](https://ffizer.github.io/ffizer/book/#install) v2.10.3 or higher

#### Generate the scaffolding

To generate the scaffolding, just run:

```sh
cargo make scaffold-smart-contract
```

Then, follow the instructions.

### Documentation

The documentation of the smart contracts must be committed to the repository. The documentation is generated from the
smart contracts' schema.

To generate the documentation follow the steps below.

#### Pre-requisites

Be sure you have the following tools installed:

- [Yarn](https://classic.yarnpkg.com/en/docs/install) v1.22.10 or higher

Then, install the dependencies:

```sh
yarn global add @adobe/jsonschema2md@7.1.5
```

#### Generate the documentation

To generate the documentation, just run:

```sh
cargo make schema
cargo make docs-generate
```

You'll find the generated documentation under the `docs` folder.

#### Commit the documentation

When developing a new contract, you should commit the generated documentation to the repository. For this, gerenate the
documentation and commit the changes:

```sh
git commit -am "docs: update generated documentation"
```

## Resources

- [CosmWasm Docs](https://docs.cosmwasm.com/)
- [OKP4 Whitepaper](https://docs.okp4.network/whitepaper/abstract)
- [OKP4 Blockchain](https://githhub.com/okp4/okp4d)

## You want to get involved? 😍

So you want to contribute? Great! ❤️ We appreciate any help you're willing to give. Don't hesitate to open issues and/or
submit pull requests.

Please check out OKP4 health files:

- [Contributing](https://github.com/okp4/.github/blob/main/CONTRIBUTING.md)
- [Code of conduct](https://github.com/okp4/.github/blob/main/CODE_OF_CONDUCT.md)
