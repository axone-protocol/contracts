---
name: testing-coverage
description: Guide for writing tests and achieving high code coverage for CosmWasm contracts. Use when creating unit tests, integration tests, or analyzing coverage reports.
license: BSD-3-Clause
metadata:
  author: axone.xyz
  version: "1.0"
---

# Testing and Code Coverage for CosmWasm Contracts

## Coverage Targets

| Component            | Target |
| -------------------- | ------ |
| **Handlers**         | 100%   |
| **State operations** | 100%   |
| **Error paths**      | 90%+   |
| **Message parsing**  | 80%+   |
| **Overall**          | 90%+   |

## Minimum Cases to Cover

- For handlers: cover the happy path, parameter variations, error paths, and resulting state changes.
- For queries: cover empty state, populated state, pagination behavior, and non-existent items.
- For state transitions: cover initial state, updates after execute, and persistence across operations.

## Running Tests

```bash
# Run all unit tests
cargo make test-unit

# Run all tests (includes integration)
cargo make test

# Run tests with coverage
cargo make test-coverage

# Run specific test
cargo test test_name -- --nocapture
```

Coverage output is written to `lcov.info`.

## Viewing Coverage

```bash
# Generate HTML report (requires lcov)
genhtml lcov.info --output-directory coverage/

# Or use cargo-llvm-cov directly
cargo llvm-cov --html --open
```

## Test Design Rules

1. Isolate tests. Each test should set up its own environment.
2. Test contract behavior, not framework internals.
3. Assert exact errors when possible.
4. Cover edge cases such as zero values, empty strings, and upper bounds.
5. Prefer descriptive test names without a `test_` prefix.
6. Use tuple-driven cases for simple matrices and structs only when the case shape needs named fields.

## References

- For cw-orch integration harness patterns and common integration scenarios, read [integration-patterns](./references/integration-patterns.md).
- For naming and data-driven test examples, read [test-style](./references/test-style.md).
