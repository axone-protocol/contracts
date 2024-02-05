use rio_api::model::{BlankNode, GraphName, Literal, NamedNode, Quad, Subject, Term};

#[derive(Debug)]
pub struct RDFStarUnsupported;

pub struct OwnedQuad {
    subject: OwnedSubject,
    predicate: String,
    object: OwnedTerm,
    graph_name: Option<OwnedGraphName>,
}

impl TryFrom<Quad<'_>> for OwnedQuad {
    type Error = RDFStarUnsupported;

    fn try_from(value: Quad<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            subject: value.subject.try_into()?,
            predicate: value.predicate.iri.to_owned(),
            object: value.object.try_into()?,
            graph_name: value.graph_name.map(OwnedGraphName::from),
        })
    }
}

impl<'a> From<&'a OwnedQuad> for Quad<'a> {
    fn from(value: &'a OwnedQuad) -> Self {
        Self {
            subject: (&value.subject).into(),
            predicate: NamedNode {
                iri: value.predicate.as_str(),
            },
            object: (&value.object).into(),
            graph_name: value.graph_name.as_ref().map(GraphName::from),
        }
    }
}

pub enum Id {
    Named(String),
    Blank(String),
}

pub type OwnedSubject = Id;

impl TryFrom<Subject<'_>> for OwnedSubject {
    type Error = RDFStarUnsupported;

    fn try_from(value: Subject<'_>) -> Result<Self, Self::Error> {
        Ok(match value {
            Subject::NamedNode(n) => Self::Named(n.iri.to_owned()),
            Subject::BlankNode(n) => Self::Blank(n.id.to_owned()),
            Subject::Triple(_) => Err(RDFStarUnsupported {})?,
        })
    }
}

impl<'a> From<&'a OwnedSubject> for Subject<'a> {
    fn from(value: &'a OwnedSubject) -> Self {
        match value {
            OwnedSubject::Named(iri) => NamedNode { iri }.into(),
            OwnedSubject::Blank(id) => BlankNode { id }.into(),
        }
    }
}

pub type OwnedGraphName = Id;

impl From<GraphName<'_>> for OwnedGraphName {
    fn from(value: GraphName<'_>) -> Self {
        match value {
            GraphName::NamedNode(n) => Self::Named(n.iri.to_owned()),
            GraphName::BlankNode(n) => Self::Blank(n.id.to_owned()),
        }
    }
}

impl<'a> From<&'a OwnedGraphName> for GraphName<'a> {
    fn from(value: &'a OwnedGraphName) -> Self {
        match value {
            OwnedGraphName::Named(iri) => NamedNode { iri }.into(),
            OwnedGraphName::Blank(id) => BlankNode { id }.into(),
        }
    }
}

pub enum OwnedTerm {
    Named(String),
    Blank(String),
    Literal(OwnedLiteral),
}

impl TryFrom<Term<'_>> for OwnedTerm {
    type Error = RDFStarUnsupported;

    fn try_from(value: Term<'_>) -> Result<Self, Self::Error> {
        Ok(match value {
            Term::NamedNode(n) => OwnedTerm::Named(n.iri.to_owned()),
            Term::BlankNode(n) => OwnedTerm::Blank(n.id.to_owned()),
            Term::Literal(l) => OwnedTerm::Literal(l.into()),
            Term::Triple(_) => Err(RDFStarUnsupported)?,
        })
    }
}

impl<'a> From<&'a OwnedTerm> for Term<'a> {
    fn from(value: &'a OwnedTerm) -> Self {
        match value {
            OwnedTerm::Named(iri) => NamedNode { iri }.into(),
            OwnedTerm::Blank(id) => BlankNode { id }.into(),
            OwnedTerm::Literal(l) => Term::Literal(l.into()),
        }
    }
}

pub enum OwnedLiteral {
    Simple(String),
    LanguageTaggedString { value: String, language: String },
    Typed { value: String, datatype: String },
}

impl From<Literal<'_>> for OwnedLiteral {
    fn from(value: Literal<'_>) -> Self {
        match value {
            Literal::Simple { value } => OwnedLiteral::Simple(value.to_owned()),
            Literal::LanguageTaggedString { value, language } => {
                OwnedLiteral::LanguageTaggedString {
                    value: value.to_owned(),
                    language: language.to_owned(),
                }
            }
            Literal::Typed { value, datatype } => OwnedLiteral::Typed {
                value: value.to_owned(),
                datatype: datatype.iri.to_owned(),
            },
        }
    }
}

impl<'a> From<&'a OwnedLiteral> for Literal<'a> {
    fn from(l: &'a OwnedLiteral) -> Self {
        match l {
            OwnedLiteral::Simple(value) => Literal::Simple { value },
            OwnedLiteral::LanguageTaggedString { value, language } => {
                Literal::LanguageTaggedString { value, language }
            }
            OwnedLiteral::Typed { value, datatype } => Literal::Typed {
                value,
                datatype: NamedNode { iri: datatype },
            },
        }
    }
}
