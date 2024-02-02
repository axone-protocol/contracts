use crate::credential::error::VerificationError;
use okp4_rdf::normalize::Normalizer;
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
        unsecured_doc: &[Quad<'_>],
        proof_opts: &[Quad<'_>],
        proof_value: &[u8],
        pub_key: &[u8],
    ) -> Result<bool, VerificationError> {
        let unsecured_doc_canon = self.canonicalize(unsecured_doc)?;
        let proof_opts_canon = self.canonicalize(proof_opts)?;

        let hash = [self.hash(proof_opts_canon), self.hash(unsecured_doc_canon)].concat();

        self.verify(&hash, proof_value, pub_key)
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
        hash: &[u8],
        signature: &[u8],
        pub_key: &[u8],
    ) -> Result<bool, VerificationError> {
        match self.sign {
            SignatureAlg::Ed25519 => cosmwasm_crypto::ed25519_verify(hash, signature, pub_key)
                .map_err(VerificationError::from),
        }
    }
}
