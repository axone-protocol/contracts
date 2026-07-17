use cosmwasm_std::Timestamp;
use getset::Getters;
use oxrdf::{
    vocab::{rdf, xsd},
    Dataset, GraphName, Literal, NamedNode, NamedNodeRef, Quad, Subject, Term,
};
use oxttl::NQuadsParser;
use rdf_canon::canonicalize;
use std::{collections::HashSet, str};
use thiserror::Error;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

const VC_ISSUER: NamedNodeRef<'static> =
    NamedNodeRef::new_unchecked("https://www.w3.org/2018/credentials#issuer");
const VC_VALID_FROM: NamedNodeRef<'static> =
    NamedNodeRef::new_unchecked("https://www.w3.org/2018/credentials#validFrom");
const VC_VALID_UNTIL: NamedNodeRef<'static> =
    NamedNodeRef::new_unchecked("https://www.w3.org/2018/credentials#validUntil");
const VC_CREDENTIAL_SUBJECT: NamedNodeRef<'static> =
    NamedNodeRef::new_unchecked("https://www.w3.org/2018/credentials#credentialSubject");

#[derive(Debug, Error, PartialEq)]
pub enum CredentialDecodingError {
    #[error("credential input format is unsupported")]
    UnsupportedFormat,

    #[error("credential input is not valid utf-8")]
    InvalidUtf8,

    #[error("credential input is not valid n-quads")]
    InvalidNQuads,

    #[error("credential RDF dataset is invalid")]
    InvalidDataset,

    #[error("credential canonicalization failed")]
    CanonicalizationFailed,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DecodedUri {
    Missing,
    Uri(String),
    Invalid,
}

#[derive(Clone, Debug, Getters, PartialEq)]
pub struct DecodedCredential {
    #[getset(get = "pub(crate)")]
    id: Option<String>,
    #[getset(get = "pub(crate)")]
    issuer: DecodedUri,
    #[getset(get = "pub(crate)")]
    valid_from: Option<Timestamp>,
    #[getset(get = "pub(crate)")]
    valid_until: Option<Timestamp>,
    #[getset(get = "pub(crate)")]
    subject_id: DecodedUri,
    #[getset(get = "pub(crate)")]
    types: Vec<String>,
    #[getset(get = "pub(crate)")]
    canonical_nquads: String,
}

impl DecodedCredential {
    pub(crate) fn new(
        id: Option<String>,
        issuer: DecodedUri,
        subject_id: DecodedUri,
        types: Vec<String>,
        canonical_nquads: String,
    ) -> Self {
        Self {
            id,
            issuer,
            valid_from: None,
            valid_until: None,
            subject_id,
            types,
            canonical_nquads,
        }
    }

    pub(crate) fn with_validity(
        mut self,
        valid_from: Option<Timestamp>,
        valid_until: Option<Timestamp>,
    ) -> Self {
        self.valid_from = valid_from;
        self.valid_until = valid_until;
        self
    }
}

#[cfg(test)]
fn decode_nquads_credential(input: &[u8]) -> Result<DecodedCredential, CredentialDecodingError> {
    let utf8 = str::from_utf8(input).map_err(|_| CredentialDecodingError::InvalidUtf8)?;
    let quads = parse_nquads_quads(utf8.as_bytes())?;
    let dataset = Dataset::from_iter(quads.iter().cloned());
    let canonical_nquads = canonicalize_dataset(&dataset)?;

    decode_dataset_credential(&quads, &dataset, canonical_nquads)
}

pub fn decode_nquads_credential_for_issuer(
    input: &[u8],
    issuer_did: &str,
) -> Result<DecodedCredential, CredentialDecodingError> {
    let utf8 = str::from_utf8(input).map_err(|_| CredentialDecodingError::InvalidUtf8)?;
    let quads = parse_nquads_quads(utf8.as_bytes())?;
    let mut dataset = Dataset::from_iter(quads.iter().cloned());
    let credential_subject = find_credential_subject(&dataset)?;
    let issuer = extract_issuer(&dataset, &credential_subject)?;

    match issuer {
        DecodedUri::Missing => {
            insert_issuer(&mut dataset, credential_subject, issuer_did)?;
        }
        DecodedUri::Uri(issuer) if issuer == issuer_did => {}
        DecodedUri::Uri(_) | DecodedUri::Invalid => {
            return Err(CredentialDecodingError::InvalidDataset);
        }
    }

    let canonical_nquads = canonicalize_dataset(&dataset)?;
    let quads = parse_nquads_quads(canonical_nquads.as_bytes())?;

    decode_dataset_credential(&quads, &dataset, canonical_nquads)
}

pub(crate) fn decode_canonical_nquads_credential(
    input: &str,
) -> Result<DecodedCredential, CredentialDecodingError> {
    let quads = parse_nquads_quads(input.as_bytes())?;
    let dataset = Dataset::from_iter(quads.iter().cloned());

    decode_dataset_credential(&quads, &dataset, input.to_string())
}

fn decode_dataset_credential(
    quads: &[Quad],
    dataset: &Dataset,
    canonical_nquads: String,
) -> Result<DecodedCredential, CredentialDecodingError> {
    let credential_subject = find_credential_subject(dataset)?;
    let id = subject_to_identifier(&credential_subject);
    let issuer = extract_issuer(dataset, &credential_subject)?;
    let valid_from = extract_validity_bound(quads, &credential_subject, VC_VALID_FROM)?;
    let valid_until = extract_validity_bound(quads, &credential_subject, VC_VALID_UNTIL)?;
    let subject_id = extract_subject_id(dataset, &credential_subject)?;
    let types = extract_types(dataset, &credential_subject);

    Ok(
        DecodedCredential::new(id, issuer, subject_id, types, canonical_nquads)
            .with_validity(valid_from, valid_until),
    )
}

#[cfg(test)]
fn parse_nquads(input: &[u8]) -> Result<Dataset, CredentialDecodingError> {
    Ok(Dataset::from_iter(parse_nquads_quads(input)?))
}

fn parse_nquads_quads(input: &[u8]) -> Result<Vec<Quad>, CredentialDecodingError> {
    NQuadsParser::new()
        .for_slice(input)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| CredentialDecodingError::InvalidNQuads)
}

pub fn find_credential_subject(dataset: &Dataset) -> Result<Subject, CredentialDecodingError> {
    let candidate_subjects: HashSet<Subject> = [
        VC_ISSUER,
        VC_VALID_FROM,
        VC_VALID_UNTIL,
        VC_CREDENTIAL_SUBJECT,
    ]
    .into_iter()
    .flat_map(|predicate| dataset.quads_for_predicate(predicate))
    .map(|quad| quad.subject.into_owned())
    .collect();

    let mut subjects = candidate_subjects.into_iter();
    let subject = subjects
        .next()
        .ok_or(CredentialDecodingError::InvalidDataset)?;

    if subjects.next().is_some() {
        return Err(CredentialDecodingError::InvalidDataset);
    }

    Ok(subject)
}

pub fn subject_to_identifier(subject: &Subject) -> Option<String> {
    match subject {
        Subject::NamedNode(node) => Some(node.as_str().to_string()),
        Subject::BlankNode(_) => None,
    }
}

fn extract_issuer(
    dataset: &Dataset,
    credential_subject: &Subject,
) -> Result<DecodedUri, CredentialDecodingError> {
    let objects = collect_objects(dataset, credential_subject, VC_ISSUER);

    match objects.as_slice() {
        [] => Ok(DecodedUri::Missing),
        [Term::NamedNode(node)] => Ok(DecodedUri::Uri(node.as_str().to_string())),
        [_] => Ok(DecodedUri::Invalid),
        _ => Err(CredentialDecodingError::InvalidDataset),
    }
}

fn extract_validity_bound(
    quads: &[Quad],
    credential_subject: &Subject,
    predicate: NamedNodeRef<'_>,
) -> Result<Option<Timestamp>, CredentialDecodingError> {
    let objects: Vec<&Term> = quads
        .iter()
        .filter(|quad| {
            quad.subject.as_ref() == credential_subject.as_ref() && quad.predicate == predicate
        })
        .map(|quad| &quad.object)
        .collect();

    match objects.as_slice() {
        [] => Ok(None),
        [Term::Literal(literal)] => parse_validity_bound(literal).map(Some),
        _ => Err(CredentialDecodingError::InvalidDataset),
    }
}

fn parse_validity_bound(literal: &Literal) -> Result<Timestamp, CredentialDecodingError> {
    if literal.datatype() != xsd::DATE_TIME_STAMP {
        return Err(CredentialDecodingError::InvalidDataset);
    }

    let datetime = OffsetDateTime::parse(literal.value(), &Rfc3339)
        .map_err(|_| CredentialDecodingError::InvalidDataset)?;
    let nanos = u64::try_from(datetime.unix_timestamp_nanos())
        .map_err(|_| CredentialDecodingError::InvalidDataset)?;

    Ok(Timestamp::from_nanos(nanos))
}

fn extract_subject_id(
    dataset: &Dataset,
    credential_subject: &Subject,
) -> Result<DecodedUri, CredentialDecodingError> {
    let objects = collect_objects(dataset, credential_subject, VC_CREDENTIAL_SUBJECT);

    match objects.as_slice() {
        [] => Ok(DecodedUri::Missing),
        [Term::NamedNode(node)] => Ok(DecodedUri::Uri(node.as_str().to_string())),
        [_] => Ok(DecodedUri::Invalid),
        _ => Err(CredentialDecodingError::InvalidDataset),
    }
}

fn extract_types(dataset: &Dataset, credential_subject: &Subject) -> Vec<String> {
    let mut types: Vec<String> = collect_objects(dataset, credential_subject, rdf::TYPE)
        .into_iter()
        .filter_map(|term| match term {
            Term::NamedNode(node) => Some(node.as_str().to_string()),
            _ => None,
        })
        .collect();

    types.sort();
    types.dedup();

    types
}

fn collect_objects(dataset: &Dataset, subject: &Subject, predicate: NamedNodeRef<'_>) -> Vec<Term> {
    dataset
        .quads_for_subject(subject.as_ref())
        .filter(|quad| quad.predicate == predicate)
        .map(|quad| quad.object.into_owned())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn insert_issuer(
    dataset: &mut Dataset,
    credential_subject: Subject,
    issuer: &str,
) -> Result<(), CredentialDecodingError> {
    let issuer = NamedNode::new(issuer).map_err(|_| CredentialDecodingError::InvalidDataset)?;
    let quad = Quad::new(
        credential_subject,
        VC_ISSUER.into_owned(),
        issuer,
        GraphName::DefaultGraph,
    );
    dataset.insert(quad.as_ref());
    Ok(())
}

fn canonicalize_dataset(dataset: &Dataset) -> Result<String, CredentialDecodingError> {
    canonicalize(dataset).map_err(map_canonicalization_error)
}

fn map_canonicalization_error(_: rdf_canon::CanonicalizationError) -> CredentialDecodingError {
    CredentialDecodingError::CanonicalizationFailed
}

#[cfg(test)]
mod tests {
    use super::{
        decode_nquads_credential, decode_nquads_credential_for_issuer, extract_issuer,
        extract_subject_id, extract_validity_bound, find_credential_subject,
        map_canonicalization_error, parse_nquads, parse_nquads_quads, parse_validity_bound,
        subject_to_identifier, CredentialDecodingError, DecodedUri, VC_ISSUER, VC_VALID_FROM,
    };
    use cosmwasm_std::Timestamp;
    use oxrdf::{BlankNode, Literal, NamedNodeRef, Subject};

    const AUTHORITY_DID: &str = "did:pkh:cosmos:axone-localnet-1:cosmos1authority";
    const CREDENTIAL_ID: &str = "urn:uuid:credential-1";
    const VC_NAMESPACE: &str = "https://www.w3.org/2018/credentials#";

    fn valid_credential(authority_did: &str) -> Vec<u8> {
        format!(
            r#"<{CREDENTIAL_ID}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{CREDENTIAL_ID}> <{}> <{authority_did}> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}credentialSubject> <did:example:subject> .
"#,
            VC_ISSUER.as_str()
        )
        .into_bytes()
    }

    fn parsed_dataset(input: &[u8]) -> oxrdf::Dataset {
        parse_nquads(input).expect("dataset should parse")
    }
    #[test]
    fn decode_credential_extracts_canonicalized_fields() {
        let decoded = decode_nquads_credential(&valid_credential(AUTHORITY_DID))
            .expect("credential should decode");

        assert_eq!(decoded.id().as_deref(), Some(CREDENTIAL_ID));
        assert_eq!(
            decoded.issuer(),
            &DecodedUri::Uri(AUTHORITY_DID.to_string())
        );
        assert_eq!(
            decoded.subject_id(),
            &DecodedUri::Uri("did:example:subject".to_string())
        );
        assert!(decoded
            .types()
            .iter()
            .any(|value| value == "https://www.w3.org/2018/credentials#VerifiableCredential"));
        assert!(decoded.canonical_nquads().contains(CREDENTIAL_ID));
    }

    #[test]
    fn decode_credential_rejects_invalid_utf8() {
        let err = decode_nquads_credential(&[0xff, 0xfe]).expect_err("invalid UTF-8 should fail");

        assert_eq!(err, CredentialDecodingError::InvalidUtf8);
    }

    #[test]
    fn decode_credential_rejects_invalid_nquads() {
        let err =
            decode_nquads_credential(b"not n-quads").expect_err("invalid n-quads should fail");

        assert_eq!(err, CredentialDecodingError::InvalidNQuads);
    }

    #[test]
    fn decode_credential_allows_missing_types_for_domain_validation() {
        let decoded = decode_nquads_credential(
            format!(
                r#"<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuer> <{AUTHORITY_DID}> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}credentialSubject> <did:example:subject> .
"#
            )
            .as_bytes(),
        )
        .expect("missing type should still decode");

        assert!(decoded.types().is_empty());
    }

    #[test]
    fn decode_credential_allows_non_verifiable_types_for_domain_validation() {
        let decoded = decode_nquads_credential(
            format!(
                r#"<{CREDENTIAL_ID}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://example.com/types/Test> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuer> <{AUTHORITY_DID}> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}credentialSubject> <did:example:subject> .
"#
            )
            .as_bytes(),
        )
        .expect("non-verifiable type should still decode");

        assert_eq!(
            decoded.types(),
            &vec!["https://example.com/types/Test".to_string()]
        );
    }

    #[test]
    fn decode_credential_keeps_missing_issuer_for_domain_validation() {
        let decoded = decode_nquads_credential(
            format!(
                r#"<{CREDENTIAL_ID}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}credentialSubject> <did:example:subject> .
"#
            )
            .as_bytes(),
        )
        .expect("missing issuer should still decode");

        assert_eq!(decoded.issuer(), &DecodedUri::Missing);
    }

    #[test]
    fn decode_credential_for_authority_injects_missing_issuer() {
        let decoded = decode_nquads_credential_for_issuer(
            format!(
                r#"<{CREDENTIAL_ID}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}credentialSubject> <did:example:subject> .
"#
            )
            .as_bytes(),
            AUTHORITY_DID,
        )
        .expect("missing issuer should be injected");

        assert_eq!(
            decoded.issuer(),
            &DecodedUri::Uri(AUTHORITY_DID.to_string())
        );
        assert!(decoded.canonical_nquads().contains(&format!(
            "<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuer> <{AUTHORITY_DID}> ."
        )));
    }

    #[test]
    fn decode_credential_for_authority_rejects_mismatched_issuer() {
        let err = decode_nquads_credential_for_issuer(
            format!(
                r#"<{CREDENTIAL_ID}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuer> <did:example:issuer> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}credentialSubject> <did:example:subject> .
"#
            )
            .as_bytes(),
            AUTHORITY_DID,
        )
        .expect_err("mismatched issuer should fail");

        assert_eq!(err, CredentialDecodingError::InvalidDataset);
    }

    #[test]
    fn decode_credential_keeps_missing_subject_for_domain_validation() {
        let decoded = decode_nquads_credential(
            format!(
                r#"<{CREDENTIAL_ID}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuer> <{AUTHORITY_DID}> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
"#
            )
            .as_bytes(),
        )
        .expect("missing subject should still decode");

        assert_eq!(decoded.subject_id(), &DecodedUri::Missing);
    }

    #[test]
    fn decode_credential_ignores_issuance_date() {
        let decoded = decode_nquads_credential(
            format!(
                r#"<{CREDENTIAL_ID}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuer> <{AUTHORITY_DID}> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuanceDate> "not-a-date"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}credentialSubject> <did:example:subject> .
"#
            )
            .as_bytes(),
        )
        .expect("issuance date should be ignored");

        assert_eq!(decoded.id().as_deref(), Some(CREDENTIAL_ID));
    }

    #[test]
    fn decode_credential_rejects_missing_credential_subject() {
        let err = decode_nquads_credential(b"").expect_err("empty dataset should fail");

        assert_eq!(err, CredentialDecodingError::InvalidDataset);
    }

    #[test]
    fn decode_credential_rejects_multiple_credential_subjects() {
        let err = decode_nquads_credential(
            format!(
                r#"<urn:uuid:credential-1> <{VC_NAMESPACE}issuer> <{AUTHORITY_DID}> .
<urn:uuid:credential-2> <{VC_NAMESPACE}issuer> <{AUTHORITY_DID}> .
"#
            )
            .as_bytes(),
        )
        .expect_err("multiple credential subjects should fail");

        assert_eq!(err, CredentialDecodingError::InvalidDataset);
    }

    #[test]
    fn find_credential_subject_ignores_typed_nodes_unrelated_to_credential_identity() {
        let dataset = parsed_dataset(
            format!(
                r#"<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuer> <{AUTHORITY_DID}> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}credentialSubject> <did:example:subject> .
<did:example:subject> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://example.com/types/Agent> .
"#
            )
            .as_bytes(),
        );

        let subject =
            find_credential_subject(&dataset).expect("typed related nodes should not shadow VC");

        assert_eq!(
            subject,
            Subject::NamedNode(NamedNodeRef::new_unchecked(CREDENTIAL_ID).into_owned())
        );
    }

    #[test]
    fn decode_credential_marks_invalid_issuer_shape_for_domain_validation() {
        let decoded = decode_nquads_credential(
            format!(
                r#"<{CREDENTIAL_ID}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuer> _:issuer .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}credentialSubject> <did:example:subject> .
"#
            )
            .as_bytes(),
        )
        .expect("blank node issuer should still decode");

        assert_eq!(decoded.issuer(), &DecodedUri::Invalid);
    }

    #[test]
    fn decode_credential_marks_invalid_subject_shape_for_domain_validation() {
        let decoded = decode_nquads_credential(
            format!(
                r#"<{CREDENTIAL_ID}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuer> <{AUTHORITY_DID}> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}credentialSubject> _:subject .
"#
            )
            .as_bytes(),
        )
        .expect("blank node subject should still decode");

        assert_eq!(decoded.subject_id(), &DecodedUri::Invalid);
    }

    #[test]
    fn subject_to_identifier_returns_none_for_blank_node_subject() {
        let subject =
            Subject::BlankNode(BlankNode::new("credential").expect("blank node should build"));

        assert_eq!(subject_to_identifier(&subject), None);
    }

    #[test]
    fn extract_issuer_rejects_multiple_values() {
        let dataset = parsed_dataset(
            format!(
                r#"<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuer> <{AUTHORITY_DID}> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuer> <did:example:issuer-2> .
"#
            )
            .as_bytes(),
        );

        let err = extract_issuer(
            &dataset,
            &Subject::NamedNode(NamedNodeRef::new_unchecked(CREDENTIAL_ID).into_owned()),
        )
        .expect_err("multiple issuer values should fail");

        assert_eq!(err, CredentialDecodingError::InvalidDataset);
    }

    #[test]
    fn extract_subject_id_rejects_multiple_values() {
        let dataset = parsed_dataset(
            format!(
                r#"<{CREDENTIAL_ID}> <{VC_NAMESPACE}credentialSubject> <did:example:subject-1> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}credentialSubject> <did:example:subject-2> .
"#
            )
            .as_bytes(),
        );

        let err = extract_subject_id(
            &dataset,
            &Subject::NamedNode(NamedNodeRef::new_unchecked(CREDENTIAL_ID).into_owned()),
        )
        .expect_err("multiple credential subjects should fail");

        assert_eq!(err, CredentialDecodingError::InvalidDataset);
    }

    #[test]
    fn extract_validity_bound_accepts_date_time_stamp() {
        let quads = parse_nquads_quads(
            format!(
                r#"<{CREDENTIAL_ID}> <{VC_NAMESPACE}validFrom> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTimeStamp> .
"#
            )
            .as_bytes(),
        )
        .expect("quads should parse");

        let bound = extract_validity_bound(
            &quads,
            &Subject::NamedNode(NamedNodeRef::new_unchecked(CREDENTIAL_ID).into_owned()),
            VC_VALID_FROM,
        )
        .expect("validity bound should decode");

        assert_eq!(bound, Some(Timestamp::from_seconds(1_735_689_600)));
    }

    #[test]
    fn extract_validity_bound_rejects_duplicate_claims() {
        let quads = parse_nquads_quads(
            format!(
                r#"<{CREDENTIAL_ID}> <{VC_NAMESPACE}validFrom> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTimeStamp> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}validFrom> "2025-01-02T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTimeStamp> .
"#
            )
            .as_bytes(),
        )
        .expect("quads should parse");

        let err = extract_validity_bound(
            &quads,
            &Subject::NamedNode(NamedNodeRef::new_unchecked(CREDENTIAL_ID).into_owned()),
            VC_VALID_FROM,
        )
        .expect_err("duplicate validity bounds should fail");

        assert_eq!(err, CredentialDecodingError::InvalidDataset);
    }

    #[test]
    fn decode_credential_rejects_repeated_identical_validity_claims() {
        let payload = format!(
            r#"<{CREDENTIAL_ID}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuer> <{AUTHORITY_DID}> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}issuanceDate> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}validFrom> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTimeStamp> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}validFrom> "2025-01-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTimeStamp> .
<{CREDENTIAL_ID}> <{VC_NAMESPACE}credentialSubject> <did:example:subject> .
"#
        );

        let err = decode_nquads_credential(payload.as_bytes())
            .expect_err("repeated validity claims should fail");

        assert_eq!(err, CredentialDecodingError::InvalidDataset);
    }

    #[test]
    fn parse_validity_bound_rejects_non_date_time_stamp_and_invalid_instants() {
        let wrong_type = Literal::new_typed_literal(
            "2025-01-01T00:00:00Z",
            NamedNodeRef::new_unchecked("http://www.w3.org/2001/XMLSchema#dateTime"),
        );
        let invalid_instant = Literal::new_typed_literal(
            "not-an-instant",
            NamedNodeRef::new_unchecked("http://www.w3.org/2001/XMLSchema#dateTimeStamp"),
        );

        assert_eq!(
            parse_validity_bound(&wrong_type),
            Err(CredentialDecodingError::InvalidDataset)
        );
        assert_eq!(
            parse_validity_bound(&invalid_instant),
            Err(CredentialDecodingError::InvalidDataset)
        );
    }

    #[test]
    fn canonicalization_failure_is_mapped() {
        let err =
            map_canonicalization_error(rdf_canon::CanonicalizationError::HndqCallLimitExceeded(0));

        assert_eq!(err, CredentialDecodingError::CanonicalizationFailed);
    }
}
