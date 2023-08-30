use crate::msg::DataFormat;
use rio_api::formatter::TriplesFormatter;
use rio_api::model::{Quad, Triple};
use rio_api::parser::{QuadsParser, TriplesParser};
use rio_turtle::{
    NQuadsFormatter, NQuadsParser, NTriplesFormatter, NTriplesParser, TurtleError, TurtleFormatter,
    TurtleParser,
};
use rio_xml::{RdfXmlError, RdfXmlFormatter, RdfXmlParser};
use std::io::{self, BufRead};

pub struct TripleReader<R: BufRead> {
    parser: TriplesParserKind<R>,
}

pub struct TripleWriter<W: std::io::Write> {
    writer: TriplesWriterKind<W>,
}

#[allow(clippy::large_enum_variant)]
pub enum TriplesParserKind<R: BufRead> {
    NTriples(NTriplesParser<R>),
    Turtle(TurtleParser<R>),
    RdfXml(RdfXmlParser<R>),
    NQuads(NQuadsParser<R>),
}

pub enum TriplesWriterKind<W: std::io::Write> {
    NTriples(NTriplesFormatter<W>),
    Turtle(TurtleFormatter<W>),
    RdfXml(io::Result<RdfXmlFormatter<W>>),
    NQuads(NQuadsFormatter<W>),
}

impl<R: BufRead> TripleReader<R> {
    pub fn new(format: &DataFormat, src: R) -> Self {
        TripleReader {
            parser: match format {
                DataFormat::RDFXml => TriplesParserKind::RdfXml(RdfXmlParser::new(src, None)),
                DataFormat::Turtle => TriplesParserKind::Turtle(TurtleParser::new(src, None)),
                DataFormat::NTriples => TriplesParserKind::NTriples(NTriplesParser::new(src)),
                DataFormat::NQuads => TriplesParserKind::NQuads(NQuadsParser::new(src)),
            },
        }
    }

    pub fn read_all<E, UF>(&mut self, mut use_fn: UF) -> Result<(), E>
    where
        UF: FnMut(Triple<'_>) -> Result<(), E>,
        E: From<TurtleError> + From<RdfXmlError>,
    {
        match &mut self.parser {
            TriplesParserKind::NTriples(parser) => parser.parse_all(&mut use_fn),
            TriplesParserKind::Turtle(parser) => parser.parse_all(&mut use_fn),
            TriplesParserKind::RdfXml(parser) => parser.parse_all(&mut use_fn),
            TriplesParserKind::NQuads(parser) => {
                parser.parse_all(&mut |quad: Quad<'_>| -> Result<(), E> {
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

impl<W: io::Write> TripleWriter<W> {
    pub fn new(format: &DataFormat, dst: W) -> Self {
        TripleWriter {
            writer: match format {
                DataFormat::RDFXml => TriplesWriterKind::RdfXml(RdfXmlFormatter::new(dst)),
                DataFormat::Turtle => TriplesWriterKind::Turtle(TurtleFormatter::new(dst)),
                DataFormat::NTriples => TriplesWriterKind::NTriples(NTriplesFormatter::new(dst)),
                DataFormat::NQuads => TriplesWriterKind::NQuads(NQuadsFormatter::new(dst)),
            },
        }
    }

    pub fn write(&mut self, triple: &Triple<'_>) -> io::Result<()> {
        match &mut self.writer {
            TriplesWriterKind::Turtle(formatter) => formatter.format(triple),
            TriplesWriterKind::NTriples(formatter) => formatter.format(triple),
            TriplesWriterKind::NQuads(formatter) => {
                use rio_api::formatter::QuadsFormatter;

                let quad = &Quad {
                    subject: triple.subject,
                    predicate: triple.predicate,
                    object: triple.object,
                    graph_name: None,
                };

                formatter.format(quad)
            }
            TriplesWriterKind::RdfXml(format_result) => match format_result {
                Ok(formatter) => formatter.format(triple),
                Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
            },
        }
    }

    #[allow(dead_code)]
    pub fn write_all(&mut self, triples: Vec<&Triple<'_>>) -> io::Result<()> {
        for triple in triples {
            self.write(triple)?;
        }
        Ok(())
    }

    pub fn finish(self) -> io::Result<W> {
        match self.writer {
            TriplesWriterKind::Turtle(formatter) => formatter.finish(),
            TriplesWriterKind::NTriples(formatter) => formatter.finish(),
            TriplesWriterKind::NQuads(formatter) => formatter.finish(),
            TriplesWriterKind::RdfXml(format_result) => match format_result {
                Ok(formatter) => formatter.finish(),
                Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
            },
        }
    }
}
