# Integration Test Patterns

Use these examples when the task needs a concrete `cw-orch` / Abstract integration harness or an example of a common integration scenario.

## Integration Test Structure

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

        let abs_client = AbstractClient::builder(mock.clone()).build()?;
        mock.add_balance(&sender, coins(1_000_000, "uaxone"))?;

        let publisher = abs_client
            .account_builder()
            .namespace(namespace)
            .build()?
            .publisher()?;

        publisher.publish_app::<MyContractInterface<_>>()?;
        let app = publisher
            .account()
            .install_app::<MyContractInterface<_>>(&MyContractInstantiateMsg::default(), &[])?;

        Ok(TestEnv {
            abs: abs_client,
            app,
        })
    }
}
```

## Success Case Testing

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

    app.do_something("param")?;

    let result = app.query_something()?;
    assert_eq!(result.value, expected_value);
    Ok(())
}
```

## Error Case Testing

```rust
#[test]
fn unauthorized_access() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;

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

## Balance and Fund Testing

```rust
#[test]
fn balance_operations() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let account = env.app.account();

    let funds = coins(100, "uaxone");
    account.add_balance(&funds)?;

    let balances = account.query_balances()?;
    assert_eq!(balances, funds);

    let mock_env = env.abs.environment();
    mock_env.add_balance(&env.app.address()?, funds.clone())?;

    Ok(())
}
```

## Multiple User Testing

```rust
#[test]
fn multi_user_scenario() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let mock = env.abs.environment();

    let alice = mock.addr_make("alice");
    let bob = mock.addr_make("bob");

    mock.add_balance(&alice, coins(1000, "uaxone"))?;
    mock.add_balance(&bob, coins(1000, "uaxone"))?;

    env.app.call_as(&alice).user_action()?;
    env.app.call_as(&bob).user_action()?;

    Ok(())
}
```
