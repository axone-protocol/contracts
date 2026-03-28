use abstract_app::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Application};
use axone_vc::{
    domain::Authority,
    msg::{AxoneVcInstantiateMsg, AxoneVcQueryMsgFns},
    AxoneVcInterface, AXONE_NAMESPACE,
};
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
    let binding = Authority::new("axone-localnet-1", &account_addr)?;
    let expected = binding.did();

    assert_eq!(authority.did, expected);
    assert!(authority
        .did
        .starts_with("did:pkh:cosmos:axone-localnet-1:"));
    assert!(authority.did.contains(":cosmos1"));

    Ok(())
}
