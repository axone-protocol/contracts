use crate::credential::crypto::{CanonicalizationAlg, CryptoSuite, DigestAlg, SignatureAlg};
use crate::credential::error::InvalidProofError;
use crate::credential::rdf_marker::{
    PROOF_RDF_PROOF_PURPOSE, PROOF_RDF_PROOF_VALUE, PROOF_RDF_PROOF_VALUE_TYPE,
    PROOF_RDF_VERIFICATION_METHOD, RDF_CREATED, RDF_DATE_TYPE, RDF_TYPE,
};
use itertools::Itertools;
use okp4_rdf::dataset::{Dataset, QuadIterator};
use rio_api::model::{GraphName, Literal, Term};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Proof<'a> {
    Ed25519Signature2020(Ed25519Signature2020Proof<'a>),
}

#[allow(dead_code)]
impl<'a> Proof<'a> {
    pub fn suitable(&self, issuer: &str, purpose: ProofPurpose) -> bool {
        match self {
            Self::Ed25519Signature2020(proof) => {
                proof.verification_method.controller == issuer && proof.purpose == purpose
            }
        }
    }

    pub fn crypto_suite(&self) -> CryptoSuite {
        match self {
            Proof::Ed25519Signature2020(_) => (
                CanonicalizationAlg::Urdna2015,
                DigestAlg::Sha256,
                SignatureAlg::Ed25519,
            )
                .into(),
        }
    }

    pub fn pub_key(&'a self) -> &'a [u8] {
        match self {
            Proof::Ed25519Signature2020(p) => &p.verification_method.pub_key,
        }
    }

    pub fn value(&'a self) -> &'a [u8] {
        match self {
            Proof::Ed25519Signature2020(p) => &p.value,
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
                (_, Some(_)) => InvalidProofError::MissingVerificationMethod,
                _ => InvalidProofError::Malformed(
                    "Proof cannot have more than one verification method".to_string(),
                ),
            })
            .and_then(|o| match o {
                Term::NamedNode(n) => Ok(n.iri),
                _ => Err(InvalidProofError::Malformed(
                    "verification method type must be a named node".to_string(),
                )),
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
                (_, Some(_)) => InvalidProofError::MissingCreated,
                _ => InvalidProofError::Malformed(
                    "Proof cannot have more than one created date".to_string(),
                ),
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
                (_, Some(_)) => InvalidProofError::MissingProofPurpose,
                _ => InvalidProofError::Malformed(
                    "Proof cannot have more than one proof purpose".to_string(),
                ),
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
                (_, Some(_)) => InvalidProofError::MissingProofValue,
                _ => InvalidProofError::Malformed(
                    "Proof cannot have more than one proof value".to_string(),
                ),
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
                (_, Some(_)) => InvalidProofError::MissingProofType,
                _ => {
                    InvalidProofError::Malformed("Proof cannot have more than one type".to_string())
                }
            })
            .and_then(|o| match o {
                Term::NamedNode(n) => Ok(n.iri),
                _ => Err(InvalidProofError::Malformed(
                    "Proof type must be a named node".to_string(),
                )),
            })?;

        match proof_type {
            "https://w3id.org/security#Ed25519Signature2020" => Ok(Self::Ed25519Signature2020(
                Ed25519Signature2020Proof::try_from((dataset, proof_graph))?,
            )),
            _ => Err(InvalidProofError::Unsupported),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Ed25519Signature2020Proof<'a> {
    verification_method: Ed25519VerificationKey2020<'a>,
    created: &'a str,
    purpose: ProofPurpose,
    value: Vec<u8>,
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
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Ed25519VerificationKey2020<'a> {
    id: &'a str,
    controller: &'a str,
    pub_key: Vec<u8>,
}

impl<'a> TryFrom<&'a str> for Ed25519VerificationKey2020<'a> {
    type Error = InvalidProofError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match value.split("#").collect::<Vec<_>>()[..] {
            [controller, key] => match controller.split(":").collect::<Vec<_>>()[..] {
                ["did", "key", controller_key] if controller_key == key => Self {
                    id: value,
                    controller,
                    pub_key: multiformats::decode_ed25519_key(key)?,
                },
                _ => Err(InvalidProofError::Malformed(
                    "couldn't parse did key for verification method".to_string(),
                ))?,
            },
            _ => Err(InvalidProofError::Malformed(
                "couldn't parse did key for verification method".to_string(),
            ))?,
        })
    }
}

#[derive(Debug, PartialEq)]
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

mod multiformats {
    use crate::credential::error::InvalidProofError;
    use multibase::Base;

    pub fn decode_ed25519_key(src: &str) -> Result<Vec<u8>, InvalidProofError> {
        let (base, data) = multibase::decode(src).map_err(|_| InvalidProofError::InvalidPubKey)?;
        if base != Base::Base58Btc {
            Err(InvalidProofError::InvalidPubKey)?
        }

        let (codec, key) =
            unsigned_varint::decode::u16(&data).map_err(|_| InvalidProofError::InvalidPubKey)?;
        if codec != 0xed {
            Err(InvalidProofError::InvalidPubKey)?
        }

        Ok(key.to_vec())
    }
}
