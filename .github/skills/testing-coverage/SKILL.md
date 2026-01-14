---
name: testing-coverage
description: Guide for writing tests and achieving high code coverage for CosmWasm contracts. Use when creating unit tests, integration tests, or analyzing coverage reports.
license: BSD-3-Clause
metadata:
  author: axone.xyz
  version: "1.0"
---

# Testing and Code Coverage for CosmWasm Contracts

This skill helps you write comprehensive tests and achieve high code coverage for Axone protocol smart contracts.

## When to use this skill

Use this skill when you need to:

- Write unit tests for contract logic
- Create integration tests with Abstract SDK
- Set up test environments with cw-orch
- Analyze and improve code coverage
- Test error conditions and edge cases

## Test Environment Setup

### Integration Test Structure

```rust
// tests/integration.rs
use my_contract::{
    contract::interface::MyContractInterface,
    msg::{
        MyContractExecuteMsgFns, MyContractInstantiateMsg, 
        MyContractQueryMsgFns, ConfigResponse,
    },
    MyContractError, AXONE_NAMESPACE,
};

use abstract_app::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Application, Environment};
use cosmwasm_std::coins;
use cw_orch::{anyhow, prelude::*};

struct TestEnv<Env: CwEnv> {
    abs: AbstractClient<Env>,
    app: Application<Env, MyContractInterface<Env>>,
}

impl TestEnv<MockBech32> {
    fn setup() -> anyhow::Result<TestEnv<MockBech32>> {
        let mock = MockBech32::new("mock");
        let sender = mock.sender_addr();
        let namespace = Namespace::new(AXONE_NAMESPACE)?;

        // Build Abstract client
        let abs_client = AbstractClient::builder(mock.clone()).build()?;
        
        // Add initial balance
        mock.add_balance(&sender, coins(1_000_000, "uaxone"))?;
        
        // Create publisher account
        let publisher = abs_client
            .account_builder()
            .namespace(namespace)
            .build()?
            .publisher()?;
        
        // Publish and install the app
        publisher.publish_app::<MyContractInterface<_>>()?;
        let app = publisher
            .account()
            .install_app::<MyContractInterface<_>>(
                &MyContractInstantiateMsg::default(), 
                &[]
            )?;

        Ok(TestEnv {
            abs: abs_client,
            app,
        })
    }
}
```

## Test Patterns

### Success Case Testing

```rust
#[test]
fn successful_install() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;

    let config = app.config()?;
    assert_eq!(config, ConfigResponse { /* expected */ });
    Ok(())
}

#[test]
fn successful_execute() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;

    // Execute action
    app.do_something("param")?;
    
    // Verify state changed
    let result = app.query_something()?;
    assert_eq!(result.value, expected_value);
    Ok(())
}
```

### Error Case Testing

```rust
#[test]
fn unauthorized_access() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;

    // Call as non-admin
    let err: MyContractError = app
        .call_as(&Addr::unchecked("not_admin"))
        .admin_only_action()
        .unwrap_err()
        .downcast()
        .unwrap();
    
    assert_eq!(err, MyContractError::Unauthorized {});
    Ok(())
}
```

### Balance and Fund Testing

```rust
#[test]
fn balance_operations() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let account = env.app.account();

    // Add balance to account
    let funds = coins(100, "uaxone");
    account.add_balance(&funds)?;
    
    let balances = account.query_balances()?;
    assert_eq!(balances, funds);

    // Add balance to any address
    let mock_env = env.abs.environment();
    mock_env.add_balance(&env.app.address()?, funds.clone())?;
    
    Ok(())
}
```

### Multiple User Testing

```rust
#[test]
fn multi_user_scenario() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let mock = env.abs.environment();
    
    // Create additional users
    let alice = mock.addr_make("alice");
    let bob = mock.addr_make("bob");
    
    mock.add_balance(&alice, coins(1000, "uaxone"))?;
    mock.add_balance(&bob, coins(1000, "uaxone"))?;
    
    // Test as different users
    env.app.call_as(&alice).user_action()?;
    env.app.call_as(&bob).user_action()?;
    
    Ok(())
}
```

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

## Coverage Analysis

After running `cargo make test-coverage`, coverage is saved to `lcov.info`.

### Viewing Coverage

```bash
# Generate HTML report (requires lcov)
genhtml lcov.info --output-directory coverage/

# Or use cargo-llvm-cov directly
cargo llvm-cov --html --open
```

### Coverage Goals

| Component | Target |
| - | - |
| **Handlers** | 100% |
| **State operations** | 100% |
| **Error paths** | 90%+ |
| **Message parsing** | 80%+ |
| **Overall** | 90%+ |

## Test Checklist

### For Each Handler

- [ ] Happy path test
- [ ] All parameter variations tested
- [ ] Error cases tested (unauthorized, invalid input, etc.)
- [ ] State changes verified
- [ ] Events/attributes checked if applicable

### For Queries

- [ ] Empty state returns appropriate response
- [ ] Populated state returns correct data
- [ ] Pagination works correctly (if applicable)
- [ ] Non-existent items handled gracefully

### For State

- [ ] Initial state after instantiation
- [ ] State updates correctly after execute
- [ ] State persists across operations

## Best Practices

1. **Isolate tests** - Each test should set up its own environment
2. **Test business logic** - Focus on contract behavior, not framework
3. **Use descriptive names** - `test_increment_when_paused_fails`
4. **Assert specific errors** - Check exact error type, not just failure
5. **Clean up** - Don't leave state that could affect other tests
6. **Test edge cases** - Zero values, max values, empty strings
