# Rust Template

> Template for Rust projects @okp4.

[![version](https://img.shields.io/github/v/release/okp4/template-rust?style=for-the-badge)](https://github.com/okp4/template-rust/releases)
[![build](https://img.shields.io/github/workflow/status/okp4/template-rust/Build?label=build&style=for-the-badge)](https://github.com/okp4/template-rust/actions/workflows/build.yml)
[![lint](https://img.shields.io/github/workflow/status/okp4/template-rust/Lint?label=lint&style=for-the-badge)](https://github.com/okp4/template-rust/actions/workflows/lint.yml)
[![test](https://img.shields.io/github/workflow/status/okp4/template-rust/Test?label=test&style=for-the-badge)](https://github.com/okp4/template-rust/actions/workflows/test.yml)
[![codecov](https://img.shields.io/codecov/c/github/okp4/template-rust?style=for-the-badge&token=K5CYM8TQQY)](https://codecov.io/gh/okp4/template-rust)
[![conventional commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg?style=for-the-badge)](https://conventionalcommits.org)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg?style=for-the-badge)](https://opensource.org/licenses/BSD-3-Clause)

## Purpose & Philosophy

This repository holds the template for building Rust projects with a consistent set of standards accross all [OKP4](https://github.com/okp4) projects. We are convinced that the quality of the code depends on clear and consistent coding conventions, with an automated enforcement (CI).

This way, the template promotes:

- the use of [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/), [semantic versionning](https://semver.org/) and [semantic releasing](https://github.com/cycjimmy/semantic-release-action) which automates the whole package release workflow including: determining the next version number, generating the release notes, and publishing the artifacts (project tarball, docker images, etc.)
- unit testing
- linting via [rust-clippy](https://github.com/rust-lang/rust-clippy)
- formatting via [rustfmt](https://github.com/rust-lang/rustfmt)
- a uniform way of building via [cargo-make](https://github.com/sagiegurari/cargo-make)

## How to use

> 🚨 do not fork this repository as it is a [template repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template)

1. Click on [Use this template](https://github.com/okp4/template-rust/generate)
2. Give a name to your project
3. Wait until the first run of CI finishes
4. Clone your new project and happy coding!

⚠ Do not forget to adapt your project to your needs by editing the `Cargo.toml` file.

## Prerequisites

Be sure you have [Rust](https://www.rust-lang.org/tools/install) properly installed with [cargo-make](https://github.com/sagiegurari/cargo-make).

## Build

```sh
cargo make
```
