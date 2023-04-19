use crate::msg::DataInput;
use cosmwasm_std::StdError;
use rio_api::model::Triple;
use rio_api::parser::TriplesParser;
use rio_turtle::{NTriplesParser, TurtleError, TurtleParser};
use rio_xml::{RdfXmlError, RdfXmlParser};
use std::io::BufReader;

pub fn parse_triples<E, T, MF, UF>(input: DataInput, map_fn: MF, use_fn: UF) -> Result<(), E>
where
    MF: FnMut(Triple<'_>) -> Result<T, E>,
    UF: FnMut(Result<T, E>) -> Result<(), E>,
    E: From<RdfXmlError> + From<TurtleError>,
{
    match input {
        DataInput::RDFXml(data) => RdfXmlParser::new(BufReader::new(data.as_slice()), None)
            .into_iter(map_fn)
            .try_for_each(use_fn),
        DataInput::Turtle(data) => TurtleParser::new(BufReader::new(data.as_slice()), None)
            .into_iter(map_fn)
            .try_for_each(use_fn),
        DataInput::NTriples(data) => NTriplesParser::new(BufReader::new(data.as_slice()))
            .into_iter(map_fn)
            .try_for_each(use_fn),
    }
}

pub fn explode_iri(iri: &str) -> Result<(&str, &str), StdError> {
    let mut marker_index: Option<usize> = None;
    for delim in ['#', '/', ':'] {
        if let Some(index) = iri.rfind(delim) {
            marker_index = match marker_index {
                Some(i) => Some(i.max(index)),
                None => Some(index),
            }
        }
    }

    if let Some(index) = marker_index {
        return Ok((&iri[..index + 1], &iri[index + 1..]));
    }

    Err(StdError::generic_err("Couldn't extract IRI namespace"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proper_explode_iri() {
        assert_eq!(
            explode_iri("http://www.w3.org/2001/XMLSchema#dateTime"),
            Ok(("http://www.w3.org/2001/XMLSchema#", "dateTime"))
        );
        assert_eq!(
            explode_iri("https://ontology.okp4.space/core/Governance"),
            Ok(("https://ontology.okp4.space/core/", "Governance"))
        );
        assert_eq!(
            explode_iri(
                "did:key:0x04d1f1b8f8a7a28f9a5a254c326a963a22f5a5b5d5f5e5d5c5b5a5958575655"
            ),
            Ok((
                "did:key:",
                "0x04d1f1b8f8a7a28f9a5a254c326a963a22f5a5b5d5f5e5d5c5b5a5958575655"
            ))
        );
        assert_eq!(
            explode_iri("wow:this/is#weird"),
            Ok(("wow:this/is#", "weird"))
        );
        assert_eq!(
            explode_iri("this#is:weird/too"),
            Ok(("this#is:weird/", "too"))
        );
        assert_eq!(
            explode_iri("this_doesn't_work"),
            Err(StdError::generic_err("Couldn't extract IRI namespace"))
        );
    }
}
