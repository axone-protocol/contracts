use crate::credential::rdf_marker::RDF_DATE_TYPE;
use crate::registrar::credential::DataverseCredential;
use crate::ContractError;
use axone_rdf::dataset::QuadIterator;
use axone_rdf::normalize::IdentifierIssuer;
use axone_rdf::serde::{DataFormat, TripleWriter};
use cosmwasm_std::{Binary, StdError};
use rio_api::model::{BlankNode, Literal, NamedNode, Subject, Term, Triple};

pub const VC_RESERVED_PREDICATES: &[NamedNode<'_>] = &[
    VC_SUBMITTER_ADDRESS,
    VC_TYPE,
    VC_ISSUER,
    VC_VALID_FROM,
    VC_VALID_UNTIL,
    VC_SUBJECT,
    VC_CLAIM,
    VC_CLAIM_ORIGINAL_NODE,
];

pub const VC_SUBMITTER_ADDRESS: NamedNode<'_> = NamedNode {
    iri: "dataverse:credential#submitterAddress",
};
pub const VC_TYPE: NamedNode<'_> = NamedNode {
    iri: "dataverse:credential#type",
};
pub const VC_ISSUER: NamedNode<'_> = NamedNode {
    iri: "dataverse:credential#issuer",
};
pub const VC_VALID_FROM: NamedNode<'_> = NamedNode {
    iri: "dataverse:credential#validFrom",
};
pub const VC_VALID_UNTIL: NamedNode<'_> = NamedNode {
    iri: "dataverse:credential#validUntil",
};
pub const VC_SUBJECT: NamedNode<'_> = NamedNode {
    iri: "dataverse:credential#subject",
};
pub const VC_CLAIM: NamedNode<'_> = NamedNode {
    iri: "dataverse:credential#claim",
};

/// Used when a claim triple contains a named node as object to establish a hierarchy, we replace this hierarchical link
/// with a blank node, and this predicate is used to allow the reconciliation with the original named node.  
pub const VC_CLAIM_ORIGINAL_NODE: NamedNode<'_> = NamedNode {
    iri: "dataverse:claim#original-node",
};

impl<'a> DataverseCredential<'a> {
    pub fn serialize(&self, format: DataFormat) -> Result<Binary, ContractError> {
        if self.contains_reserved_predicates() {
            Err(ContractError::UnsupportedCredential(
                "Claim contains reserved predicates.".to_string(),
            ))?;
        }

        let claim_node = BlankNode { id: "c0" };
        // Used to rename all blank nodes to avoid conflict with the forged claim node `c0`
        let mut blank_issuer = IdentifierIssuer::new("b", 0u128);
        // Used to replace named node based hierarchy with blank nodes
        let mut named_issuer = IdentifierIssuer::new("a", 0u128);
        let triples: Vec<Triple<'_>> =
            self.as_triples(claim_node, &mut named_issuer, &mut blank_issuer)?;
        let out: Vec<u8> = Vec::default();
        let mut writer = TripleWriter::new(&format, out);
        for triple in triples {
            writer.write(&triple).map_err(|e| {
                StdError::serialize_err("triple", format!("Error writing triple: {e}"))
            })?;
        }

        Ok(Binary::from(writer.finish().map_err(|e| {
            StdError::serialize_err("triple", format!("Error writing triple: {e}"))
        })?))
    }

    fn as_triples(
        &'a self,
        claim_node: BlankNode<'a>,
        named_issuer: &'a mut IdentifierIssuer,
        blank_issuer: &'a mut IdentifierIssuer,
    ) -> Result<Vec<Triple<'a>>, ContractError> {
        let c_subject = Subject::NamedNode(NamedNode { iri: self.id });

        let mut triples = vec![
            Triple {
                subject: c_subject,
                predicate: VC_SUBMITTER_ADDRESS,
                object: Term::Literal(Literal::Simple {
                    value: self.submitter_addr.as_str(),
                }),
            },
            Triple {
                subject: c_subject,
                predicate: VC_ISSUER,
                object: Term::NamedNode(NamedNode { iri: self.issuer }),
            },
            Triple {
                subject: c_subject,
                predicate: VC_TYPE,
                object: Term::NamedNode(NamedNode { iri: self.r#type }),
            },
            Triple {
                subject: c_subject,
                predicate: VC_VALID_FROM,
                object: Term::Literal(Literal::Typed {
                    value: self.valid_from,
                    datatype: RDF_DATE_TYPE,
                }),
            },
            Triple {
                subject: c_subject,
                predicate: VC_SUBJECT,
                object: Term::NamedNode(NamedNode { iri: self.claim.id }),
            },
        ];

        triples.extend(self.claim_as_triples(claim_node, named_issuer, blank_issuer)?);

        if let Some(valid_until) = self.valid_until {
            triples.push(Triple {
                subject: c_subject,
                predicate: VC_VALID_UNTIL,
                object: Term::Literal(Literal::Typed {
                    value: valid_until,
                    datatype: RDF_DATE_TYPE,
                }),
            });
        }

        Ok(triples)
    }

    fn claim_as_triples(
        &'a self,
        claim_node: BlankNode<'a>,
        named_issuer: &'a mut IdentifierIssuer,
        blank_issuer: &'a mut IdentifierIssuer,
    ) -> Result<Vec<Triple<'a>>, ContractError> {
        // issue replacement identifiers for nodes
        self.claim.content.iter().for_each(|q| {
            match q.subject {
                Subject::NamedNode(NamedNode { iri }) if iri != self.claim.id => {
                    named_issuer.get_or_issue(iri.to_string());
                }
                Subject::BlankNode(BlankNode { id }) => {
                    blank_issuer.get_or_issue(id.to_string());
                }
                _ => (),
            };

            if let Term::BlankNode(BlankNode { id }) = q.object {
                blank_issuer.get_or_issue(id.to_string());
            }
        });

        let mut triples = self
            .claim
            .content
            .iter()
            .map(|q| {
                let subject = match q.subject {
                    Subject::NamedNode(n) if n.iri == self.claim.id => {
                        Subject::BlankNode(claim_node)
                    }
                    Subject::NamedNode(n) if n.iri != self.claim.id => {
                        Subject::BlankNode(BlankNode {
                            id: named_issuer.get(n.iri).ok_or_else(|| {
                                ContractError::Unexpected(
                                    "Could not replace named node, canonical identifier not found"
                                        .to_string(),
                                )
                            })?,
                        })
                    }
                    Subject::BlankNode(BlankNode { id }) => Subject::BlankNode(BlankNode {
                        id: blank_issuer.get(id).ok_or_else(|| {
                            ContractError::Unexpected(
                                "Could not replace blank node, canonical identifier not found"
                                    .to_string(),
                            )
                        })?,
                    }),
                    _ => q.subject,
                };
                let object = match q.object {
                    Term::NamedNode(n) => match named_issuer.get(n.iri) {
                        Some(id) => Term::BlankNode(BlankNode { id }),
                        None => Term::NamedNode(n),
                    },
                    Term::BlankNode(BlankNode { id }) => Term::BlankNode(BlankNode {
                        id: blank_issuer.get(id).ok_or_else(|| {
                            ContractError::Unexpected(
                                "Could not replace blank node, canonical identifier not found"
                                    .to_string(),
                            )
                        })?,
                    }),
                    _ => q.object,
                };

                Ok(Triple {
                    subject,
                    predicate: q.predicate,
                    object,
                })
            })
            .collect::<Result<Vec<Triple<'a>>, ContractError>>()?;

        named_issuer
            .issued_iter()
            .for_each(|(original, (_, replacement))| {
                triples.push(Triple {
                    subject: Subject::BlankNode(BlankNode { id: replacement }),
                    predicate: VC_CLAIM_ORIGINAL_NODE,
                    object: Term::NamedNode(NamedNode { iri: original }),
                });
            });

        triples.push(Triple {
            subject: Subject::NamedNode(NamedNode { iri: self.id }),
            predicate: VC_CLAIM,
            object: Term::BlankNode(claim_node),
        });

        Ok(triples)
    }

    fn contains_reserved_predicates(&self) -> bool {
        self.claim
            .content
            .iter()
            .predicates()
            .any(|p| VC_RESERVED_PREDICATES.contains(&p))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::credential::vc::VerifiableCredential;
    use crate::testutil::testutil;
    use axone_rdf::dataset::Dataset;
    use cosmwasm_std::Addr;

    #[test]
    fn proper_serialization() {
        let owned_quads = testutil::read_test_quads("vc-valid.nq");
        let dataset = Dataset::from(owned_quads.as_slice());
        let vc = VerifiableCredential::try_from(&dataset).unwrap();
        let dc = DataverseCredential::try_from((
            Addr::unchecked("axone1072nc6egexqr2v6vpp7yxwm68plvqnkf5uemr0"),
            &vc,
        ))
        .unwrap();

        let expected = "<https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#submitterAddress> \"axone1072nc6egexqr2v6vpp7yxwm68plvqnkf5uemr0\" .
<https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#issuer> <did:key:zQ3shs7auhJSmVJpiUbQWco6bxxEhSqWnVEPvaBHBRvBKw6Q3> .
<https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#type> <https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/DigitalServiceDescriptionCredential> .
<https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#validFrom> \"2024-01-22T00:00:00\"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#subject> <did:key:zQ3shhb4SvzBRLbBonsvKb3WX6WoDeKWHpsXXXMhAJETqXAfB> .
_:c0 <https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/hasCategory> <https://w3id.org/axone/ontology/vnext/thesaurus/digital-service-category/Storage> .
_:c0 <https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/hasTag> \"Cloud\" .
<https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#claim> _:c0 .
<https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#validUntil> \"2025-01-22T00:00:00\"^^<http://www.w3.org/2001/XMLSchema#dateTime> .\n";

        let serialization_res = dc.serialize(DataFormat::NQuads);
        assert!(serialization_res.is_ok());

        assert_eq!(
            String::from_utf8(serialization_res.unwrap().to_vec()).unwrap(),
            expected
        );
    }

    #[test]
    fn proper_named_hierarchy_serialization() {
        let owned_quads = testutil::read_test_quads("vc-claim-hierarchy.nq");
        let dataset = Dataset::from(owned_quads.as_slice());
        let vc = VerifiableCredential::try_from(&dataset).unwrap();
        let dc = DataverseCredential::try_from((
            Addr::unchecked("axone1072nc6egexqr2v6vpp7yxwm68plvqnkf5uemr0"),
            &vc,
        ))
        .unwrap();

        let expected = "<https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#submitterAddress> \"axone1072nc6egexqr2v6vpp7yxwm68plvqnkf5uemr0\" .
<https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#issuer> <did:key:zQ3shs7auhJSmVJpiUbQWco6bxxEhSqWnVEPvaBHBRvBKw6Q3> .
<https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#type> <https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/DigitalServiceDescriptionCredential> .
<https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#validFrom> \"2024-01-22T00:00:00\"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#subject> <did:key:zQ3shhb4SvzBRLbBonsvKb3WX6WoDeKWHpsXXXMhAJETqXAfB> .
_:c0 <https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/hasCategory> <https://w3id.org/axone/ontology/vnext/thesaurus/digital-service-category/Storage> .
_:c0 <https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/hasTag> \"Cloud\" .
_:c0 <test:claim#named-hierarchy> _:a0 .
_:a0 <test:claim#nested-predicate> \"nested value\" .
_:a0 <dataverse:claim#original-node> <test:named-link> .
<https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#claim> _:c0 .
<https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#validUntil> \"2025-01-22T00:00:00\"^^<http://www.w3.org/2001/XMLSchema#dateTime> .\n";

        let serialization_res = dc.serialize(DataFormat::NQuads);
        assert!(serialization_res.is_ok());

        assert_eq!(
            String::from_utf8(serialization_res.unwrap().to_vec()).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_reserved_predicates() {
        let owned_quads = testutil::read_test_quads("vc-unsupported-4.nq");
        let dataset = Dataset::from(owned_quads.as_slice());
        let vc = VerifiableCredential::try_from(&dataset).unwrap();
        let dc = DataverseCredential::try_from((
            Addr::unchecked("axone1072nc6egexqr2v6vpp7yxwm68plvqnkf5uemr0"),
            &vc,
        ))
        .unwrap();

        let res = dc.serialize(DataFormat::NQuads);
        assert!(res.is_err());
        if let ContractError::UnsupportedCredential(msg) = res.err().unwrap() {
            assert_eq!(msg, "Claim contains reserved predicates.".to_string());
        } else {
            assert!(false);
        }
    }
}
