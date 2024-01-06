use crate::did::consts::RDF_TYPE;
use crate::ContractError;
use itertools::Itertools;
use okp4_rdf::dataset::{Dataset, QuadIterator};
use rio_api::model::Term;

pub struct Proof<'a> {
    type_: String,
    inner: Dataset<'a>,
}

impl<'a> TryFrom<Dataset<'a>> for Proof<'a> {
    type Error = ContractError;

    fn try_from(dataset: Dataset<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            type_: dataset
                .match_pattern(None, Some(RDF_TYPE), None, None)
                .objects()
                .exactly_one()
                .map_err(|_| {
                    ContractError::InvalidCredential(
                        "Credential proof can must have only one type".to_string(),
                    )
                })
                .and_then(|o| match o {
                    Term::NamedNode(n) => Ok(n.iri.to_string()),
                    _ => Err(ContractError::InvalidCredential(
                        "Credential proof type must be a named node".to_string(),
                    )),
                })?,
            inner: dataset,
        })
    }
}
