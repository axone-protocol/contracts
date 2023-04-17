use cosmwasm_std::{StdError, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Subject {
    Named(Node),
    Blank(BlankNode),
}

pub type Predicate = Node;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Object {
    Named(Node),
    Blank(BlankNode),
    Literal(Literal),
}

pub type BlankNode = String;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub namespace: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    Simple { value: String },
    I18NString { value: String, language: String },
    Typed { value: String, datatype: Node },
}
