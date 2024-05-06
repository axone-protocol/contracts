use crate::credential::error::{InvalidCredentialError, InvalidProofError, VerificationError};
use crate::credential::proof::{Proof, ProofPurpose};
use crate::credential::rdf_marker::*;
use axone_rdf::dataset::QuadIterator;
use axone_rdf::dataset::{Dataset, QuadPattern};
use cosmwasm_std::DepsMut;
use itertools::Itertools;
use rio_api::model::{BlankNode, Literal, NamedNode, Subject, Term};

#[derive(Debug, PartialEq)]
pub struct VerifiableCredential<'a> {
    pub id: &'a str,
    pub types: Vec<&'a str>,
    pub issuer: &'a str,
    pub issuance_date: &'a str,
    pub expiration_date: Option<&'a str>,
    pub claims: Vec<Claim<'a>>,
    pub status: Option<Status<'a>>,
    pub proof: Vec<Proof<'a>>,
    unsecured_document: Dataset<'a>,
}

#[derive(Debug, PartialEq)]
pub struct Claim<'a> {
    pub id: &'a str,
    pub content: Dataset<'a>,
}

#[derive(Debug, PartialEq)]
pub struct Status<'a> {
    id: &'a str,
    type_: &'a str,
    content: Dataset<'a>,
}

impl<'a> TryFrom<&'a Dataset<'a>> for VerifiableCredential<'a> {
    type Error = InvalidCredentialError;

    fn try_from(dataset: &'a Dataset<'a>) -> Result<Self, Self::Error> {
        let id = Self::extract_identifier(dataset)?;

        let (proofs, proof_graphs): (Vec<Proof<'a>>, Vec<BlankNode<'a>>) =
            Self::extract_proofs(dataset, id)?.into_iter().unzip();

        let mut unsecured_filter: Vec<QuadPattern<'_>> = proof_graphs
            .into_iter()
            .map(|g| (None, None, None, Some(Some(g.into()))).into())
            .collect();

        unsecured_filter.push((Some(id.into()), Some(VC_RDF_PROOF), None, None).into());

        Ok(Self {
            id: id.iri,
            types: Self::extract_types(dataset, id)?,
            issuer: Self::extract_issuer(dataset, id)?.iri,
            issuance_date: Self::extract_issuance_date(dataset, id)?,
            expiration_date: Self::extract_expiration_date(dataset, id)?,
            claims: Self::extract_claims(dataset, id)?,
            status: Self::extract_status(dataset, id)?,
            proof: proofs,
            unsecured_document: Dataset::new(
                dataset
                    .iter()
                    .skip_patterns(unsecured_filter)
                    .copied()
                    .collect(),
            ),
        })
    }
}

impl<'a> VerifiableCredential<'a> {
    pub fn verify(&self, deps: &'_ DepsMut<'_>) -> Result<(), VerificationError> {
        let proof = self
            .proof
            .iter()
            .find(|p| p.suitable(self.issuer, ProofPurpose::AssertionMethod))
            .ok_or(VerificationError::NoSuitableProof)?;

        let crypto_suite = proof.crypto_suite();
        crypto_suite.verify_document(
            deps,
            self.unsecured_document.as_ref(),
            proof.options(),
            proof.proof_material(),
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
                (_, Some(_)) => InvalidCredentialError::Malformed(
                    "Credential cannot have more than one id".to_string(),
                ),
                _ => InvalidCredentialError::MissingIdentifier,
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
                content: dataset.sub_graph(claim_id.into()),
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
                                .copied()
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
            .filter_map(|o| match o {
                Term::BlankNode(n) => {
                    let proof_res = Proof::try_from((dataset, n.into()));
                    match proof_res {
                        Err(InvalidProofError::Unsupported) => None,
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
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testutil::testutil;
    use cosmwasm_std::testing::mock_dependencies;
    use rio_api::model::Quad;

    #[test]
    fn proper_vc_from_dataset() {
        let owned_quads = testutil::read_test_quads("vc-eddsa-2020-ok-unsecured.nq");
        let unsecure_dataset = Dataset::from(owned_quads.as_slice());

        let owned_quads = testutil::read_test_quads("vc-eddsa-2020-ok.nq");
        let dataset = Dataset::from(owned_quads.as_slice());

        let vc_res = VerifiableCredential::try_from(&dataset);
        assert!(vc_res.is_ok());
        let vc = vc_res.unwrap();
        assert_eq!(vc.id, "http://example.edu/credentials/3732");
        assert_eq!(
            vc.types,
            vec![
                "https://example.org/examples#UniversityDegreeCredential",
                "https://www.w3.org/2018/credentials#VerifiableCredential"
            ]
        );
        assert_eq!(
            vc.issuer,
            "did:key:z6MkpwdnLPAm4apwcrRYQ6fZ3rAcqjLZR4AMk14vimfnozqY"
        );
        assert_eq!(vc.issuance_date, "2024-02-16T00:00:00Z");
        assert_eq!(vc.expiration_date, Some("2026-02-16T00:00:00Z"));
        assert_eq!(
            vc.claims,
            vec![Claim {
                id: "did:key:zDnaeUm3QkcyZWZTPttxB711jgqRDhkwvhF485SFw1bDZ9AQw",
                content: Dataset::new(vec![
                    Quad {
                        subject: NamedNode {
                            iri: "did:key:zDnaeUm3QkcyZWZTPttxB711jgqRDhkwvhF485SFw1bDZ9AQw"
                        }
                        .into(),
                        predicate: NamedNode {
                            iri: "https://example.org/examples#degree"
                        },
                        object: BlankNode { id: "b2" }.into(),
                        graph_name: None
                    },
                    Quad {
                        subject: BlankNode { id: "b2" }.into(),
                        predicate: NamedNode {
                            iri: "http://schema.org/name"
                        },
                        object: Literal::Typed {
                            value: "Bachelor of Science and Arts",
                            datatype: NamedNode {
                                iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML"
                            }
                        }
                        .into(),
                        graph_name: None
                    },
                    Quad {
                        subject: BlankNode { id: "b2" }.into(),
                        predicate: NamedNode {
                            iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
                        },
                        object: NamedNode {
                            iri: "https://example.org/examples#BachelorDegree"
                        }
                        .into(),
                        graph_name: None
                    }
                ])
            }]
        );
        assert_eq!(vc.status, None);
        assert_eq!(vc.proof.len(), 1usize);
        assert_eq!(vc.unsecured_document, unsecure_dataset);
    }

    #[test]
    fn vc_verify() {
        let cases = vec![
            "vc-eddsa-2018-ok.nq",
            "vc-eddsa-2020-ok.nq",
            "vc-ecdsa-2019-ok.nq",
            "vc-di-ed-ok.nq",
        ];
        let mut deps = mock_dependencies();

        for case in cases {
            let owned_quads = testutil::read_test_quads(case);
            let dataset = Dataset::from(owned_quads.as_slice());
            let vc = VerifiableCredential::try_from(&dataset).unwrap();
            let verif_res = vc.verify(&deps.as_mut());
            assert!(verif_res.is_ok());
        }
    }
}
