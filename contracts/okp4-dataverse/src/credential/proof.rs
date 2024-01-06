use crate::credential::error::InvalidCredentialError;
use crate::credential::rdf_marker::RDF_TYPE;
use itertools::Itertools;
use okp4_rdf::dataset::{Dataset, QuadIterator};
use rio_api::model::Term;

#[allow(dead_code)]
pub struct Proof<'a> {
    type_: String,
    inner: Dataset<'a>,
}

impl<'a> TryFrom<Dataset<'a>> for Proof<'a> {
    type Error = InvalidCredentialError;

    fn try_from(dataset: Dataset<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            type_: dataset
                .match_pattern(None, Some(RDF_TYPE), None, None)
                .objects()
                .exactly_one()
                .map_err(|e| match e.size_hint() {
                    (_, Some(_)) => InvalidCredentialError::MissingProofType,
                    _ => InvalidCredentialError::Malformed(
                        "Proof cannot have more than one type".to_string(),
                    ),
                })
                .and_then(|o| match o {
                    Term::NamedNode(n) => Ok(n.iri.to_string()),
                    _ => Err(InvalidCredentialError::Malformed(
                        "Proof type must be a named node".to_string(),
                    )),
                })?,
            inner: dataset,
        })
    }
}
