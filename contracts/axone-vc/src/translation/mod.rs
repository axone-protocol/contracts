mod credential_rdf;

pub use credential_rdf::decode_nquads_credential;
pub(crate) use credential_rdf::{
    decode_canonical_nquads_credential, CredentialDecodingError, DecodedCredential, DecodedUri,
};
