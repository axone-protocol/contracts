use abstract_app::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Application};
use axone_vc::{
    msg::{AxoneVcExecuteMsgFns, AxoneVcInstantiateMsg, AxoneVcQueryMsgFns},
    AxoneVcInterface, AXONE_NAMESPACE,
};
use cw_orch::{anyhow, prelude::*};

struct TestEnv<Env: CwEnv> {
    app: Application<Env, AxoneVcInterface<Env>>,
}

impl TestEnv<MockBech32> {
    fn setup() -> anyhow::Result<Self> {
        let chain = MockBech32::new("mock");
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
fn foo_roundtrip() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;

    let initial = AxoneVcQueryMsgFns::foo(&env.app)?;
    assert_eq!(initial.value, "foo");

    AxoneVcExecuteMsgFns::foo(&env.app, "bar".to_string())?;

    let updated = AxoneVcQueryMsgFns::foo(&env.app)?;
    assert_eq!(updated.value, "bar");

    Ok(())
}
