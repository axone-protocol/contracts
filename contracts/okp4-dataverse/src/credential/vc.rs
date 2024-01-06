use crate::credential::error::InvalidCredentialError;
use crate::credential::proof::Proof;
use crate::credential::rdf_markers::*;
use itertools::Itertools;
use okp4_rdf::dataset::Dataset;
use okp4_rdf::dataset::QuadIterator;
use rio_api::model::{BlankNode, Literal, NamedNode, Subject, Term};

#[allow(dead_code)]
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
pub struct Claim<'a> {
    id: &'a str,
    content: Dataset<'a>,
}

#[allow(dead_code)]
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
            .map(|o| {
                match o {
                    Term::BlankNode(n) => Ok(n),
                    _ => Err(InvalidCredentialError::Malformed(
                        "Credential proof must be encapsulated in blank node graph names"
                            .to_string(),
                    )),
                }
                .and_then(|g| {
                    Proof::try_from(Dataset::new(
                        dataset
                            .match_pattern(None, None, None, Some(Some(g.into())))
                            .map(|quad| *quad)
                            .collect(),
                    ))
                    .map(|p| (p, g))
                })
            })
            .collect()
    }
}
