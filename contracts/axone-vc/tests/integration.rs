use abstract_app::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Application};
use axone_vc::{
    msg::{AxoneVcExecuteMsgFns, AxoneVcInstantiateMsg, AxoneVcQueryMsgFns, CredentialInputFormat},
    AxoneVcInterface, AXONE_NAMESPACE,
};
use bech32::{Bech32, Hrp};
use cosmwasm_std::{Binary, Timestamp};
use cw_orch::contract::interface_traits::CallAs;
use cw_orch::{anyhow, prelude::*};

const COLLAB_AI_ZONE_PROFILE: &str = include_str!("fixtures/collab-ai-zone-profile.nq");
const RESOURCE_LICENSE_ASSERTION: &str = include_str!("fixtures/resource-license-assertion.nq");
const VC_ISSUER_PREDICATE: &str = "<https://www.w3.org/2018/credentials#issuer>";
const SOURCE_ISSUER_DID: &str =
    "<did:pkh:cosmos:axone-1:cosmos1s7auhjsmvjpiubqwco6bxxehsqwnvepvabhbrv>";
const RESOURCE_LICENSE_ASSERTION_ID: &str =
    "https://credentials.axone.xyz/assertion/resource-license";
const ABSTRACT_EVENT_TYPE: &str = "wasm-abstract";

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
fn issue_credential_emits_event() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let authority = AxoneVcQueryMsgFns::authority(&env.app)?;
    let credential = resource_license_assertion_payload(&authority.did);

    let response = env.app.issue_credential(
        Binary::from(credential),
        Some(CredentialInputFormat::NQuads),
    )?;

    assert_eq!(
        response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "action")
            .expect("Missing action attribute"),
        "issue_credential"
    );
    assert_eq!(
        response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "identifier")
            .expect("Missing identifier attribute"),
        RESOURCE_LICENSE_ASSERTION_ID
    );
    assert_eq!(
        response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "issuer")
            .expect("Missing issuer attribute"),
        authority.did
    );
    assert_eq!(
        response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "subject")
            .expect("Missing subject attribute"),
        "urn:uuid:5d29ea71-003f-46e7-a74d-d8d598629ed8"
    );
    assert_eq!(
        response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "types")
            .expect("Missing types attribute"),
        "https://w3id.org/axone/ontology/v4/schema/core/assertion/AssertionCredential,https://www.w3.org/2018/credentials#VerifiableCredential"
    );
    assert!(response
        .event_attr_value(ABSTRACT_EVENT_TYPE, "issued_at")
        .is_err());
    assert!(response
        .event_attr_value(ABSTRACT_EVENT_TYPE, "valid_from")
        .is_err());
    assert!(response
        .event_attr_value(ABSTRACT_EVENT_TYPE, "valid_until")
        .is_err());

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

    assert!(
        format!("{err:?}").contains("credential already exists"),
        "{err:?}"
    );

    Ok(())
}

#[test]
fn issue_credential_rejects_non_host_account_sender() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let authority = AxoneVcQueryMsgFns::authority(&env.app)?;
    let credential = Binary::from(resource_license_assertion_payload(&authority.did));
    let unauthorized = env.app.environment().addr_make("unauthorized");

    let err = env
        .app
        .call_as(&unauthorized)
        .issue_credential(credential, Some(CredentialInputFormat::NQuads))
        .expect_err("non-host sender should be rejected");

    assert!(format!("{err:?}").contains("Caller is not admin"));

    Ok(())
}

#[test]
fn verify_credential_reports_activity_and_validity_interval() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let authority = AxoneVcQueryMsgFns::authority(&env.app)?;
    let credential_id = "urn:uuid:credential-validity";

    assert_eq!(
        AxoneVcQueryMsgFns::verify_credential(&env.app, credential_id.to_string(), None)?,
        axone_vc::msg::VerifyCredentialResponse {
            exists: false,
            valid: false,
        }
    );

    let issue_response = env.app.issue_credential(
        Binary::from(credential_payload_with_validity(
            &authority.did,
            credential_id,
            "1970-01-01T00:00:10Z",
            "1970-01-01T00:00:20Z",
        )),
        Some(CredentialInputFormat::NQuads),
    )?;
    assert_eq!(
        issue_response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "valid_from")
            .expect("Missing valid_from attribute"),
        "10.000000000"
    );
    assert_eq!(
        issue_response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "valid_until")
            .expect("Missing valid_until attribute"),
        "20.000000000"
    );

    for (at, expected_valid) in [
        (None, true),
        (Some(Timestamp::from_seconds(10)), true),
        (Some(Timestamp::from_seconds(20)), false),
    ] {
        let response =
            AxoneVcQueryMsgFns::verify_credential(&env.app, credential_id.to_string(), at)?;
        assert!(response.exists);
        assert_eq!(response.valid, expected_valid);
    }

    env.app.revoke_credential(credential_id.to_string())?;
    assert_eq!(
        AxoneVcQueryMsgFns::verify_credential(&env.app, credential_id.to_string(), None)?,
        axone_vc::msg::VerifyCredentialResponse {
            exists: false,
            valid: false,
        }
    );

    Ok(())
}

#[test]
fn credential_raw_returns_the_expected_canonical_representation() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let authority = AxoneVcQueryMsgFns::authority(&env.app)?;
    let credential_id = "urn:uuid:credential-raw";
    let input = format!(
        r#"<{credential_id}> <https://www.w3.org/2018/credentials#issuer> <{}> .
<{credential_id}> <https://www.w3.org/2018/credentials#credentialSubject> <did:example:subject> .
<{credential_id}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{credential_id}> <https://www.w3.org/2018/credentials#issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
"#,
        authority.did
    );
    let expected = format!(
        r#"<{credential_id}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{credential_id}> <https://www.w3.org/2018/credentials#credentialSubject> <did:example:subject> .
<{credential_id}> <https://www.w3.org/2018/credentials#issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{credential_id}> <https://www.w3.org/2018/credentials#issuer> <{}> .
"#,
        authority.did
    );

    assert_ne!(input, expected);
    env.app.issue_credential(
        Binary::from(input.into_bytes()),
        Some(CredentialInputFormat::NQuads),
    )?;

    let response = AxoneVcQueryMsgFns::credential_raw(&env.app, credential_id.to_string())?;
    assert_eq!(response.credential, Binary::from(expected.into_bytes()));

    Ok(())
}

#[test]
fn credential_raw_rejects_unknown_and_revoked_credentials() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let credential_id = "urn:uuid:credential-raw";

    let err = AxoneVcQueryMsgFns::credential_raw(&env.app, credential_id.to_string())
        .expect_err("unknown credential should be rejected");
    assert!(
        format!("{err:?}").contains("credential not found"),
        "{err:?}"
    );

    let authority = AxoneVcQueryMsgFns::authority(&env.app)?;
    env.app.issue_credential(
        Binary::from(credential_payload_with_validity(
            &authority.did,
            credential_id,
            "1970-01-01T00:00:10Z",
            "1970-01-01T00:00:20Z",
        )),
        Some(CredentialInputFormat::NQuads),
    )?;
    env.app.revoke_credential(credential_id.to_string())?;

    let err = AxoneVcQueryMsgFns::credential_raw(&env.app, credential_id.to_string())
        .expect_err("revoked credential should be rejected");
    assert!(
        format!("{err:?}").contains("credential not found"),
        "{err:?}"
    );

    Ok(())
}

#[test]
fn revoke_credential_accepts_issued_credential() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let authority = AxoneVcQueryMsgFns::authority(&env.app)?;
    let credential = Binary::from(resource_license_assertion_payload(&authority.did));

    env.app
        .issue_credential(credential, Some(CredentialInputFormat::NQuads))?;
    env.app
        .revoke_credential(RESOURCE_LICENSE_ASSERTION_ID.to_string())?;

    Ok(())
}

#[test]
fn revoke_credential_emits_event() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let authority = AxoneVcQueryMsgFns::authority(&env.app)?;
    let credential = Binary::from(resource_license_assertion_payload(&authority.did));

    env.app
        .issue_credential(credential, Some(CredentialInputFormat::NQuads))?;
    let response = env
        .app
        .revoke_credential(RESOURCE_LICENSE_ASSERTION_ID.to_string())?;

    assert_eq!(
        response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "action")
            .expect("Missing action attribute"),
        "revoke_credential"
    );
    assert_eq!(
        response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "identifier")
            .expect("Missing identifier attribute"),
        RESOURCE_LICENSE_ASSERTION_ID
    );
    assert_eq!(
        response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "issuer")
            .expect("Missing issuer attribute"),
        authority.did
    );
    assert!(response
        .event_attr_value(ABSTRACT_EVENT_TYPE, "revoked_at")
        .is_err());

    Ok(())
}

#[test]
fn revoke_credential_rejects_unknown_credential() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;

    let err = env
        .app
        .revoke_credential(RESOURCE_LICENSE_ASSERTION_ID.to_string())
        .expect_err("unknown credential should be rejected");

    assert!(format!("{err:?}").contains("credential unknown"), "{err:?}");

    Ok(())
}

#[test]
fn revoke_credential_rejects_duplicates() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let authority = AxoneVcQueryMsgFns::authority(&env.app)?;
    let credential = Binary::from(resource_license_assertion_payload(&authority.did));

    env.app
        .issue_credential(credential, Some(CredentialInputFormat::NQuads))?;
    env.app
        .revoke_credential(RESOURCE_LICENSE_ASSERTION_ID.to_string())?;
    let err = env
        .app
        .revoke_credential(RESOURCE_LICENSE_ASSERTION_ID.to_string())
        .expect_err("duplicate revocation should fail");

    assert!(
        format!("{err:?}").contains("credential already revoked"),
        "{err:?}"
    );

    Ok(())
}

#[test]
fn revoke_credential_prevents_reissuing_same_identifier() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let authority = AxoneVcQueryMsgFns::authority(&env.app)?;
    let credential = Binary::from(resource_license_assertion_payload(&authority.did));

    env.app
        .issue_credential(credential.clone(), Some(CredentialInputFormat::NQuads))?;
    env.app
        .revoke_credential(RESOURCE_LICENSE_ASSERTION_ID.to_string())?;
    let err = env
        .app
        .issue_credential(credential, Some(CredentialInputFormat::NQuads))
        .expect_err("revoked credential identifier should not be reusable");

    assert!(format!("{err:?}").contains("credential revoked"), "{err:?}");

    Ok(())
}

#[test]
fn revoke_credential_rejects_non_host_account_sender() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let authority = AxoneVcQueryMsgFns::authority(&env.app)?;
    let credential = Binary::from(resource_license_assertion_payload(&authority.did));
    let unauthorized = env.app.environment().addr_make("unauthorized");

    env.app
        .issue_credential(credential, Some(CredentialInputFormat::NQuads))?;
    let err = env
        .app
        .call_as(&unauthorized)
        .revoke_credential(RESOURCE_LICENSE_ASSERTION_ID.to_string())
        .expect_err("non-host sender should be rejected");

    assert!(format!("{err:?}").contains("Caller is not admin"));

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

fn credential_payload_with_validity(
    authority_did: &str,
    credential_id: &str,
    valid_from: &str,
    valid_until: &str,
) -> Vec<u8> {
    format!(
        r#"<{credential_id}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{credential_id}> <https://www.w3.org/2018/credentials#issuer> <{authority_did}> .
<{credential_id}> <https://www.w3.org/2018/credentials#validFrom> "{valid_from}"^^<http://www.w3.org/2001/XMLSchema#dateTimeStamp> .
<{credential_id}> <https://www.w3.org/2018/credentials#validUntil> "{valid_until}"^^<http://www.w3.org/2001/XMLSchema#dateTimeStamp> .
<{credential_id}> <https://www.w3.org/2018/credentials#credentialSubject> <did:example:subject> .
"#
    )
    .into_bytes()
}
