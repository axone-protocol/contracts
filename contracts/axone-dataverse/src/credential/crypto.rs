use crate::credential::error::VerificationError;
use crate::credential::proof::ProofMaterial;
use axone_rdf::normalize::Normalizer;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use cosmwasm_std::DepsMut;
use rio_api::model::Quad;
use sha2::Digest;

pub enum CanonicalizationAlg {
    Urdna2015,
}

pub enum DigestAlg {
    Sha256,
}

pub enum SignatureAlg {
    Ed25519,
    Secp256k1,
}

pub struct CryptoSuite {
    canon: CanonicalizationAlg,
    hash: DigestAlg,
    sign: SignatureAlg,
}

impl From<(CanonicalizationAlg, DigestAlg, SignatureAlg)> for CryptoSuite {
    fn from(value: (CanonicalizationAlg, DigestAlg, SignatureAlg)) -> Self {
        Self {
            canon: value.0,
            hash: value.1,
            sign: value.2,
        }
    }
}

impl CryptoSuite {
    pub fn verify_document(
        &self,
        deps: &'_ DepsMut<'_>,
        unsecured_doc: &[Quad<'_>],
        proof_opts: &[Quad<'_>],
        proof_material: ProofMaterial<'_>,
        pub_key: &[u8],
    ) -> Result<(), VerificationError> {
        let unsecured_doc_canon = self.canonicalize(unsecured_doc)?;
        let proof_opts_canon = self.canonicalize(proof_opts)?;

        let hash = [self.hash(proof_opts_canon), self.hash(unsecured_doc_canon)].concat();

        match proof_material {
            ProofMaterial::Signature(v) => self.verify(deps, &hash, v, pub_key),
            ProofMaterial::Jws(jws) => {
                let (headers_b64, signature_b64) = Self::explode_jws(jws)?;
                let signature = BASE64_URL_SAFE_NO_PAD
                    .decode(signature_b64)
                    .map_err(|_| VerificationError::InvalidJws)?;

                let signing_input = [headers_b64, b".", &hash].concat();
                let signing_input = match self.sign {
                    SignatureAlg::Ed25519 => signing_input,
                    SignatureAlg::Secp256k1 => {
                        let mut hasher = sha2::Sha256::new();
                        hasher.update(signing_input);

                        hasher.finalize().to_vec()
                    }
                };

                self.verify(deps, &signing_input, &signature, pub_key)
            }
        }
    }

    fn canonicalize(&self, unsecured_document: &[Quad<'_>]) -> Result<String, VerificationError> {
        match self.canon {
            CanonicalizationAlg::Urdna2015 => {
                let mut normalizer = Normalizer::new();
                normalizer
                    .normalize(unsecured_document)
                    .map_err(VerificationError::from)
            }
        }
    }

    fn hash(&self, transformed_document: String) -> Vec<u8> {
        match self.hash {
            DigestAlg::Sha256 => {
                let mut hasher = sha2::Sha256::new();
                hasher.update(transformed_document);

                hasher.finalize().to_vec()
            }
        }
    }

    fn verify(
        &self,
        deps: &'_ DepsMut<'_>,
        message: &[u8],
        signature: &[u8],
        pub_key: &[u8],
    ) -> Result<(), VerificationError> {
        match match self.sign {
            SignatureAlg::Ed25519 => deps.api.ed25519_verify(message, signature, pub_key),
            SignatureAlg::Secp256k1 => deps.api.secp256k1_verify(message, signature, pub_key),
        } {
            Ok(true) => Ok(()),
            Ok(false) => Err(VerificationError::WrongSignature),
            Err(e) => Err(VerificationError::from(e)),
        }
    }

    fn explode_jws(jws: &str) -> Result<(&[u8], &[u8]), VerificationError> {
        let mut parts = jws.split('.');
        Ok(
            match (parts.next(), parts.next(), parts.next(), parts.next()) {
                (Some(headers), Some(_), Some(sig), None) => (headers.as_bytes(), sig.as_bytes()),
                _ => Err(VerificationError::InvalidJws)?,
            },
        )
    }
}
