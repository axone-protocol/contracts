use crate::domain::Uri;
use crate::{
    contract::AxoneVcResult,
    domain::{Credential, CredentialError},
    msg::CredentialInputFormat,
    services::authority,
    state,
    state::{
        credential as stored_credential, credentials as stored_credentials, credentials_by_subject,
        credentials_by_type, has_credential, is_revoked, record_credential, CredentialRecord,
        CredentialTombstone,
    },
    translation::{
        decode_canonical_nquads_credential, decode_nquads_credential_for_issuer,
        CredentialDecodingError, DecodedQuad,
    },
};
use cosmwasm_std::{Binary, StdError, StdResult, Storage, Timestamp};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct IssueCredentialResult {
    pub credential_id: String,
    pub issuer: String,
    pub subject: String,
    pub types: Vec<String>,
    pub valid_from: Option<Timestamp>,
    pub valid_until: Option<Timestamp>,
}

#[derive(Debug, Error, PartialEq)]
pub enum IssueCredentialError {
    #[error("credential already exists")]
    CredentialAlreadyExists,

    #[error("credential revoked")]
    CredentialRevoked,

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
    let (credential, record) = issue_credential_with_authority(storage, authority, input, format)?;

    record_credential(storage, credential.id(), &record)?;

    Ok(IssueCredentialResult {
        credential_id: credential.id().clone(),
        issuer: credential.issuer().clone(),
        subject: credential.subject_id().clone(),
        types: credential.types().clone(),
        valid_from: credential.valid_from(),
        valid_until: credential.valid_until(),
    })
}

fn issue_credential_with_authority(
    storage: &dyn Storage,
    authority: crate::domain::Authority,
    input: &[u8],
    format: CredentialInputFormat,
) -> Result<(Credential, CredentialRecord), IssueCredentialError> {
    let decoded = match format {
        CredentialInputFormat::NQuads => {
            decode_nquads_credential_for_issuer(input, authority.did())?
        }
    };
    let canonical_nquads = decoded.canonical_nquads().clone();
    let valid_from = *decoded.valid_from();
    let valid_until = *decoded.valid_until();
    let credential = Credential::try_from(decoded)?;

    if has_credential(storage, credential.id()) {
        return Err(IssueCredentialError::CredentialAlreadyExists);
    }

    if is_revoked(storage, credential.id()) {
        return Err(IssueCredentialError::CredentialRevoked);
    }

    let record = CredentialRecord::new(
        canonical_nquads,
        credential.subject_id().clone(),
        credential.types().clone(),
        valid_from,
        valid_until,
    );

    Ok((credential, record))
}

#[derive(Debug, PartialEq)]
pub struct VerifyCredentialResult {
    pub exists: bool,
    pub valid: bool,
}

pub fn verify_credential(
    storage: &dyn Storage,
    credential_id: &str,
    valid_at: Option<Timestamp>,
) -> AxoneVcResult<VerifyCredentialResult> {
    let Some(record) = stored_credential(storage, credential_id)? else {
        return Ok(VerifyCredentialResult {
            exists: false,
            valid: false,
        });
    };

    let valid = valid_at.is_none_or(|at| {
        record.valid_from.is_none_or(|from| from <= at)
            && record.valid_until.is_none_or(|until| at < until)
    });

    Ok(VerifyCredentialResult {
        exists: true,
        valid,
    })
}

pub fn credential_raw(storage: &dyn Storage, credential_id: &str) -> AxoneVcResult<Binary> {
    let record = stored_credential(storage, credential_id)?
        .ok_or_else(|| StdError::not_found("credential"))?;

    Ok(Binary::from(record.canonical_nquads.into_bytes()))
}

#[derive(Debug, PartialEq)]
pub struct CredentialResult {
    pub parsed: Credential,
    pub quads: Vec<DecodedQuad>,
}

pub fn credential(storage: &dyn Storage, credential_id: &str) -> AxoneVcResult<CredentialResult> {
    let record = stored_credential(storage, credential_id)?
        .ok_or_else(|| StdError::not_found("credential"))?;
    let canonical = record.canonical_nquads;
    let decoded = decode_canonical_nquads_credential(&canonical)?;
    let quads = decoded.quads().clone();
    let parsed = Credential::try_from(decoded)?;

    Ok(CredentialResult { parsed, quads })
}

pub fn credentials(
    storage: &dyn Storage,
    subject: Option<&str>,
    credential_type: Option<&str>,
    valid_at: Option<Timestamp>,
    limit: usize,
    start_after: Option<String>,
) -> AxoneVcResult<Vec<Uri>> {
    if limit == 0 {
        return Ok(Vec::new());
    }

    let it = match (subject, credential_type) {
        (Some(subject), _) => credentials_by_subject(storage, subject, start_after.as_deref()),
        (None, Some(ctype)) => credentials_by_type(storage, ctype, start_after.as_deref()),
        (None, None) => stored_credentials(storage, start_after.as_deref()),
    };

    let type_filter: Box<dyn FnMut(&StdResult<(String, CredentialRecord)>) -> bool> =
        match credential_type {
            Some(t) => {
                let t = t.to_owned();
                Box::new(move |r| match r {
                    Ok((_, c)) => c.types.contains(&t),
                    Err(_) => true,
                })
            }
            None => Box::new(|_| true),
        };

    let valid_filter: Box<dyn FnMut(&StdResult<(String, CredentialRecord)>) -> bool> =
        match valid_at {
            Some(t) => Box::new(move |r| match r {
                Ok((_, c)) => {
                    c.valid_from.map(|from| from <= t).unwrap_or(true)
                        && c.valid_until.map(|until| t < until).unwrap_or(true)
                }
                Err(_) => true,
            }),
            None => Box::new(|_| true),
        };

    it.filter(type_filter)
        .filter(valid_filter)
        .take(limit)
        .map(|item| item.map(|(id, _record)| id).map_err(Into::into))
        .collect::<AxoneVcResult<Vec<_>>>()
}

#[derive(Debug, PartialEq)]
pub struct RevokeCredentialResult {
    pub identifier: String,
    pub issuer: String,
}

#[derive(Debug, Error, PartialEq)]
pub enum RevokeCredentialError {
    #[error("credential already revoked")]
    CredentialAlreadyRevoked,

    #[error("credential unknown")]
    UnknownCredential,
}

pub fn revoke_credential(
    storage: &mut dyn Storage,
    credential_id: &str,
) -> AxoneVcResult<RevokeCredentialResult> {
    let authority = authority(storage)?;
    if is_revoked(storage, credential_id) {
        return Err(RevokeCredentialError::CredentialAlreadyRevoked.into());
    }
    if !has_credential(storage, credential_id) {
        return Err(RevokeCredentialError::UnknownCredential.into());
    }

    let tombstone = CredentialTombstone::new();
    state::revoke_credential(storage, credential_id, &tombstone)?;

    Ok(RevokeCredentialResult {
        identifier: credential_id.to_string(),
        issuer: authority.did().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::Authority, error::AxoneVcError, msg::CredentialInputFormat,
        services::initialize_authority, state::load_credential,
        translation::CredentialDecodingError,
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

    fn credential_payload_with_validity(
        authority_did: &str,
        id: &str,
        valid_from: &str,
        valid_until: &str,
    ) -> Vec<u8> {
        format!(
            r#"<{id}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{id}> <https://www.w3.org/2018/credentials#issuer> <{authority_did}> .
<{id}> <https://www.w3.org/2018/credentials#issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{id}> <https://www.w3.org/2018/credentials#validFrom> "{valid_from}"^^<http://www.w3.org/2001/XMLSchema#dateTimeStamp> .
<{id}> <https://www.w3.org/2018/credentials#validUntil> "{valid_until}"^^<http://www.w3.org/2001/XMLSchema#dateTimeStamp> .
<{id}> <https://www.w3.org/2018/credentials#credentialSubject> <did:example:subject> .
"#
        )
        .into_bytes()
    }

    fn credential_payload_with_metadata(
        authority_did: &str,
        id: &str,
        subject: &str,
        extra_types: &[&str],
        validity: Option<(&str, &str)>,
    ) -> Vec<u8> {
        let mut payload = format!(
            r#"<{id}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{id}> <https://www.w3.org/2018/credentials#issuer> <{authority_did}> .
<{id}> <https://www.w3.org/2018/credentials#issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{id}> <https://www.w3.org/2018/credentials#credentialSubject> <{subject}> .
"#
        );

        for credential_type in extra_types {
            payload.push_str(&format!(
                "<{id}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <{credential_type}> .\n"
            ));
        }

        if let Some((valid_from, valid_until)) = validity {
            payload.push_str(&format!(
                r#"<{id}> <https://www.w3.org/2018/credentials#validFrom> "{valid_from}"^^<http://www.w3.org/2001/XMLSchema#dateTimeStamp> .
<{id}> <https://www.w3.org/2018/credentials#validUntil> "{valid_until}"^^<http://www.w3.org/2001/XMLSchema#dateTimeStamp> .
"#
            ));
        }

        payload.into_bytes()
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
        assert_eq!(result.issuer, authority.did());
        assert_eq!(result.subject, "did:example:subject");
        assert_eq!(
            result.types,
            vec!["https://www.w3.org/2018/credentials#VerifiableCredential"]
        );
        assert_eq!(result.valid_from, None);
        assert_eq!(result.valid_until, None);

        let record = load_credential(deps.as_ref().storage, credential_id)
            .expect("credential should be persisted");
        assert!(record.canonical_nquads.contains(credential_id));
        assert!(record.canonical_nquads.contains(authority.did()));
        assert_eq!(record.valid_from, None);
        assert_eq!(record.valid_until, None);
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
        let authority = initialized_authority(&mut deps);

        let result = issue_credential(
            deps.as_mut().storage,
            &credential_payload_without_issuer(credential_id),
            CredentialInputFormat::NQuads,
        )
        .expect("missing issuer should be inferred from authority");

        assert_eq!(result.credential_id, credential_id);
        assert_eq!(result.issuer, authority.did());

        let record = load_credential(deps.as_ref().storage, credential_id)
            .expect("credential should be persisted");
        assert!(record.canonical_nquads.contains(&format!(
            "<{credential_id}> <https://www.w3.org/2018/credentials#issuer> <{}> .",
            authority.did()
        )));
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

    #[test]
    fn issue_credential_with_authority_rejects_revoked() {
        let mut deps = mock_dependencies();
        let authority = initialized_authority(&mut deps);
        let payload = credential_payload(authority.did(), "urn:uuid:credential-1");

        issue_credential(
            deps.as_mut().storage,
            &payload,
            CredentialInputFormat::NQuads,
        )
        .expect("first submit should succeed");

        revoke_credential(deps.as_mut().storage, "urn:uuid:credential-1")
            .expect("revocation should succeed");

        let err = issue_credential_with_authority(
            deps.as_ref().storage,
            authority,
            &payload,
            CredentialInputFormat::NQuads,
        )
        .expect_err("second submit should fail");

        assert_eq!(err, IssueCredentialError::CredentialRevoked);
    }

    #[test]
    fn issue_credential_persists_validity_interval_and_verifies_boundaries() {
        let mut deps = mock_dependencies();
        let authority = initialized_authority(&mut deps);
        let credential_id = "urn:uuid:credential-validity";
        let valid_from = Timestamp::from_seconds(10);
        let valid_until = Timestamp::from_seconds(20);

        issue_credential(
            deps.as_mut().storage,
            &credential_payload_with_validity(
                authority.did(),
                credential_id,
                "1970-01-01T00:00:10Z",
                "1970-01-01T00:00:20Z",
            ),
            CredentialInputFormat::NQuads,
        )
        .expect("credential should issue");

        let record = load_credential(deps.as_ref().storage, credential_id)
            .expect("credential should be stored");
        assert_eq!(record.valid_from, Some(valid_from));
        assert_eq!(record.valid_until, Some(valid_until));

        let cases = [
            (None, true),
            (Some(Timestamp::from_seconds(9)), false),
            (Some(valid_from), true),
            (Some(Timestamp::from_seconds(19)), true),
            (Some(valid_until), false),
        ];
        for (at, expected_valid) in cases {
            assert_eq!(
                verify_credential(deps.as_ref().storage, credential_id, at)
                    .expect("verification should succeed"),
                VerifyCredentialResult {
                    exists: true,
                    valid: expected_valid,
                }
            );
        }
    }

    #[test]
    fn credential_returns_parsed_credential_and_decoded_quads() {
        let mut deps = mock_dependencies();
        let authority = initialized_authority(&mut deps);
        let credential_id = "urn:uuid:credential-query";
        let valid_from = Timestamp::from_seconds(10);
        let valid_until = Timestamp::from_seconds(20);

        issue_credential(
            deps.as_mut().storage,
            &credential_payload_with_validity(
                authority.did(),
                credential_id,
                "1970-01-01T00:00:10Z",
                "1970-01-01T00:00:20Z",
            ),
            CredentialInputFormat::NQuads,
        )
        .expect("credential should issue");

        let result =
            credential(deps.as_ref().storage, credential_id).expect("credential should load");

        assert_eq!(result.parsed.id(), credential_id);
        assert_eq!(result.parsed.issuer(), authority.did());
        assert_eq!(result.parsed.subject_id(), "did:example:subject");
        assert_eq!(
            result.parsed.types(),
            &vec!["https://www.w3.org/2018/credentials#VerifiableCredential".to_string()]
        );
        assert_eq!(result.parsed.valid_from(), Some(valid_from));
        assert_eq!(result.parsed.valid_until(), Some(valid_until));
        let quads = result
            .quads
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        assert!(quads.contains(credential_id));
        assert!(quads.contains(authority.did()));
        assert!(quads.contains("validFrom"));
        assert!(quads.contains("validUntil"));
    }

    #[test]
    fn credential_rejects_unknown_and_revoked_credentials() {
        let mut deps = mock_dependencies();
        let authority = initialized_authority(&mut deps);
        let credential_id = "urn:uuid:credential-query";

        let err = credential(deps.as_ref().storage, credential_id)
            .expect_err("unknown credential should fail");
        assert!(err.to_string().contains("credential not found"));

        issue_credential(
            deps.as_mut().storage,
            &credential_payload(authority.did(), credential_id),
            CredentialInputFormat::NQuads,
        )
        .expect("credential should issue");
        revoke_credential(deps.as_mut().storage, credential_id).expect("credential should revoke");

        let err = credential(deps.as_ref().storage, credential_id)
            .expect_err("revoked credential should fail");
        assert!(err.to_string().contains("credential not found"));
    }

    #[test]
    fn credentials_returns_empty_list_without_credentials_and_with_zero_limit() {
        let mut deps = mock_dependencies();
        let authority = initialized_authority(&mut deps);

        assert_eq!(
            credentials(deps.as_ref().storage, None, None, None, 10, None)
                .expect("credentials query should succeed"),
            Vec::<String>::new()
        );

        issue_credential(
            deps.as_mut().storage,
            &credential_payload(authority.did(), "urn:uuid:credential-1"),
            CredentialInputFormat::NQuads,
        )
        .expect("credential should issue");

        assert_eq!(
            credentials(deps.as_ref().storage, None, None, None, 0, None)
                .expect("credentials query should succeed"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn credentials_paginates_identifiers_in_ascending_order() {
        let mut deps = mock_dependencies();
        let authority = initialized_authority(&mut deps);

        for id in [
            "urn:uuid:credential-c",
            "urn:uuid:credential-a",
            "urn:uuid:credential-b",
        ] {
            issue_credential(
                deps.as_mut().storage,
                &credential_payload(authority.did(), id),
                CredentialInputFormat::NQuads,
            )
            .expect("credential should issue");
        }

        assert_eq!(
            credentials(deps.as_ref().storage, None, None, None, 2, None)
                .expect("first page should load"),
            vec!["urn:uuid:credential-a", "urn:uuid:credential-b"]
        );
        assert_eq!(
            credentials(
                deps.as_ref().storage,
                None,
                None,
                None,
                2,
                Some("urn:uuid:credential-b".to_string())
            )
            .expect("second page should load"),
            vec!["urn:uuid:credential-c"]
        );
    }

    #[test]
    fn credentials_filters_by_subject_type_and_validity() {
        let mut deps = mock_dependencies();
        let authority = initialized_authority(&mut deps);

        for payload in [
            credential_payload_with_metadata(
                authority.did(),
                "urn:uuid:credential-a",
                "did:example:alice",
                &["https://example.com/types/Employee"],
                Some(("1970-01-01T00:00:10Z", "1970-01-01T00:00:20Z")),
            ),
            credential_payload_with_metadata(
                authority.did(),
                "urn:uuid:credential-b",
                "did:example:bob",
                &["https://example.com/types/Member"],
                None,
            ),
            credential_payload_with_metadata(
                authority.did(),
                "urn:uuid:credential-c",
                "did:example:alice",
                &["https://example.com/types/Member"],
                Some(("1970-01-01T00:00:20Z", "1970-01-01T00:00:30Z")),
            ),
            credential_payload_with_metadata(
                authority.did(),
                "urn:uuid:credential-d",
                "did:example:alice",
                &["https://example.com/types/Employee"],
                None,
            ),
        ] {
            issue_credential(
                deps.as_mut().storage,
                &payload,
                CredentialInputFormat::NQuads,
            )
            .expect("credential should issue");
        }

        assert_eq!(
            credentials(
                deps.as_ref().storage,
                Some("did:example:alice"),
                None,
                None,
                10,
                None
            )
            .expect("subject filter should load"),
            vec![
                "urn:uuid:credential-a",
                "urn:uuid:credential-c",
                "urn:uuid:credential-d"
            ]
        );
        assert_eq!(
            credentials(
                deps.as_ref().storage,
                None,
                Some("https://example.com/types/Member"),
                None,
                10,
                None
            )
            .expect("type filter should load"),
            vec!["urn:uuid:credential-b", "urn:uuid:credential-c"]
        );
        assert_eq!(
            credentials(
                deps.as_ref().storage,
                Some("did:example:alice"),
                Some("https://example.com/types/Member"),
                Some(Timestamp::from_seconds(25)),
                10,
                None
            )
            .expect("combined filters should load"),
            vec!["urn:uuid:credential-c"]
        );
        assert_eq!(
            credentials(
                deps.as_ref().storage,
                None,
                None,
                Some(Timestamp::from_seconds(20)),
                10,
                None
            )
            .expect("validity filter should load"),
            vec![
                "urn:uuid:credential-b",
                "urn:uuid:credential-c",
                "urn:uuid:credential-d"
            ]
        );
    }

    #[test]
    fn credentials_excludes_revoked_credentials_from_indexes() {
        let mut deps = mock_dependencies();
        let authority = initialized_authority(&mut deps);

        issue_credential(
            deps.as_mut().storage,
            &credential_payload_with_metadata(
                authority.did(),
                "urn:uuid:credential-a",
                "did:example:alice",
                &["https://example.com/types/Member"],
                None,
            ),
            CredentialInputFormat::NQuads,
        )
        .expect("credential should issue");
        revoke_credential(deps.as_mut().storage, "urn:uuid:credential-a")
            .expect("credential should revoke");

        assert_eq!(
            credentials(
                deps.as_ref().storage,
                Some("did:example:alice"),
                Some("https://example.com/types/Member"),
                None,
                10,
                None
            )
            .expect("credentials query should succeed"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn issue_credential_rejects_invalid_validity_claims() {
        let mut deps = mock_dependencies();
        let authority = initialized_authority(&mut deps);

        let wrong_type = String::from_utf8(credential_payload_with_validity(
            authority.did(),
            "urn:uuid:wrong-validity-type",
            "1970-01-01T00:00:10Z",
            "1970-01-01T00:00:20Z",
        ))
        .expect("test payload should be UTF-8")
        .replace("#dateTimeStamp", "#dateTime");
        let err = issue_credential(
            deps.as_mut().storage,
            wrong_type.as_bytes(),
            CredentialInputFormat::NQuads,
        )
        .expect_err("non-dateTimeStamp validity claims should fail");
        assert_eq!(
            err,
            AxoneVcError::IssueCredential(IssueCredentialError::Decode(
                CredentialDecodingError::InvalidDataset
            ))
        );

        let err = issue_credential(
            deps.as_mut().storage,
            &credential_payload_with_validity(
                authority.did(),
                "urn:uuid:inverted-validity",
                "1970-01-01T00:00:20Z",
                "1970-01-01T00:00:10Z",
            ),
            CredentialInputFormat::NQuads,
        )
        .expect_err("inverted validity interval should fail");
        assert_eq!(
            err,
            AxoneVcError::IssueCredential(IssueCredentialError::Domain(
                CredentialError::InvalidValidityInterval
            ))
        );
    }

    #[test]
    fn verify_credential_treats_unknown_and_revoked_credentials_as_absent() {
        let mut deps = mock_dependencies();
        let authority = initialized_authority(&mut deps);
        let credential_id = "urn:uuid:credential-1";

        assert_eq!(
            verify_credential(deps.as_ref().storage, credential_id, None)
                .expect("verification should succeed"),
            VerifyCredentialResult {
                exists: false,
                valid: false,
            }
        );

        issue_credential(
            deps.as_mut().storage,
            &credential_payload(authority.did(), credential_id),
            CredentialInputFormat::NQuads,
        )
        .expect("credential should issue");
        revoke_credential(deps.as_mut().storage, credential_id).expect("credential should revoke");

        assert_eq!(
            verify_credential(deps.as_ref().storage, credential_id, None)
                .expect("verification should succeed"),
            VerifyCredentialResult {
                exists: false,
                valid: false,
            }
        );
    }

    #[test]
    fn revoke_credential_success() {
        let mut deps = mock_dependencies();
        let authority = initialized_authority(&mut deps);
        let payload = credential_payload(authority.did(), "urn:uuid:credential-1");

        issue_credential(
            deps.as_mut().storage,
            &payload,
            CredentialInputFormat::NQuads,
        )
        .expect("submit should succeed");

        let res = revoke_credential(deps.as_mut().storage, "urn:uuid:credential-1");
        assert!(res.is_ok(), "revoke should succeed");
        assert_eq!(
            res.unwrap(),
            RevokeCredentialResult {
                identifier: "urn:uuid:credential-1".to_string(),
                issuer: authority.did().to_string(),
            }
        );
    }

    #[test]
    fn revoke_credential_rejects_unknown_credential() {
        let mut deps = mock_dependencies();
        initialized_authority(&mut deps);

        let err = revoke_credential(deps.as_mut().storage, "urn:uuid:credential-1")
            .expect_err("revocation should fail");

        assert_eq!(
            err,
            AxoneVcError::RevokeCredential(RevokeCredentialError::UnknownCredential)
        );
    }

    #[test]
    fn revoke_credential_rejects_revoked_credential() {
        let mut deps = mock_dependencies();
        let authority = initialized_authority(&mut deps);
        let payload = credential_payload(authority.did(), "urn:uuid:credential-1");

        issue_credential(
            deps.as_mut().storage,
            &payload,
            CredentialInputFormat::NQuads,
        )
        .expect("submit should succeed");

        revoke_credential(deps.as_mut().storage, "urn:uuid:credential-1")
            .expect("first revoke should succeed");

        let err = revoke_credential(deps.as_mut().storage, "urn:uuid:credential-1")
            .expect_err("second revocation should fail");

        assert_eq!(
            err,
            AxoneVcError::RevokeCredential(RevokeCredentialError::CredentialAlreadyRevoked)
        );
    }
}
