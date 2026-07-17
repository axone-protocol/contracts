mod credential_rdf;

pub(crate) use credential_rdf::{
    decode_canonical_nquads_credential, decode_nquads_credential_for_issuer,
    CredentialDecodingError, DecodedCredential, DecodedQuad, DecodedUri,
};
