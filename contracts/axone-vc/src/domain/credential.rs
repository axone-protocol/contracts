use crate::translation::{DecodedCredential, DecodedUri};
use cosmwasm_std::Timestamp;
use getset::{CopyGetters, Getters};
use thiserror::Error;

pub type Uri = String;

const VC_VERIFIABLE_CREDENTIAL: &str = "https://www.w3.org/2018/credentials#VerifiableCredential";

#[derive(Debug, Error, PartialEq)]
pub enum CredentialError {
    #[error("credential identifier missing")]
    MissingIdentifier,

    #[error("credential issuer is invalid")]
    InvalidIssuer,

    #[error("credential subject missing")]
    MissingSubject,

    #[error("credential type missing")]
    MissingType,

    #[error("credential is not a verifiable credential")]
    NotVerifiableCredential,

    #[error("credential validity interval is invalid")]
    InvalidValidityInterval,
}

#[derive(Clone, CopyGetters, Debug, Getters, PartialEq)]
pub struct Credential {
    #[getset(get = "pub")]
    id: Uri,
    #[getset(get = "pub")]
    issuer: Uri,
    #[getset(get_copy = "pub")]
    valid_from: Option<Timestamp>,
    #[getset(get_copy = "pub")]
    valid_until: Option<Timestamp>,
    #[getset(get = "pub")]
    subject_id: Uri,
    #[getset(get = "pub")]
    types: Vec<String>,
}

impl TryFrom<DecodedCredential> for Credential {
    type Error = CredentialError;

    fn try_from(decoded: DecodedCredential) -> Result<Self, Self::Error> {
        let id = decoded
            .id()
            .clone()
            .ok_or(CredentialError::MissingIdentifier)?;
        let issuer = match decoded.issuer() {
            DecodedUri::Uri(uri) => uri.clone(),
            DecodedUri::Missing | DecodedUri::Invalid => {
                return Err(CredentialError::InvalidIssuer);
            }
        };

        Self::try_new(
            id,
            issuer,
            *decoded.valid_from(),
            *decoded.valid_until(),
            decoded.subject_id(),
            decoded.types().clone(),
        )
    }
}

impl Credential {
    fn try_new(
        id: Uri,
        issuer: Uri,
        valid_from: Option<Timestamp>,
        valid_until: Option<Timestamp>,
        subject_id: &DecodedUri,
        types: Vec<String>,
    ) -> Result<Self, CredentialError> {
        let subject_id = match subject_id {
            DecodedUri::Uri(uri) => uri.clone(),
            DecodedUri::Missing | DecodedUri::Invalid => {
                return Err(CredentialError::MissingSubject);
            }
        };

        if types.is_empty() {
            return Err(CredentialError::MissingType);
        }

        if !types.iter().any(|value| value == VC_VERIFIABLE_CREDENTIAL) {
            return Err(CredentialError::NotVerifiableCredential);
        }

        if matches!((valid_from, valid_until), (Some(from), Some(until)) if from >= until) {
            return Err(CredentialError::InvalidValidityInterval);
        }

        Ok(Self {
            id,
            issuer,
            valid_from,
            valid_until,
            subject_id,
            types,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Credential;
    use crate::{
        domain::CredentialError,
        translation::{DecodedCredential, DecodedUri},
    };
    use cosmwasm_std::Timestamp;

    const AUTHORITY_DID: &str = "did:pkh:cosmos:axone-localnet-1:cosmos1authority";
    const VC_TYPE: &str = "https://www.w3.org/2018/credentials#VerifiableCredential";

    fn decoded_credential() -> DecodedCredential {
        DecodedCredential::new(
            Some("urn:uuid:credential-1".to_string()),
            DecodedUri::Uri(AUTHORITY_DID.to_string()),
            DecodedUri::Uri("did:example:subject".to_string()),
            vec![
                VC_TYPE.to_string(),
                "https://example.com/types/Test".to_string(),
            ],
            String::new(),
        )
    }

    #[test]
    fn try_from_accepts_valid_credential() {
        let credential =
            Credential::try_from(decoded_credential()).expect("credential should be valid");

        assert_eq!(credential.id(), "urn:uuid:credential-1");
        assert_eq!(credential.issuer(), AUTHORITY_DID);
        assert_eq!(credential.subject_id(), "did:example:subject");
        assert_eq!(credential.types().len(), 2);
    }

    #[test]
    fn try_from_decoded_credential_requires_explicit_issuer() {
        let decoded = DecodedCredential::new(
            Some("urn:uuid:credential-1".to_string()),
            DecodedUri::Missing,
            DecodedUri::Uri("did:example:subject".to_string()),
            vec![VC_TYPE.to_string()],
            String::new(),
        );

        let err = Credential::try_from(decoded).expect_err("missing issuer should fail");

        assert_eq!(err, CredentialError::InvalidIssuer);
    }

    #[test]
    fn try_from_requires_identifier() {
        let decoded = DecodedCredential::new(
            None,
            DecodedUri::Uri(AUTHORITY_DID.to_string()),
            DecodedUri::Uri("did:example:subject".to_string()),
            vec![
                VC_TYPE.to_string(),
                "https://example.com/types/Test".to_string(),
            ],
            String::new(),
        );
        let err = Credential::try_from(decoded).expect_err("missing identifier should fail");

        assert_eq!(err, CredentialError::MissingIdentifier);
    }

    #[test]
    fn try_from_accepts_any_valid_issuer() {
        let decoded = DecodedCredential::new(
            Some("urn:uuid:credential-1".to_string()),
            DecodedUri::Uri("did:example:issuer".to_string()),
            DecodedUri::Uri("did:example:subject".to_string()),
            vec![
                VC_TYPE.to_string(),
                "https://example.com/types/Test".to_string(),
            ],
            String::new(),
        );
        let credential = Credential::try_from(decoded).expect("credential should be valid");

        assert_eq!(credential.issuer(), "did:example:issuer");
    }

    #[test]
    fn try_from_rejects_invalid_issuer_shape() {
        let decoded = DecodedCredential::new(
            Some("urn:uuid:credential-1".to_string()),
            DecodedUri::Invalid,
            DecodedUri::Uri("did:example:subject".to_string()),
            vec![
                VC_TYPE.to_string(),
                "https://example.com/types/Test".to_string(),
            ],
            String::new(),
        );
        let err = Credential::try_from(decoded).expect_err("invalid issuer should fail");

        assert_eq!(err, CredentialError::InvalidIssuer);
    }

    #[test]
    fn try_from_requires_subject() {
        let decoded = DecodedCredential::new(
            Some("urn:uuid:credential-1".to_string()),
            DecodedUri::Uri(AUTHORITY_DID.to_string()),
            DecodedUri::Missing,
            vec![
                VC_TYPE.to_string(),
                "https://example.com/types/Test".to_string(),
            ],
            String::new(),
        );
        let err = Credential::try_from(decoded).expect_err("missing subject should fail");

        assert_eq!(err, CredentialError::MissingSubject);
    }

    #[test]
    fn try_from_requires_types() {
        let decoded = DecodedCredential::new(
            Some("urn:uuid:credential-1".to_string()),
            DecodedUri::Uri(AUTHORITY_DID.to_string()),
            DecodedUri::Uri("did:example:subject".to_string()),
            vec![],
            String::new(),
        );
        let err = Credential::try_from(decoded).expect_err("missing types should fail");

        assert_eq!(err, CredentialError::MissingType);
    }

    #[test]
    fn try_from_requires_verifiable_credential_type() {
        let decoded = DecodedCredential::new(
            Some("urn:uuid:credential-1".to_string()),
            DecodedUri::Uri(AUTHORITY_DID.to_string()),
            DecodedUri::Uri("did:example:subject".to_string()),
            vec!["https://example.com/types/Test".to_string()],
            String::new(),
        );
        let err = Credential::try_from(decoded).expect_err("missing vc type should fail");

        assert_eq!(err, CredentialError::NotVerifiableCredential);
    }

    #[test]
    fn try_from_rejects_inverted_validity_interval() {
        let decoded = DecodedCredential::new(
            Some("urn:uuid:credential-1".to_string()),
            DecodedUri::Uri(AUTHORITY_DID.to_string()),
            DecodedUri::Uri("did:example:subject".to_string()),
            vec![VC_TYPE.to_string()],
            String::new(),
        )
        .with_validity(
            Some(Timestamp::from_seconds(20)),
            Some(Timestamp::from_seconds(10)),
        );

        let err =
            Credential::try_from(decoded).expect_err("inverted validity interval should fail");

        assert_eq!(err, CredentialError::InvalidValidityInterval);
    }
}
