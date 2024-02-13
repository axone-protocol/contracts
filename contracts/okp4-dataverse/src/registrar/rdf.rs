use crate::credential::rdf_marker::RDF_DATE_TYPE;
use crate::registrar::credential::DataverseCredential;
use crate::ContractError;
use cosmwasm_std::{Binary, StdError};
use okp4_rdf::serde::{DataFormat, TripleWriter};
use rio_api::model::{BlankNode, Literal, NamedNode, Subject, Term, Triple};

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

impl<'a> TryFrom<&'a DataverseCredential<'a>> for Vec<Triple<'a>> {
    type Error = ContractError;

    fn try_from(credential: &'a DataverseCredential<'a>) -> Result<Self, Self::Error> {
        let c_subject = Subject::NamedNode(NamedNode { iri: credential.id });
        //todo: use the canon identifier issuer instead and rename all blank nodes
        let claim_node = BlankNode { id: "c0" };

        let mut triples = vec![
            Triple {
                subject: c_subject,
                predicate: VC_SUBMITTER_ADDRESS,
                object: Term::NamedNode(NamedNode {
                    iri: credential.submitter_addr.as_str(),
                }),
            },
            Triple {
                subject: c_subject,
                predicate: VC_ISSUER,
                object: Term::NamedNode(NamedNode {
                    iri: credential.issuer,
                }),
            },
            Triple {
                subject: c_subject,
                predicate: VC_TYPE,
                object: Term::NamedNode(NamedNode {
                    iri: credential.r#type,
                }),
            },
            Triple {
                subject: c_subject,
                predicate: VC_VALID_FROM,
                object: Term::Literal(Literal::Typed {
                    value: credential.valid_from,
                    datatype: RDF_DATE_TYPE,
                }),
            },
            Triple {
                subject: c_subject,
                predicate: VC_SUBJECT,
                object: Term::NamedNode(NamedNode {
                    iri: credential.subject,
                }),
            },
            Triple {
                subject: c_subject,
                predicate: VC_CLAIM,
                object: Term::BlankNode(claim_node),
            },
        ];

        triples.extend(credential.claim.iter().map(|q| {
            let subject = match q.subject {
                Subject::NamedNode(n) if n.iri == credential.subject => {
                    Subject::BlankNode(claim_node)
                }
                _ => q.subject,
            };
            Triple {
                subject,
                predicate: q.predicate,
                object: q.object,
            }
        }));

        if let Some(valid_until) = credential.valid_until {
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
}

pub fn serialize(
    credential: &DataverseCredential<'_>,
    format: DataFormat,
) -> Result<Binary, ContractError> {
    let triples: Vec<Triple<'_>> = credential.try_into()?;
    let out: Vec<u8> = Vec::default();
    let mut writer = TripleWriter::new(&format, out);
    for triple in triples {
        writer
            .write(&triple)
            .map_err(|e| StdError::serialize_err("triple", format!("Error writing triple: {e}")))?;
    }

    Ok(Binary::from(writer.finish().map_err(|e| {
        StdError::serialize_err("triple", format!("Error writing triple: {e}"))
    })?))
}
