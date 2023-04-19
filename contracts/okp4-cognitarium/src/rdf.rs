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
    for delim in ['#', '/'] {
        if let Some(index) = iri.rfind(delim) {
            return Ok((&iri[..index], &iri[index..]));
        }
    }

    Err(StdError::generic_err("Couldn't extract IRI namespace"))
}
