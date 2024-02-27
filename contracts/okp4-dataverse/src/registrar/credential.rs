use crate::credential::rdf_marker::IRI_VC_TYPE;
use crate::credential::vc::VerifiableCredential;
use crate::registrar::rdf::VC_CLAIM;
use crate::ContractError;
use cosmwasm_std::Addr;
use itertools::Itertools;
use rio_api::model::{BlankNode, NamedNode, Subject, Term, Triple};

#[derive(Debug, PartialEq)]
pub struct DataverseCredential<'a> {
    pub submitter_addr: Addr,
    pub id: &'a str,
    pub issuer: &'a str,
    pub r#type: &'a str,
    pub valid_from: &'a str,
    pub valid_until: Option<&'a str>,
    pub subject: &'a str,
    pub claim: Vec<Triple<'a>>,
}

impl<'a> DataverseCredential<'a> {
    fn extract_vc_type(vc: &'a VerifiableCredential<'a>) -> Result<&'a str, ContractError> {
        vc.types
            .iter()
            .filter(|t| *t != &IRI_VC_TYPE)
            .exactly_one()
            .map_err(|_| {
                ContractError::UnsupportedCredential(
                    "credential is expected to have exactly one type".to_string(),
                )
            })
            .copied()
    }

    fn extract_vc_claim(
        vc: &'a VerifiableCredential<'a>,
    ) -> Result<(&'a str, Vec<Triple<'a>>), ContractError> {
        //todo: use the canon identifier issuer instead and rename all blank nodes
        let claim_node = BlankNode { id: "c0" };

        let claim = vc.claims.iter().exactly_one().map_err(|_| {
            ContractError::UnsupportedCredential(
                "credential is expected to contain exactly one claim".to_string(),
            )
        })?;

        let mut triples = claim
            .content
            .iter()
            .map(|q| {
                let subject = match q.subject {
                    Subject::NamedNode(n) => {
                        if n.iri != claim.id {
                            Err(ContractError::UnsupportedCredential(
                                "claim hierarchy can be forge only through blank nodes".to_string(),
                            ))?;
                        }
                        Subject::BlankNode(claim_node)
                    }
                    _ => q.subject,
                };
                Ok(Triple {
                    subject,
                    predicate: q.predicate,
                    object: q.object,
                })
            })
            .collect::<Result<Vec<Triple<'a>>, ContractError>>()?;

        triples.push(Triple {
            subject: Subject::NamedNode(NamedNode { iri: vc.id }),
            predicate: VC_CLAIM,
            object: Term::BlankNode(BlankNode { id: "c0" }),
        });

        Ok((claim.id, triples))
    }
}

impl<'a> TryFrom<(Addr, &'a VerifiableCredential<'a>)> for DataverseCredential<'a> {
    type Error = ContractError;

    fn try_from(
        (submitter_addr, vc): (Addr, &'a VerifiableCredential<'a>),
    ) -> Result<Self, Self::Error> {
        let (subject, claim) = DataverseCredential::extract_vc_claim(vc)?;
        Ok(DataverseCredential {
            submitter_addr,
            id: vc.id,
            issuer: vc.issuer,
            r#type: DataverseCredential::extract_vc_type(vc)?,
            valid_from: vc.issuance_date,
            valid_until: vc.expiration_date,
            subject,
            claim,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testutil::testutil;
    use okp4_rdf::dataset::Dataset;
    use rio_api::model::Literal;

    #[test]
    fn proper_from_verifiable() {
        let owned_quads = testutil::read_test_quads("vc-valid.nq");
        let dataset = Dataset::from(owned_quads.as_slice());
        let vc = VerifiableCredential::try_from(&dataset).unwrap();
        let dc_res = DataverseCredential::try_from((
            Addr::unchecked("okp41072nc6egexqr2v6vpp7yxwm68plvqnkf6xsytf"),
            &vc,
        ));

        assert!(dc_res.is_ok());
        assert_eq!(dc_res.unwrap(), DataverseCredential {
                submitter_addr: Addr::unchecked("okp41072nc6egexqr2v6vpp7yxwm68plvqnkf6xsytf"),
                id: "https://w3id.org/okp4/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9",
                issuer: "did:key:zQ3shs7auhJSmVJpiUbQWco6bxxEhSqWnVEPvaBHBRvBKw6Q3",
                r#type: "https://w3id.org/okp4/ontology/vnext/schema/credential/digital-service/description/DigitalServiceDescriptionCredential",
                valid_from: "2024-01-22T00:00:00",
                valid_until: Some("2025-01-22T00:00:00"),
                subject: "did:key:zQ3shhb4SvzBRLbBonsvKb3WX6WoDeKWHpsXXXMhAJETqXAfB",
                claim: vec![Triple {
                    subject: BlankNode {id: "c0"}.into(),
                    predicate: NamedNode {iri: "https://w3id.org/okp4/ontology/vnext/schema/credential/digital-service/description/hasCategory"},
                    object: NamedNode {iri: "https://w3id.org/okp4/ontology/vnext/thesaurus/digital-service-category/Storage"}.into(),
                },Triple {
                    subject: BlankNode {id: "c0"}.into(),
                    predicate: NamedNode {iri: "https://w3id.org/okp4/ontology/vnext/schema/credential/digital-service/description/hasTag"},
                    object: Literal::Simple {value: "Cloud"}.into(),
                },Triple {
                    subject: NamedNode {iri: "https://w3id.org/okp4/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9"}.into(),
                    predicate: NamedNode {iri: "dataverse:credential#claim"},
                    object: BlankNode {id: "c0"}.into(),
                }],
            })
    }

    #[test]
    fn unsupported_from_verifiable() {
        let cases = vec![
            (
                "vc-unsupported-1.nq",
                "credential is expected to have exactly one type",
            ),
            (
                "vc-unsupported-2.nq",
                "credential is expected to have exactly one type",
            ),
            (
                "vc-unsupported-3.nq",
                "credential is expected to contain exactly one claim",
            ),
            (
                "vc-unsupported-4.nq",
                "claim hierarchy can be forge only through blank nodes",
            ),
        ];

        for case in cases {
            let owned_quads = testutil::read_test_quads(case.0);
            let dataset = Dataset::from(owned_quads.as_slice());
            let vc = VerifiableCredential::try_from(&dataset).unwrap();
            let dc_res = DataverseCredential::try_from((
                Addr::unchecked("okp41072nc6egexqr2v6vpp7yxwm68plvqnkf6xsytf"),
                &vc,
            ));

            assert!(dc_res.is_err());
            if let ContractError::UnsupportedCredential(msg) = dc_res.err().unwrap() {
                assert_eq!(msg, case.1.to_string());
            } else {
                assert!(false);
            }
        }
    }
}
