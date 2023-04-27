use blake3::Hash;
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use serde::{Deserialize, Serialize};

pub struct TripleIndexes<'a> {
    subject_and_predicate:
        MultiIndex<'a, (Subject, Predicate), Triple, (&'a [u8], Predicate, Subject)>,
}

impl IndexList<Triple> for TripleIndexes<'_> {
    fn get_indexes(&self) -> Box<dyn Iterator<Item = &'_ dyn Index<Triple>> + '_> {
        Box::new(vec![&self.subject_and_predicate as &dyn Index<Triple>].into_iter())
    }
}

pub fn triples<'a>() -> IndexedMap<'a, (&'a [u8], Predicate, Subject), Triple, TripleIndexes<'a>> {
    IndexedMap::new(
        "TRIPLE",
        TripleIndexes {
            subject_and_predicate: MultiIndex::new(
                |_pk, triple| (triple.subject.clone(), triple.predicate.clone()),
                "TRIPLE",
                "TRIPLE__SUBJECT_PREDICATE",
            ),
        },
    )
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Triple {
    pub subject: Subject,
    pub predicate: Predicate,
    pub object: Object,
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

impl Object {
    pub fn as_hash(&self) -> Hash {
        let mut hasher = blake3::Hasher::new();
        match self {
            Object::Named(n) => {
                hasher
                    .update(&[b'n'])
                    .update(n.namespace.to_be_bytes().as_slice())
                    .update(n.namespace.to_be_bytes().as_slice());
            }
            Object::Blank(n) => {
                hasher.update(&[b'b']).update(n.as_bytes());
            }
            Object::Literal(l) => {
                hasher.update(&[b'l']);
                match l {
                    Literal::Simple { value } => hasher.update(&[b's']).update(value.as_bytes()),
                    Literal::I18NString { value, language } => hasher
                        .update(&[b'i'])
                        .update(value.as_bytes())
                        .update(language.as_bytes()),
                    Literal::Typed { value, datatype } => hasher
                        .update(&[b't'])
                        .update(value.as_bytes())
                        .update(datatype.namespace.to_be_bytes().as_slice())
                        .update(datatype.value.as_bytes()),
                };
            }
        }

        hasher.finalize()
    }
}

pub type BlankNode = String;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub namespace: u128,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    Simple { value: String },
    I18NString { value: String, language: String },
    Typed { value: String, datatype: Node },
}
