use crate::credential::crypto::{CanonicalizationAlg, CryptoSuite, DigestAlg, SignatureAlg};
use crate::credential::error::InvalidProofError;
use crate::credential::rdf_marker::{
    PROOF_RDF_CRYPTOSUITE, PROOF_RDF_JWS, PROOF_RDF_PROOF_PURPOSE, PROOF_RDF_PROOF_VALUE,
    PROOF_RDF_PROOF_VALUE_TYPE, PROOF_RDF_VERIFICATION_METHOD, RDF_CREATED, RDF_DATE_TYPE,
    RDF_TYPE,
};
use axone_rdf::dataset::{Dataset, QuadIterator};
use itertools::Itertools;
use rio_api::model::{GraphName, Literal, NamedNode, Quad, Term};

#[derive(Debug, PartialEq)]
pub enum Proof<'a> {
    Ed25519Signature2018(Ed25519Signature2018Proof<'a>),
    Ed25519Signature2020(Ed25519Signature2020Proof<'a>),
    EcdsaSecp256k1Signature2019(EcdsaSecp256k1Signature2019Proof<'a>),
    DataIntegrity(DataIntegrityProof<'a>),
}

#[derive(Debug, PartialEq)]
pub enum ProofMaterial<'a> {
    Signature(&'a [u8]),
    Jws(&'a str),
}

impl<'a> Proof<'a> {
    pub fn suitable(&self, issuer: &str, purpose: ProofPurpose) -> bool {
        let (controller, proof_purpose) = match self {
            Self::Ed25519Signature2018(proof) => {
                (proof.verification_method.controller, proof.purpose)
            }
            Self::Ed25519Signature2020(proof) => {
                (proof.verification_method.controller, proof.purpose)
            }
            Self::EcdsaSecp256k1Signature2019(proof) => {
                (proof.verification_method.controller, proof.purpose)
            }
            Proof::DataIntegrity(proof) => (proof.verification_method.controller, proof.purpose),
        };

        controller == issuer && proof_purpose == purpose
    }

    pub fn crypto_suite(&self) -> CryptoSuite {
        match self {
            Proof::Ed25519Signature2018(_) | Proof::Ed25519Signature2020(_) => (
                CanonicalizationAlg::Urdna2015,
                DigestAlg::Sha256,
                SignatureAlg::Ed25519,
            ),
            Proof::EcdsaSecp256k1Signature2019(_) => (
                CanonicalizationAlg::Urdna2015,
                DigestAlg::Sha256,
                SignatureAlg::Secp256k1,
            ),
            Proof::DataIntegrity(p) => (
                CanonicalizationAlg::Urdna2015,
                DigestAlg::Sha256,
                p.cryptosuite.into(),
            ),
        }
        .into()
    }

    pub fn pub_key(&'a self) -> &'a [u8] {
        match self {
            Proof::Ed25519Signature2018(p) => &p.verification_method.pub_key,
            Proof::Ed25519Signature2020(p) => &p.verification_method.pub_key,
            Proof::EcdsaSecp256k1Signature2019(p) => &p.verification_method.pub_key,
            Proof::DataIntegrity(p) => &p.verification_method.pub_key,
        }
    }

    pub fn proof_material(&'a self) -> ProofMaterial<'a> {
        match self {
            Proof::Ed25519Signature2018(p) => ProofMaterial::Jws(p.jws),
            Proof::Ed25519Signature2020(p) => ProofMaterial::Signature(p.value.as_slice()),
            Proof::EcdsaSecp256k1Signature2019(p) => ProofMaterial::Jws(p.jws),
            Proof::DataIntegrity(p) => ProofMaterial::Signature(p.value.as_slice()),
        }
    }

    pub fn options(&'a self) -> &'a [Quad<'a>] {
        match self {
            Proof::Ed25519Signature2018(p) => p.options.as_ref(),
            Proof::Ed25519Signature2020(p) => p.options.as_ref(),
            Proof::EcdsaSecp256k1Signature2019(p) => p.options.as_ref(),
            Proof::DataIntegrity(p) => p.options.as_ref(),
        }
    }

    fn extract_verification_method(
        dataset: &'a Dataset<'a>,
        proof_graph: GraphName<'a>,
    ) -> Result<&'a str, InvalidProofError> {
        dataset
            .match_pattern(
                None,
                Some(PROOF_RDF_VERIFICATION_METHOD),
                None,
                Some(Some(proof_graph)),
            )
            .objects()
            .exactly_one()
            .map_err(|e| match e.size_hint() {
                (_, Some(_)) => InvalidProofError::Malformed(
                    "Proof cannot have more than one verification method".to_string(),
                ),
                _ => InvalidProofError::MissingVerificationMethod,
            })
            .and_then(|o| match o {
                Term::NamedNode(n) => Ok(n.iri),
                _ => Err(InvalidProofError::Malformed(
                    "verification method type must be a named node".to_string(),
                )),
            })
    }

    fn parse_verification_method(raw: &'a str) -> Result<(&'a str, &'a str), InvalidProofError> {
        Ok(match raw.split('#').collect::<Vec<_>>()[..] {
            [controller, key] => match controller.split(':').collect::<Vec<_>>()[..] {
                ["did", "key", controller_key] if controller_key == key => (controller, key),
                _ => Err(InvalidProofError::Malformed(
                    "couldn't parse did key for verification method".to_string(),
                ))?,
            },
            _ => Err(InvalidProofError::Malformed(
                "couldn't parse did key for verification method".to_string(),
            ))?,
        })
    }

    fn extract_created(
        dataset: &'a Dataset<'a>,
        proof_graph: GraphName<'a>,
    ) -> Result<&'a str, InvalidProofError> {
        dataset
            .match_pattern(None, Some(RDF_CREATED), None, Some(Some(proof_graph)))
            .objects()
            .exactly_one()
            .map_err(|e| match e.size_hint() {
                (_, Some(_)) => InvalidProofError::Malformed(
                    "Proof cannot have more than one created date".to_string(),
                ),
                _ => InvalidProofError::MissingCreated,
            })
            .and_then(|o| match o {
                Term::Literal(Literal::Typed { value, datatype }) if datatype == RDF_DATE_TYPE => {
                    Ok(value)
                }
                _ => Err(InvalidProofError::Malformed(
                    "Proof created date must be a date".to_string(),
                )),
            })
    }

    fn extract_proof_purpose(
        dataset: &'a Dataset<'a>,
        proof_graph: GraphName<'a>,
    ) -> Result<&'a str, InvalidProofError> {
        dataset
            .match_pattern(
                None,
                Some(PROOF_RDF_PROOF_PURPOSE),
                None,
                Some(Some(proof_graph)),
            )
            .objects()
            .exactly_one()
            .map_err(|e| match e.size_hint() {
                (_, Some(_)) => InvalidProofError::Malformed(
                    "Proof cannot have more than one proof purpose".to_string(),
                ),
                _ => InvalidProofError::MissingProofPurpose,
            })
            .and_then(|o| match o {
                Term::NamedNode(n) => Ok(n.iri),
                _ => Err(InvalidProofError::Malformed(
                    "proof purpose type must be a named node".to_string(),
                )),
            })
    }

    fn extract_proof_value(
        dataset: &'a Dataset<'a>,
        proof_graph: GraphName<'a>,
    ) -> Result<&'a str, InvalidProofError> {
        dataset
            .match_pattern(
                None,
                Some(PROOF_RDF_PROOF_VALUE),
                None,
                Some(Some(proof_graph)),
            )
            .objects()
            .exactly_one()
            .map_err(|e| match e.size_hint() {
                (_, Some(_)) => InvalidProofError::Malformed(
                    "Proof cannot have more than one proof value".to_string(),
                ),
                _ => InvalidProofError::MissingProofValue,
            })
            .and_then(|o| match o {
                Term::Literal(Literal::Typed { value, datatype })
                    if datatype == PROOF_RDF_PROOF_VALUE_TYPE =>
                {
                    Ok(value)
                }
                _ => Err(InvalidProofError::Malformed(
                    "Proof value must be a multibase".to_string(),
                )),
            })
    }

    fn extract_jws(
        dataset: &'a Dataset<'a>,
        proof_graph: GraphName<'a>,
    ) -> Result<&'a str, InvalidProofError> {
        dataset
            .match_pattern(None, Some(PROOF_RDF_JWS), None, Some(Some(proof_graph)))
            .objects()
            .exactly_one()
            .map_err(|e| match e.size_hint() {
                (_, Some(_)) => InvalidProofError::Malformed(
                    "Proof cannot have more than one proof jws".to_string(),
                ),
                _ => InvalidProofError::MissingProofValue,
            })
            .and_then(|o| match o {
                Term::Literal(Literal::Simple { value }) => Ok(value),
                _ => Err(InvalidProofError::Malformed(
                    "Proof jws must be a string".to_string(),
                )),
            })
    }

    fn extract_proof_options(
        dataset: &'a Dataset<'a>,
        proof_graph: GraphName<'a>,
        value_predicate: NamedNode<'a>,
    ) -> Dataset<'a> {
        Dataset::new(
            dataset
                .match_pattern(None, None, None, Some(Some(proof_graph)))
                .skip_pattern((None, Some(value_predicate), None, None).into())
                .map(|quad| Quad {
                    subject: quad.subject,
                    predicate: quad.predicate,
                    object: quad.object,
                    graph_name: None,
                })
                .collect(),
        )
    }
}

impl<'a> TryFrom<(&'a Dataset<'a>, GraphName<'a>)> for Proof<'a> {
    type Error = InvalidProofError;

    fn try_from(
        (dataset, proof_graph): (&'a Dataset<'a>, GraphName<'a>),
    ) -> Result<Self, Self::Error> {
        let proof_type = dataset
            .match_pattern(None, Some(RDF_TYPE), None, Some(Some(proof_graph)))
            .objects()
            .exactly_one()
            .map_err(|e| match e.size_hint() {
                (_, Some(_)) => {
                    InvalidProofError::Malformed("Proof cannot have more than one type".to_string())
                }
                _ => InvalidProofError::MissingProofType,
            })
            .and_then(|o| match o {
                Term::NamedNode(n) => Ok(n.iri),
                _ => Err(InvalidProofError::Malformed(
                    "Proof type must be a named node".to_string(),
                )),
            })?;

        match proof_type {
            "https://w3id.org/security#Ed25519Signature2018" => Ok(Self::Ed25519Signature2018(
                Ed25519Signature2018Proof::try_from((dataset, proof_graph))?,
            )),
            "https://w3id.org/security#Ed25519Signature2020" => Ok(Self::Ed25519Signature2020(
                Ed25519Signature2020Proof::try_from((dataset, proof_graph))?,
            )),
            "https://w3id.org/security#EcdsaSecp256k1Signature2019" => {
                Ok(Self::EcdsaSecp256k1Signature2019(
                    EcdsaSecp256k1Signature2019Proof::try_from((dataset, proof_graph))?,
                ))
            }
            "https://w3id.org/security#DataIntegrityProof" => Ok(Self::DataIntegrity(
                DataIntegrityProof::try_from((dataset, proof_graph))?,
            )),
            _ => Err(InvalidProofError::Unsupported),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ProofPurpose {
    AssertionMethod,
    Unused,
}

impl<'a> From<&'a str> for ProofPurpose {
    fn from(value: &'a str) -> Self {
        match value {
            "https://w3id.org/security#assertionMethod" => ProofPurpose::AssertionMethod,
            _ => ProofPurpose::Unused,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Ed25519Signature2018Proof<'a> {
    // The verification method format being the same as the 2020 signature proof we reuse it.
    verification_method: Ed25519VerificationKey2020<'a>,
    created: &'a str,
    purpose: ProofPurpose,
    jws: &'a str,
    options: Dataset<'a>,
}

impl<'a> TryFrom<(&'a Dataset<'a>, GraphName<'a>)> for Ed25519Signature2018Proof<'a> {
    type Error = InvalidProofError;

    fn try_from(
        (dataset, proof_graph): (&'a Dataset<'a>, GraphName<'a>),
    ) -> Result<Self, Self::Error> {
        let v_method = Proof::extract_verification_method(dataset, proof_graph)?;
        let p_purpose = Proof::extract_proof_purpose(dataset, proof_graph)?;

        Ok(Self {
            verification_method: v_method.try_into()?,
            created: Proof::extract_created(dataset, proof_graph)?,
            purpose: p_purpose.into(),
            jws: Proof::extract_jws(dataset, proof_graph)?,
            options: Proof::extract_proof_options(dataset, proof_graph, PROOF_RDF_JWS),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Ed25519Signature2020Proof<'a> {
    verification_method: Ed25519VerificationKey2020<'a>,
    created: &'a str,
    purpose: ProofPurpose,
    value: Vec<u8>,
    options: Dataset<'a>,
}

impl<'a> TryFrom<(&'a Dataset<'a>, GraphName<'a>)> for Ed25519Signature2020Proof<'a> {
    type Error = InvalidProofError;

    fn try_from(
        (dataset, proof_graph): (&'a Dataset<'a>, GraphName<'a>),
    ) -> Result<Self, Self::Error> {
        let v_method = Proof::extract_verification_method(dataset, proof_graph)?;
        let p_purpose = Proof::extract_proof_purpose(dataset, proof_graph)?;
        let (_, p_value) = multibase::decode(Proof::extract_proof_value(dataset, proof_graph)?)
            .map_err(InvalidProofError::from)?;

        Ok(Self {
            verification_method: v_method.try_into()?,
            created: Proof::extract_created(dataset, proof_graph)?,
            purpose: p_purpose.into(),
            value: p_value,
            options: Proof::extract_proof_options(dataset, proof_graph, PROOF_RDF_PROOF_VALUE),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Ed25519VerificationKey2020<'a> {
    id: &'a str,
    controller: &'a str,
    pub_key: Vec<u8>,
}

impl<'a> TryFrom<&'a str> for Ed25519VerificationKey2020<'a> {
    type Error = InvalidProofError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (controller, key) = Proof::parse_verification_method(value)?;
        Ok(Self {
            id: value,
            controller,
            pub_key: multiformats::decode_ed25519_key(key)?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct EcdsaSecp256k1Signature2019Proof<'a> {
    verification_method: EcdsaSecp256k1VerificationKey2019<'a>,
    created: &'a str,
    purpose: ProofPurpose,
    jws: &'a str,
    options: Dataset<'a>,
}

impl<'a> TryFrom<(&'a Dataset<'a>, GraphName<'a>)> for EcdsaSecp256k1Signature2019Proof<'a> {
    type Error = InvalidProofError;

    fn try_from(
        (dataset, proof_graph): (&'a Dataset<'a>, GraphName<'a>),
    ) -> Result<Self, Self::Error> {
        let v_method = Proof::extract_verification_method(dataset, proof_graph)?;
        let p_purpose = Proof::extract_proof_purpose(dataset, proof_graph)?;

        Ok(Self {
            verification_method: v_method.try_into()?,
            created: Proof::extract_created(dataset, proof_graph)?,
            purpose: p_purpose.into(),
            jws: Proof::extract_jws(dataset, proof_graph)?,
            options: Proof::extract_proof_options(dataset, proof_graph, PROOF_RDF_JWS),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct EcdsaSecp256k1VerificationKey2019<'a> {
    id: &'a str,
    controller: &'a str,
    pub_key: Vec<u8>,
}

impl<'a> TryFrom<&'a str> for EcdsaSecp256k1VerificationKey2019<'a> {
    type Error = InvalidProofError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (controller, key) = Proof::parse_verification_method(value)?;
        Ok(Self {
            id: value,
            controller,
            pub_key: multiformats::decode_secp256k1_key(key)?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct DataIntegrityProof<'a> {
    cryptosuite: DataIntegrityCryptoSuite,
    verification_method: Multikey<'a>,
    created: &'a str,
    purpose: ProofPurpose,
    value: Vec<u8>,
    options: Dataset<'a>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DataIntegrityCryptoSuite {
    EddsaRdfc2022,
}

impl From<DataIntegrityCryptoSuite> for SignatureAlg {
    fn from(value: DataIntegrityCryptoSuite) -> Self {
        match value {
            DataIntegrityCryptoSuite::EddsaRdfc2022 => SignatureAlg::Ed25519,
        }
    }
}

impl<'a> DataIntegrityProof<'a> {
    fn extract_cryptosuite(
        dataset: &'a Dataset<'a>,
        proof_graph: GraphName<'a>,
    ) -> Result<DataIntegrityCryptoSuite, InvalidProofError> {
        dataset
            .match_pattern(
                None,
                Some(PROOF_RDF_CRYPTOSUITE),
                None,
                Some(Some(proof_graph)),
            )
            .objects()
            .exactly_one()
            .map_err(|e| match e.size_hint() {
                (_, Some(_)) => InvalidProofError::Malformed(
                    "Proof cannot have more than one proof cryptosuite".to_string(),
                ),
                _ => InvalidProofError::MissingProofCryptosuite,
            })
            .and_then(|o| match o {
                Term::Literal(Literal::Simple { value })
                | Term::Literal(Literal::Typed { value, datatype: _ }) => Ok(value),
                _ => Err(InvalidProofError::Malformed(
                    "Proof cryptosuite must be a cryptosuite string".to_string(),
                )),
            })
            .and_then(|suite| {
                Ok(match suite {
                    "eddsa-rdfc-2022" | "eddsa-2022" => DataIntegrityCryptoSuite::EddsaRdfc2022,
                    _ => Err(InvalidProofError::Malformed(
                        "Proof cryptosuite unknown or unsupported".to_string(),
                    ))?,
                })
            })
    }
}

impl<'a> TryFrom<(&'a Dataset<'a>, GraphName<'a>)> for DataIntegrityProof<'a> {
    type Error = InvalidProofError;

    fn try_from(
        (dataset, proof_graph): (&'a Dataset<'a>, GraphName<'a>),
    ) -> Result<Self, Self::Error> {
        let cryptosuite = DataIntegrityProof::extract_cryptosuite(dataset, proof_graph)?;
        let v_method = Proof::extract_verification_method(dataset, proof_graph)?;
        let p_purpose = Proof::extract_proof_purpose(dataset, proof_graph)?;
        let (_, p_value) = multibase::decode(Proof::extract_proof_value(dataset, proof_graph)?)
            .map_err(InvalidProofError::from)?;

        Ok(Self {
            cryptosuite,
            verification_method: (v_method, cryptosuite).try_into()?,
            created: Proof::extract_created(dataset, proof_graph)?,
            purpose: p_purpose.into(),
            value: p_value,
            options: Proof::extract_proof_options(dataset, proof_graph, PROOF_RDF_PROOF_VALUE),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Multikey<'a> {
    id: &'a str,
    controller: &'a str,
    pub_key: Vec<u8>,
}

impl<'a> TryFrom<(&'a str, DataIntegrityCryptoSuite)> for Multikey<'a> {
    type Error = InvalidProofError;

    fn try_from(
        (value, cryptosuite): (&'a str, DataIntegrityCryptoSuite),
    ) -> Result<Self, Self::Error> {
        let (controller, key) = Proof::parse_verification_method(value)?;
        Ok(Self {
            id: value,
            controller,
            pub_key: match cryptosuite {
                DataIntegrityCryptoSuite::EddsaRdfc2022 => multiformats::decode_ed25519_key(key),
            }?,
        })
    }
}

mod multiformats {
    use crate::credential::error::InvalidProofError;
    use multibase::Base;

    pub fn decode_ed25519_key(src: &str) -> Result<Vec<u8>, InvalidProofError> {
        let (base, data) = multibase::decode(src).map_err(|_| InvalidProofError::InvalidPubKey)?;
        if base != Base::Base58Btc {
            Err(InvalidProofError::InvalidPubKey)?;
        }

        let (codec, key) =
            unsigned_varint::decode::u16(&data).map_err(|_| InvalidProofError::InvalidPubKey)?;
        if codec != 0xed {
            Err(InvalidProofError::InvalidPubKey)?;
        }

        Ok(key.to_vec())
    }

    pub fn decode_secp256k1_key(src: &str) -> Result<Vec<u8>, InvalidProofError> {
        let (base, data) = multibase::decode(src).map_err(|_| InvalidProofError::InvalidPubKey)?;
        if base != Base::Base58Btc {
            Err(InvalidProofError::InvalidPubKey)?;
        }

        let (codec, key) =
            unsigned_varint::decode::u16(&data).map_err(|_| InvalidProofError::InvalidPubKey)?;
        if codec != 0xe7 {
            Err(InvalidProofError::InvalidPubKey)?;
        }

        Ok(key.to_vec())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testutil::testutil;
    use base64::prelude::BASE64_STANDARD;
    use base64::Engine;
    use rio_api::model::BlankNode;

    #[test]
    fn proof_from_dataset() {
        let quads = testutil::read_test_quads("proof-ed255192020-options.nq");
        let proof_ok_options = Dataset::from(quads.as_slice());

        let cases: Vec<(&str, Result<Proof<'_>, InvalidProofError>)> = vec![
            (
                "proof-ed255192020-ok.nq",
                Ok(Proof::Ed25519Signature2020(Ed25519Signature2020Proof {
                    created: "2023-11-29T10:07:56Z",
                    verification_method: Ed25519VerificationKey2020 {
                        id: "did:key:z6MkqxFfjh6HNFuNSGmqVDJxL4fcdbcBco7CNHBLjEo125wu#z6MkqxFfjh6HNFuNSGmqVDJxL4fcdbcBco7CNHBLjEo125wu",
                        controller: "did:key:z6MkqxFfjh6HNFuNSGmqVDJxL4fcdbcBco7CNHBLjEo125wu",
                        pub_key: BASE64_STANDARD.decode("qt35Ph/BPVyvU0YhVdJ47m0p6APFYPoC5V5C7s5cdyg=").unwrap(),
                    },
                    purpose: ProofPurpose::AssertionMethod,
                    value: BASE64_STANDARD.decode("371GN4kfgVEWv3/QY9qx1buNm9gYJGWgYOgMSVKOsnoJekPoQV2fjqR+3XMjd3avpQlARFyD/3a0J5tUS4aBCQ==").unwrap(),
                    options: proof_ok_options,
                })),
            ),
            (
                "proof-invalid-pkey.nq",
                Err(InvalidProofError::InvalidPubKey),
            ),
            (
                "proof-malformed.nq",
                Err(InvalidProofError::Malformed("Proof type must be a named node".to_string())),
            ),
            (
                "proof-malformed-value.nq",
                Err(InvalidProofError::MalformedProofValue(multibase::Error::UnknownBase('5'))),
            ),
            (
                "proof-missing-created.nq",
                Err(InvalidProofError::MissingCreated),
            ),
            (
                "proof-missing-method.nq",
                Err(InvalidProofError::MissingVerificationMethod),
            ),
            (
                "proof-missing-purpose.nq",
                Err(InvalidProofError::MissingProofPurpose),
            ),
            (
                "proof-missing-type.nq",
                Err(InvalidProofError::MissingProofType),
            ),
            (
                "proof-missing-value.nq",
                Err(InvalidProofError::MissingProofValue),
            ),
            (
                "proof-unsupported.nq",
                Err(InvalidProofError::Unsupported),
            ),
        ];

        for (test_file, expected) in cases {
            let owned_quads = testutil::read_test_quads(test_file);
            let dataset = Dataset::from(owned_quads.as_slice());

            let proof_res =
                Proof::try_from((&dataset, GraphName::BlankNode(BlankNode { id: "b0" })));
            assert_eq!(proof_res, expected)
        }
    }
}
