use crate::credential::error::{InvalidCredentialError, InvalidProofError, VerificationError};
use crate::credential::proof::{Proof, ProofPurpose};
use crate::credential::rdf_marker::*;
use itertools::Itertools;
use okp4_rdf::dataset::Dataset;
use okp4_rdf::dataset::QuadIterator;
use rio_api::model::{BlankNode, Literal, NamedNode, Subject, Term};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct VerifiableCredential<'a> {
    id: &'a str,
    types: Vec<&'a str>,
    issuer: &'a str,
    issuance_date: &'a str,
    expiration_date: Option<&'a str>,
    claims: Vec<Claim<'a>>,
    status: Option<Status<'a>>,
    proof: Vec<Proof<'a>>,
    unsecured_document: Dataset<'a>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Claim<'a> {
    id: &'a str,
    content: Dataset<'a>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Status<'a> {
    id: &'a str,
    type_: &'a str,
    content: Dataset<'a>,
}

impl<'a> TryFrom<&'a Dataset<'a>> for VerifiableCredential<'a> {
    type Error = InvalidCredentialError;

    fn try_from(dataset: &'a Dataset<'a>) -> Result<Self, Self::Error> {
        let id = Self::extract_identifier(&dataset)?;

        let (proofs, proof_graphs): (Vec<Proof<'a>>, Vec<BlankNode<'a>>) =
            Self::extract_proofs(dataset, id)?.into_iter().unzip();

        if proofs.is_empty() {
            return Err(InvalidCredentialError::MissingProof);
        }

        let unsecured_filter = proof_graphs
            .into_iter()
            .map(|g| (None, None, None, Some(Some(g.into()))).into())
            .collect();

        Ok(Self {
            id: id.iri,
            types: Self::extract_types(dataset, id)?,
            issuer: Self::extract_issuer(&dataset, id)?.iri,
            issuance_date: Self::extract_issuance_date(&dataset, id)?,
            expiration_date: Self::extract_expiration_date(&dataset, id)?,
            claims: Self::extract_claims(dataset, id)?,
            status: Self::extract_status(dataset, id)?,
            proof: proofs,
            unsecured_document: Dataset::new(
                dataset
                    .iter()
                    .skip_patterns(unsecured_filter)
                    .map(|quad| *quad)
                    .collect(),
            ),
        })
    }
}

impl<'a> VerifiableCredential<'a> {
    pub fn verify(&self) -> Result<bool, VerificationError> {
        let proof = self
            .proof
            .iter()
            .find(|p| p.suitable(self.issuer, ProofPurpose::AssertionMethod))
            .ok_or(VerificationError::NoSuitableProof)?;

        let crypto_suite = proof.crypto_suite();
        crypto_suite.verify_document(
            self.unsecured_document.as_ref(),
            proof.value(),
            proof.pub_key(),
        )
    }

    fn extract_identifier(
        dataset: &'a Dataset<'a>,
    ) -> Result<NamedNode<'a>, InvalidCredentialError> {
        dataset
            .match_pattern(None, Some(RDF_TYPE), Some(VC_RDF_TYPE), None)
            .subjects()
            .exactly_one()
            .map_err(|e| match e.size_hint() {
                (_, Some(_)) => InvalidCredentialError::MissingIdentifier,
                _ => InvalidCredentialError::Malformed(
                    "Credential cannot have more than one id".to_string(),
                ),
            })
            .and_then(|s| match s {
                Subject::NamedNode(n) => Ok(n),
                _ => Err(InvalidCredentialError::Malformed(
                    "Credential identifier must be a named node".to_string(),
                )),
            })
    }

    fn extract_types(
        dataset: &'a Dataset<'a>,
        id: NamedNode<'a>,
    ) -> Result<Vec<&'a str>, InvalidCredentialError> {
        dataset
            .match_pattern(Some(id.into()), Some(RDF_TYPE), None, None)
            .objects()
            .map(|o| match o {
                Term::NamedNode(n) => Ok(n.iri),
                _ => Err(InvalidCredentialError::Malformed(
                    "Credential type must be a named node".to_string(),
                )),
            })
            .collect()
    }

    fn extract_issuer(
        dataset: &'a Dataset<'a>,
        id: NamedNode<'a>,
    ) -> Result<NamedNode<'a>, InvalidCredentialError> {
        dataset
            .match_pattern(Some(id.into()), Some(VC_RDF_ISSUER), None, None)
            .objects()
            .exactly_one()
            .map_err(|e| match e.size_hint() {
                (_, Some(_)) => InvalidCredentialError::MissingIssuer,
                _ => InvalidCredentialError::Malformed(
                    "Credential cannot have more than one issuer".to_string(),
                ),
            })
            .and_then(|o| match o {
                Term::NamedNode(n) => Ok(n),
                _ => Err(InvalidCredentialError::Malformed(
                    "Credential issuer must be a named node".to_string(),
                )),
            })
    }

    fn extract_issuance_date(
        dataset: &'a Dataset<'a>,
        id: NamedNode<'a>,
    ) -> Result<&'a str, InvalidCredentialError> {
        dataset
            .match_pattern(Some(id.into()), Some(VC_RDF_ISSUANCE_DATE), None, None)
            .objects()
            .exactly_one()
            .map_err(|e| match e.size_hint() {
                (_, Some(_)) => InvalidCredentialError::MissingIssuanceDate,
                _ => InvalidCredentialError::Malformed(
                    "Credential cannot have more than one issuance date".to_string(),
                ),
            })
            .and_then(|o| match o {
                Term::Literal(Literal::Typed { value, datatype }) if datatype == RDF_DATE_TYPE => {
                    Ok(value)
                }
                _ => Err(InvalidCredentialError::Malformed(
                    "Credential issuance date must be a date".to_string(),
                )),
            })
    }

    fn extract_expiration_date(
        dataset: &'a Dataset<'a>,
        id: NamedNode<'a>,
    ) -> Result<Option<&'a str>, InvalidCredentialError> {
        dataset
            .match_pattern(Some(id.into()), Some(VC_RDF_EXPIRATION_DATE), None, None)
            .objects()
            .at_most_one()
            .map_err(|_| {
                InvalidCredentialError::Malformed(
                    "Credential cannot have more than one expiration date".to_string(),
                )
            })
            .and_then(|o| match o {
                Some(t) => match t {
                    Term::Literal(Literal::Typed { value, datatype })
                        if datatype == RDF_DATE_TYPE =>
                    {
                        Ok(Some(value))
                    }
                    _ => Err(InvalidCredentialError::Malformed(
                        "Credential expiration date must be a date".to_string(),
                    )),
                },
                None => Ok(None),
            })
    }

    fn extract_claims(
        dataset: &'a Dataset<'a>,
        id: NamedNode<'a>,
    ) -> Result<Vec<Claim<'a>>, InvalidCredentialError> {
        dataset
            .match_pattern(Some(id.into()), Some(VC_RDF_CREDENTIAL_SUBJECT), None, None)
            .objects()
            .map(|claim_id| match claim_id {
                Term::NamedNode(n) => Ok(n),
                _ => Err(InvalidCredentialError::Malformed(
                    "Credential claim ids must be named nodes".to_string(),
                )),
            })
            .map_ok(|claim_id| Claim {
                id: claim_id.iri,
                content: Dataset::new(
                    dataset
                        .match_pattern(Some(claim_id.into()), None, None, None)
                        .map(|quad| *quad)
                        .collect(),
                ),
            })
            .collect()
    }

    fn extract_status(
        dataset: &'a Dataset<'a>,
        id: NamedNode<'a>,
    ) -> Result<Option<Status<'a>>, InvalidCredentialError> {
        dataset
            .match_pattern(Some(id.into()), Some(VC_RDF_CREDENTIAL_STATUS), None, None)
            .objects()
            .at_most_one()
            .map_err(|_| {
                InvalidCredentialError::Malformed(
                    "Credential cannot have more than one expiration date".to_string(),
                )
            })
            .and_then(|maybe_term| match maybe_term {
                Some(term) => match term {
                    Term::NamedNode(n) => Ok(Some(Status {
                        id: n.iri,
                        type_: Self::extract_types(dataset, n)?
                            .iter()
                            .exactly_one()
                            .map_err(|_| {
                                InvalidCredentialError::Malformed(
                                    "Credential status can only have one type".to_string(),
                                )
                            })?,
                        content: Dataset::new(
                            dataset
                                .match_pattern(Some(n.into()), None, None, None)
                                .map(|quad| *quad)
                                .collect(),
                        ),
                    })),
                    _ => Err(InvalidCredentialError::Malformed(
                        "Credential status id must be a named node".to_string(),
                    )),
                },
                None => Ok(None),
            })
    }

    fn extract_proofs(
        dataset: &'a Dataset<'a>,
        id: NamedNode<'a>,
    ) -> Result<Vec<(Proof<'a>, BlankNode<'a>)>, InvalidCredentialError> {
        dataset
            .match_pattern(Some(id.into()), Some(VC_RDF_PROOF), None, None)
            .objects()
            .map(|o| match o {
                Term::BlankNode(n) => {
                    let proof_res = Proof::try_from((dataset, n.into()));
                    match proof_res {
                        Err(e) if e == InvalidProofError::Unsupported => None,
                        _ => Some(
                            proof_res
                                .map(|p| (p, n))
                                .map_err(InvalidCredentialError::from),
                        ),
                    }
                }
                _ => Some(Err(InvalidCredentialError::Malformed(
                    "Credential proof must be encapsulated in blank node graph names".to_string(),
                ))),
            })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use okp4_rdf::serde::NQuadsReader;
    use rio_api::model::Quad;

    #[test]
    fn mescouilles() {
        let vc_raw = r#"<did:key:z6MkqxFfjh6HNFuNSGmqVDJxL4fcdbcBco7CNHBLjEo125wu> <https://schema.org/name> "Hometown Theatres, Inc." .
<did:v1:test:nym:z6MkhYBppZa2aD5xitZg3FbWLYPupRMAEecKFLQvmoYw8yEa> <https://schema.org/owns> _:b2 .
<https://vcplayground.org/credential/RK55U9YAbe28e_lDGcMnd> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://contexts.vcplayground.org/examples/movie-ticket/vocab#MovieTicketCredential> .
<https://vcplayground.org/credential/RK55U9YAbe28e_lDGcMnd> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.w3.org/2018/credentials#VerifiableCredential> .
<https://vcplayground.org/credential/RK55U9YAbe28e_lDGcMnd> <https://schema.org/description> "Admit one: Plan 9 from Outer Space, 3pm showing." .
<https://vcplayground.org/credential/RK55U9YAbe28e_lDGcMnd> <https://schema.org/image> <data:image/png;base64,iVBORw0KGgoA> .
<https://vcplayground.org/credential/RK55U9YAbe28e_lDGcMnd> <https://w3id.org/security#proof> _:b0 .
<https://vcplayground.org/credential/RK55U9YAbe28e_lDGcMnd> <https://www.w3.org/2018/credentials#credentialSubject> <did:v1:test:nym:z6MkhYBppZa2aD5xitZg3FbWLYPupRMAEecKFLQvmoYw8yEa> .
<https://vcplayground.org/credential/RK55U9YAbe28e_lDGcMnd> <https://www.w3.org/2018/credentials#issuanceDate> "2023-11-29T10:07:56.079Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<https://vcplayground.org/credential/RK55U9YAbe28e_lDGcMnd> <https://www.w3.org/2018/credentials#issuer> <did:key:z6MkqxFfjh6HNFuNSGmqVDJxL4fcdbcBco7CNHBLjEo125wu> .
_:b1 <http://purl.org/dc/terms/created> "2023-11-29T10:07:56Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> _:b0 .
_:b1 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519Signature2020> _:b0 .
_:b1 <https://w3id.org/security#proofPurpose> <https://w3id.org/security#assertionMethod> _:b0 .
_:b1 <https://w3id.org/security#proofValue> "z5UT4w3v6uSJ3srR3ZFSZBbgjaMRyEUaaGdnZzEb2oc1YTskkpff9qYt2GiTDuU2wqEh3f99YvWubPuqVNWrn9hNx"^^<https://w3id.org/security#multibase> _:b0 .
_:b1 <https://w3id.org/security#verificationMethod> <did:key:z6MkqxFfjh6HNFuNSGmqVDJxL4fcdbcBco7CNHBLjEo125wu#z6MkqxFfjh6HNFuNSGmqVDJxL4fcdbcBco7CNHBLjEo125wu> _:b0 .
_:b2 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://schema.org/Ticket> .
_:b2 <https://schema.org/location> _:b3 .
_:b2 <https://schema.org/startDate> "2022-08-26T19:00:00.000Z" .
_:b2 <https://schema.org/ticketNumber> "457812" .
_:b2 <https://schema.org/ticketToken> "urn:1a1e549a-2867" .
_:b2 <https://schema.org/ticketedSeat> _:b5 .
_:b3 <https://schema.org/PostalAddress> _:b4 .
_:b3 <https://schema.org/name> "Hometown Theatres, Inc." .
_:b4 <https://schema.org/addressLocality> "Your Town" .
_:b4 <https://schema.org/addressRegion> "VA" .
_:b4 <https://schema.org/postalCode> "24060" .
_:b4 <https://schema.org/streetAddress> "123 Main St." .
_:b5 <https://schema.org/seatNumber> "11" .
_:b5 <https://schema.org/seatRow> "E" .
_:b5 <https://schema.org/seatSection> "Theatre 3" ."#;

        let mut reader = NQuadsReader::new(vc_raw.as_bytes());
        let owned_quads = reader.read_all().unwrap();
        let quads: Vec<Quad<'_>> = owned_quads.iter().map(Quad::from).collect();
        let dataset = Dataset::new(quads);

        let vc = VerifiableCredential::try_from(&dataset);

        assert!(vc.is_ok());
        let vc = vc.unwrap();
        assert_eq!(
            vc.id,
            "https://vcplayground.org/credential/RK55U9YAbe28e_lDGcMnd"
        );

        let verif = vc.verify();
        assert!(verif.is_ok());
    }
}
