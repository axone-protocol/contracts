use crate::error::RDFParseError;
use crate::msg::DataInput;
use crate::state;
use cosmwasm_std::StdError;
use rio_api::model::{Literal, NamedNode, Subject, Term, Triple};
use rio_api::parser::TriplesParser;
use rio_turtle::{NTriplesParser, TurtleParser};
use rio_xml::RdfXmlParser;
use std::io::{BufRead, BufReader};

pub struct TripleReader<R: BufRead> {
    parser: TriplesParserKind<R>,
    buffer: Vec<state::Triple>,
}

pub enum TriplesParserKind<R: BufRead> {
    NTriples(NTriplesParser<R>),
    Turtle(TurtleParser<R>),
    RdfXml(RdfXmlParser<R>),
}

pub fn read_triples(graph: &DataInput) -> TripleReader<BufReader<&[u8]>> {
    TripleReader::new(match graph {
        DataInput::RDFXml(data) => {
            TriplesParserKind::RdfXml(RdfXmlParser::new(BufReader::new(data.as_slice()), None))
        }
        DataInput::Turtle(data) => {
            TriplesParserKind::Turtle(TurtleParser::new(BufReader::new(data.as_slice()), None))
        }
        DataInput::NTriples(data) => {
            TriplesParserKind::NTriples(NTriplesParser::new(BufReader::new(data.as_slice())))
        }
    })
}

pub type NSResolveFn<'a> = Box<dyn FnMut(String) -> Result<u128, StdError> + 'a>;

impl<R: BufRead> TripleReader<R> {
    pub fn new(parser: TriplesParserKind<R>) -> Self {
        TripleReader {
            parser,
            buffer: Vec::new(),
        }
    }

    pub fn next(
        &mut self,
        ns_resolve_fn: &mut NSResolveFn,
    ) -> Option<Result<state::Triple, RDFParseError>> {
        loop {
            if let Some(t) = self.buffer.pop() {
                return Some(Ok(t));
            }

            if let Err(e) = match &mut self.parser {
                TriplesParserKind::NTriples(parser) => {
                    Self::read(parser, &mut self.buffer, ns_resolve_fn)
                }
                TriplesParserKind::Turtle(parser) => {
                    Self::read(parser, &mut self.buffer, ns_resolve_fn)
                }
                TriplesParserKind::RdfXml(parser) => {
                    Self::read(parser, &mut self.buffer, ns_resolve_fn)
                }
            }? {
                return Some(Err(e));
            }
        }
    }

    fn read<P, E>(
        parser: &mut P,
        buffer: &mut Vec<state::Triple>,
        ns_resolve_fn: &mut NSResolveFn,
    ) -> Option<Result<(), E>>
    where
        P: TriplesParser,
        E: From<P::Error> + From<RDFParseError>,
    {
        if parser.is_end() {
            None?
        }

        if let Err(e) = parser.parse_step(&mut |t| {
            buffer.push(Self::triple(&t, ns_resolve_fn)?);
            Ok(())
        }) {
            Some(Err(e))
        } else {
            Some(Ok(()))
        }
    }

    fn triple(
        triple: &Triple,
        ns_resolve_fn: &mut NSResolveFn,
    ) -> Result<state::Triple, RDFParseError> {
        Ok(state::Triple {
            subject: Self::subject(triple.subject, ns_resolve_fn)?,
            predicate: Self::node(triple.predicate, ns_resolve_fn)?,
            object: Self::object(triple.object, ns_resolve_fn)?,
        })
    }

    fn subject(
        subject: Subject,
        ns_resolve_fn: &mut NSResolveFn,
    ) -> Result<state::Subject, RDFParseError> {
        match subject {
            Subject::NamedNode(node) => {
                Self::node(node, ns_resolve_fn).map(|n| state::Subject::Named(n))
            }
            Subject::BlankNode(node) => Ok(state::Subject::Blank(node.id.to_string())),
            _ => Err(RDFParseError::Unexpected(
                "RDF star syntax unsupported".to_string(),
            )),
        }
    }

    fn node(
        node: NamedNode,
        ns_resolve_fn: &mut NSResolveFn,
    ) -> Result<state::Node, RDFParseError> {
        let (ns, v) = explode_iri(node.iri)?;
        Ok(state::Node {
            namespace: ns_resolve_fn(ns)?,
            value: v,
        })
    }

    fn object(
        object: Term,
        ns_resolve_fn: &mut NSResolveFn,
    ) -> Result<state::Object, RDFParseError> {
        match object {
            Term::BlankNode(node) => Ok(state::Object::Blank(node.id.to_string())),
            Term::NamedNode(node) => {
                Self::node(node, ns_resolve_fn).map(|n| state::Object::Named(n))
            }
            Term::Literal(literal) => {
                Self::literal(literal, ns_resolve_fn).map(|l| state::Object::Literal(l))
            }
            _ => Err(RDFParseError::Unexpected(
                "RDF star syntax unsupported".to_string(),
            )),
        }
    }

    fn literal(
        literal: Literal,
        ns_resolve_fn: &mut NSResolveFn,
    ) -> Result<state::Literal, RDFParseError> {
        match literal {
            Literal::Simple { value } => Ok(state::Literal::Simple {
                value: value.to_string(),
            }),
            Literal::LanguageTaggedString { value, language } => Ok(state::Literal::I18NString {
                value: value.to_string(),
                language: language.to_string(),
            }),
            Literal::Typed { value, datatype } => {
                Self::node(datatype, ns_resolve_fn).map(|node| state::Literal::Typed {
                    value: value.to_string(),
                    datatype: node,
                })
            }
        }
    }
}

pub fn explode_iri(iri: &str) -> Result<(String, String), RDFParseError> {
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

    Err(RDFParseError::Unexpected(
        "Couldn't extract IRI namespace".to_string(),
    ))
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
            Err(RDFParseError::Unexpected(
                "Couldn't extract IRI namespace".to_string()
            ))
        );
    }
}
