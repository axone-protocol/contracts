use crate::msg::DataFormat;
use cosmwasm_std::{StdError, StdResult};
use rio_api::model::{Quad, Triple};
use rio_api::parser::{QuadsParser, TriplesParser};
use rio_turtle::{NQuadsParser, NTriplesParser, TurtleError, TurtleParser};
use rio_xml::{RdfXmlError, RdfXmlParser};
use std::io::BufRead;

pub struct TripleReader<R: BufRead> {
    parser: TriplesParserKind<R>,
}

#[allow(clippy::large_enum_variant)]
pub enum TriplesParserKind<R: BufRead> {
    NTriples(NTriplesParser<R>),
    Turtle(TurtleParser<R>),
    RdfXml(RdfXmlParser<R>),
    NQuads(NQuadsParser<R>),
}

pub fn read_triples<R: BufRead>(format: DataFormat, src: R) -> TripleReader<R> {
    TripleReader::new(match format {
        DataFormat::RDFXml => TriplesParserKind::RdfXml(RdfXmlParser::new(src, None)),
        DataFormat::Turtle => TriplesParserKind::Turtle(TurtleParser::new(src, None)),
        DataFormat::NTriples => TriplesParserKind::NTriples(NTriplesParser::new(src)),
        DataFormat::NQuads => TriplesParserKind::NQuads(NQuadsParser::new(src)),
    })
}

impl<R: BufRead> TripleReader<R> {
    pub fn new(parser: TriplesParserKind<R>) -> Self {
        TripleReader { parser }
    }

    pub fn read_all<E, UF>(&mut self, mut use_fn: UF) -> Result<(), E>
    where
        UF: FnMut(Triple) -> Result<(), E>,
        E: From<TurtleError> + From<RdfXmlError>,
    {
        match &mut self.parser {
            TriplesParserKind::NTriples(parser) => parser.parse_all(&mut use_fn),
            TriplesParserKind::Turtle(parser) => parser.parse_all(&mut use_fn),
            TriplesParserKind::RdfXml(parser) => parser.parse_all(&mut use_fn),
            TriplesParserKind::NQuads(parser) => {
                parser.parse_all(&mut |quad: Quad| -> Result<(), E> {
                    use_fn(Triple {
                        subject: quad.subject,
                        predicate: quad.predicate,
                        object: quad.object,
                    })
                })
            }
        }
    }
}

pub fn explode_iri(iri: &str) -> StdResult<(String, String)> {
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
        return Ok((iri[..index + 1].to_string(), iri[index + 1..].to_string()));
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
            Ok((
                "http://www.w3.org/2001/XMLSchema#".to_string(),
                "dateTime".to_string()
            ))
        );
        assert_eq!(
            explode_iri("https://ontology.okp4.space/core/Governance"),
            Ok((
                "https://ontology.okp4.space/core/".to_string(),
                "Governance".to_string()
            ))
        );
        assert_eq!(
            explode_iri(
                "did:key:0x04d1f1b8f8a7a28f9a5a254c326a963a22f5a5b5d5f5e5d5c5b5a5958575655"
            ),
            Ok((
                "did:key:".to_string(),
                "0x04d1f1b8f8a7a28f9a5a254c326a963a22f5a5b5d5f5e5d5c5b5a5958575655".to_string()
            ))
        );
        assert_eq!(
            explode_iri("wow:this/is#weird"),
            Ok(("wow:this/is#".to_string(), "weird".to_string()))
        );
        assert_eq!(
            explode_iri("this#is:weird/too"),
            Ok(("this#is:weird/".to_string(), "too".to_string()))
        );
        assert_eq!(
            explode_iri("this_doesn't_work"),
            Err(StdError::generic_err("Couldn't extract IRI namespace"))
        );
    }
}
