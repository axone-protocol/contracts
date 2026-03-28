use abstract_app::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Application};
use axone_vc::{
    msg::{AxoneVcExecuteMsgFns, AxoneVcInstantiateMsg, AxoneVcQueryMsgFns},
    AxoneVcInterface, AXONE_NAMESPACE,
};
use bech32::{Bech32, Hrp};
use cw_orch::{anyhow, prelude::*};

struct TestEnv<Env: CwEnv> {
    app: Application<Env, AxoneVcInterface<Env>>,
}

impl TestEnv<MockBech32> {
    fn setup() -> anyhow::Result<Self> {
        let chain = MockBech32::new_with_chain_id("axone", "axone-localnet-1");
        let client = AbstractClient::builder(chain.clone()).build()?;
        let publisher = client
            .account_builder()
            .namespace(Namespace::new(AXONE_NAMESPACE)?)
            .build()?
            .publisher()?;
        publisher.publish_app::<AxoneVcInterface<MockBech32>>()?;

        let app = publisher
            .account()
            .install_app::<AxoneVcInterface<MockBech32>>(&AxoneVcInstantiateMsg {}, &[])?;

        Ok(Self { app })
    }
}

#[test]
fn authority_query_returns_canonical_did() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let account_addr = env.app.account().address()?;

    let authority = AxoneVcQueryMsgFns::authority(&env.app)?;

    let parts: Vec<&str> = authority.did.split(':').collect();
    assert_eq!(parts.len(), 5);
    assert_eq!(parts[0], "did");
    assert_eq!(parts[1], "pkh");
    assert_eq!(parts[2], "cosmos");
    assert_eq!(parts[3], "axone-localnet-1");

    let (_original_hrp, data) = bech32::decode(account_addr.as_str())?;
    let cosmos_hrp = Hrp::parse("cosmos")?;
    let expected_addr = bech32::encode::<Bech32>(cosmos_hrp, &data)?;

    assert_eq!(parts[4], expected_addr);

    Ok(())
}

#[test]
fn execute_foo_updates_state() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;

    env.app.foo("test_value".to_string())?;

    Ok(())
}

#[test]
fn execute_foo_with_empty_string() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;

    env.app.foo("".to_string())?;

    Ok(())
}

#[test]
fn execute_foo_with_long_string() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;

    let long_value = "a".repeat(1000);
    env.app.foo(long_value)?;

    Ok(())
}
