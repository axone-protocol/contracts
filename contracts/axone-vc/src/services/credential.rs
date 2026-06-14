use crate::{
    contract::AxoneVcResult,
    domain::{Credential, CredentialError},
    msg::CredentialInputFormat,
    services::authority,
    state::{has_credential, record_credential, CredentialRecord},
    translation::{decode_nquads_credential, CredentialDecodingError},
};

use cosmwasm_std::Storage;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct IssueCredentialResult {
    pub credential_id: String,
}

#[derive(Debug, Error, PartialEq)]
pub enum IssueCredentialError {
    #[error("credential already exists")]
    CredentialAlreadyExists,

    #[error(transparent)]
    Decode(#[from] CredentialDecodingError),

    #[error(transparent)]
    Domain(#[from] CredentialError),
}

pub fn issue_credential(
    storage: &mut dyn Storage,
    input: &[u8],
    format: CredentialInputFormat,
) -> AxoneVcResult<IssueCredentialResult> {
    let authority = authority(storage)?;
    let (credential_id, record) =
        issue_credential_with_authority(storage, authority, input, format)?;

    record_credential(storage, credential_id.as_str(), &record)?;

    Ok(IssueCredentialResult { credential_id })
}

fn issue_credential_with_authority(
    storage: &dyn Storage,
    authority: crate::domain::Authority,
    input: &[u8],
    format: CredentialInputFormat,
) -> Result<(String, CredentialRecord), IssueCredentialError> {
    let decoded = match format {
        CredentialInputFormat::NQuads => decode_nquads_credential(input)?,
    };
    let canonical_nquads = decoded.canonical_nquads().clone();
    let credential = Credential::try_from((decoded, authority))?;

    if has_credential(storage, credential.id()) {
        return Err(IssueCredentialError::CredentialAlreadyExists);
    }

    Ok((
        credential.id().clone(),
        CredentialRecord::new(canonical_nquads),
    ))
}

#[cfg(test)]
mod tests {
    use super::{issue_credential, issue_credential_with_authority, IssueCredentialError};
    use crate::{
        domain::Authority, error::AxoneVcError, msg::CredentialInputFormat,
        services::initialize_authority, state::load_credential,
    };
    use bech32::{Bech32, Hrp};
    use cosmwasm_std::{testing::mock_dependencies, Addr};

    fn credential_payload(authority_did: &str, id: &str) -> Vec<u8> {
        format!(
            r#"<{id}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{id}> <https://www.w3.org/2018/credentials#issuer> <{authority_did}> .
<{id}> <https://www.w3.org/2018/credentials#issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{id}> <https://www.w3.org/2018/credentials#credentialSubject> <did:example:subject> .
"#
        )
        .into_bytes()
    }

    fn credential_payload_without_issuer(id: &str) -> Vec<u8> {
        format!(
            r#"<{id}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{id}> <https://www.w3.org/2018/credentials#issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{id}> <https://www.w3.org/2018/credentials#credentialSubject> <did:example:subject> .
"#
        )
        .into_bytes()
    }

    fn initialized_authority(
        deps: &mut cosmwasm_std::OwnedDeps<
            cosmwasm_std::testing::MockStorage,
            cosmwasm_std::testing::MockApi,
            cosmwasm_std::testing::MockQuerier,
        >,
    ) -> Authority {
        let payload = [0x31; 20];
        let account_addr = bech32::encode::<Bech32>(Hrp::parse("axone").unwrap(), &payload)
            .expect("valid address");

        initialize_authority(
            deps.as_mut().storage,
            "axone-localnet-1",
            &Addr::unchecked(account_addr),
        )
        .expect("authority should initialize")
    }

    #[test]
    fn issue_credential_persists_canonical_record() {
        let mut deps = mock_dependencies();
        let credential_id = "urn:uuid:credential-1";
        let authority = initialized_authority(&mut deps);

        let result = issue_credential(
            deps.as_mut().storage,
            &credential_payload(authority.did(), credential_id),
            CredentialInputFormat::NQuads,
        )
        .expect("submit should succeed");

        assert_eq!(result.credential_id, credential_id);

        let record = load_credential(deps.as_ref().storage, credential_id)
            .expect("credential should be persisted");
        assert!(record.canonical_nquads.contains(credential_id));
        assert!(record.canonical_nquads.contains(authority.did()));
    }

    #[test]
    fn issue_credential_rejects_duplicates() {
        let mut deps = mock_dependencies();
        let authority = initialized_authority(&mut deps);
        let payload = credential_payload(authority.did(), "urn:uuid:credential-1");

        issue_credential(
            deps.as_mut().storage,
            &payload,
            CredentialInputFormat::NQuads,
        )
        .expect("first submit should succeed");

        let err = issue_credential(
            deps.as_mut().storage,
            &payload,
            CredentialInputFormat::NQuads,
        )
        .expect_err("second submit should fail");

        assert_eq!(
            err,
            AxoneVcError::IssueCredential(IssueCredentialError::CredentialAlreadyExists)
        );
    }

    #[test]
    fn issue_credential_accepts_missing_issuer() {
        let mut deps = mock_dependencies();
        let credential_id = "urn:uuid:credential-2";
        initialized_authority(&mut deps);

        let result = issue_credential(
            deps.as_mut().storage,
            &credential_payload_without_issuer(credential_id),
            CredentialInputFormat::NQuads,
        )
        .expect("missing issuer should be inferred from authority");

        assert_eq!(result.credential_id, credential_id);
    }

    #[test]
    fn issue_credential_with_authority_rejects_duplicates() {
        let mut deps = mock_dependencies();
        let authority = initialized_authority(&mut deps);
        let payload = credential_payload(authority.did(), "urn:uuid:credential-1");

        issue_credential(
            deps.as_mut().storage,
            &payload,
            CredentialInputFormat::NQuads,
        )
        .expect("first submit should succeed");

        let err = issue_credential_with_authority(
            deps.as_ref().storage,
            authority,
            &payload,
            CredentialInputFormat::NQuads,
        )
        .expect_err("second submit should fail");

        assert_eq!(err, IssueCredentialError::CredentialAlreadyExists);
    }
}
