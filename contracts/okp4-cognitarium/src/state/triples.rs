use crate::rdf::explode_iri;
use cosmwasm_std::{StdError, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use rio_api::model::NamedNode;
use serde::{Deserialize, Serialize};

pub struct TripleIndexes<'a> {
    subject_and_predicate: MultiIndex<'a, (Subject, Predicate), Triple, Uint128>,
    predicate_and_object: MultiIndex<'a, (Predicate, Object), Triple, Uint128>,
}

impl IndexList<Triple> for TripleIndexes<'_> {
    fn get_indexes(&self) -> Box<dyn Iterator<Item = &'_ dyn Index<Triple>> + '_> {
        Box::new(
            vec![
                &self.subject_and_predicate as &dyn Index<Triple>,
                &self.predicate_and_object,
            ]
            .into_iter(),
        )
    }
}

pub fn triples<'a>() -> IndexedMap<'a, u128, Triple, TripleIndexes<'a>> {
    IndexedMap::new(
        "TRIPLE",
        TripleIndexes {
            subject_and_predicate: MultiIndex::new(
                |_pk, triple| (triple.subject.clone(), triple.predicate.clone()),
                "TRIPLE",
                "TRIPLE__SUBJECT_PREDICATE",
            ),
            predicate_and_object: MultiIndex::new(
                |_pk, triple| (triple.predicate.clone(), triple.object.clone()),
                "TRIPLE",
                "TRIPLE__PREDICATE_OBJECT",
            ),
        },
    )
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Triple {
    subject: Subject,
    predicate: Predicate,
    object: Object,
}

impl<'a> TryFrom<rio_api::model::Triple<'a>> for Triple {
    type Error = StdError;

    fn try_from(value: rio_api::model::Triple<'a>) -> Result<Self, Self::Error> {
        Ok(Triple {
            subject: value.subject.try_into()?,
            predicate: value.predicate.try_into()?,
            object: value.object.try_into()?,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Subject {
    Named(Node),
    Blank(BlankNode),
}

impl<'a> TryFrom<rio_api::model::Subject<'a>> for Subject {
    type Error = StdError;

    fn try_from(value: rio_api::model::Subject<'a>) -> Result<Self, Self::Error> {
        match value {
            rio_api::model::Subject::NamedNode(node) => {
                Node::try_from(node).map(|n| Subject::Named(n))
            }
            rio_api::model::Subject::BlankNode(node) => Ok(Subject::Blank(node.id.to_string())),
            _ => Err(StdError::generic_err("Not implemented")),
        }
    }
}

pub type Predicate = Node;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Object {
    Named(Node),
    Blank(BlankNode),
    Literal(Literal),
}

impl<'a> TryFrom<rio_api::model::Term<'a>> for Object {
    type Error = StdError;

    fn try_from(value: rio_api::model::Term<'a>) -> Result<Self, Self::Error> {
        match value {
            rio_api::model::Term::BlankNode(node) => Ok(Object::Blank(node.id.to_string())),
            rio_api::model::Term::NamedNode(node) => Node::try_from(node).map(|n| Object::Named(n)),
            rio_api::model::Term::Literal(literal) => {
                Literal::try_from(literal).map(|l| Object::Literal(l))
            }
            _ => Err(StdError::generic_err("Not implemented")),
        }
    }
}

pub type BlankNode = String;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub namespace: String,
    pub value: String,
}

impl<'a> TryFrom<NamedNode<'a>> for Node {
    type Error = StdError;

    fn try_from(value: NamedNode) -> Result<Self, Self::Error> {
        explode_iri(value.iri).map(|(ns, v)| Self {
            namespace: ns.to_string(),
            value: v.to_string(),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    Simple { value: String },
    I18NString { value: String, language: String },
    Typed { value: String, datatype: Node },
}

impl<'a> TryFrom<rio_api::model::Literal<'a>> for Literal {
    type Error = StdError;

    fn try_from(value: rio_api::model::Literal<'a>) -> Result<Self, Self::Error> {
        match value {
            rio_api::model::Literal::Simple { value } => Ok(Literal::Simple {
                value: value.to_string(),
            }),
            rio_api::model::Literal::LanguageTaggedString { value, language } => {
                Ok(Literal::I18NString {
                    value: value.to_string(),
                    language: language.to_string(),
                })
            }
            rio_api::model::Literal::Typed { value, datatype } => {
                Node::try_from(datatype).map(|node| Literal::Typed {
                    value: value.to_string(),
                    datatype: node,
                })
            }
        }
    }
}
