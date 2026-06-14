use abstract_app::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Application};
use axone_vc::{
    msg::{AxoneVcExecuteMsgFns, AxoneVcInstantiateMsg, AxoneVcQueryMsgFns, CredentialInputFormat},
    AxoneVcInterface, AXONE_NAMESPACE,
};
use bech32::{Bech32, Hrp};
use cosmwasm_std::Binary;
use cw_orch::{anyhow, prelude::*};

const COLLAB_AI_ZONE_PROFILE: &str = include_str!("fixtures/collab-ai-zone-profile.nq");
const RESOURCE_LICENSE_ASSERTION: &str = include_str!("fixtures/resource-license-assertion.nq");
const VC_ISSUER_PREDICATE: &str = "<https://www.w3.org/2018/credentials#issuer>";
const SOURCE_ISSUER_DID: &str =
    "<did:pkh:cosmos:axone-1:cosmos1s7auhjsmvjpiubqwco6bxxehsqwnvepvabhbrv>";

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
fn issue_credential_accepts_resource_license_assertion_example() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let authority = AxoneVcQueryMsgFns::authority(&env.app)?;
    let credential = resource_license_assertion_payload(&authority.did);

    env.app.issue_credential(Binary::from(credential), None)?;

    Ok(())
}

#[test]
fn issue_credential_accepts_collab_ai_zone_profile_example_without_issuer() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let credential = Binary::from(collab_ai_zone_profile_payload_without_issuer());

    env.app
        .issue_credential(credential, Some(CredentialInputFormat::NQuads))?;

    Ok(())
}

#[test]
fn issue_credential_rejects_duplicates() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let authority = AxoneVcQueryMsgFns::authority(&env.app)?;
    let credential = Binary::from(resource_license_assertion_payload(&authority.did));

    env.app
        .issue_credential(credential.clone(), Some(CredentialInputFormat::NQuads))?;
    let err = env
        .app
        .issue_credential(credential, Some(CredentialInputFormat::NQuads))
        .expect_err("duplicate submit should fail");

    assert!(!err.to_string().is_empty());

    Ok(())
}

fn resource_license_assertion_payload(authority_did: &str) -> Vec<u8> {
    RESOURCE_LICENSE_ASSERTION
        .replace(SOURCE_ISSUER_DID, &format!("<{}>", authority_did))
        .into_bytes()
}

fn collab_ai_zone_profile_payload_without_issuer() -> Vec<u8> {
    COLLAB_AI_ZONE_PROFILE
        .lines()
        .filter(|line| !line.contains(VC_ISSUER_PREDICATE))
        .collect::<Vec<_>>()
        .join("\n")
        .into_bytes()
}
