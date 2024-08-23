use crate::state::NamespaceSolver;
use blake3::Hash;
use cosmwasm_std::StdResult;
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use serde::{Deserialize, Serialize};

/// Represents a triple primary key as a tuple of:
/// - Object hash
/// - Predicate in a binary format
/// - Subject in a binary format
pub type TriplePK<'a> = (&'a [u8], Vec<u8>, Vec<u8>);

pub struct TripleIndexes<'a> {
    pub subject_and_predicate: MultiIndex<'a, (Vec<u8>, Vec<u8>), Triple, TriplePK<'a>>,
}

impl IndexList<Triple> for TripleIndexes<'_> {
    fn get_indexes(&self) -> Box<dyn Iterator<Item = &'_ dyn Index<Triple>> + '_> {
        let subject_and_predicate: &dyn Index<Triple> = &self.subject_and_predicate;
        Box::new(vec![subject_and_predicate].into_iter())
    }
}

pub fn triples<'a>() -> IndexedMap<TriplePK<'a>, Triple, TripleIndexes<'a>> {
    IndexedMap::new(
        "TRIPLE",
        TripleIndexes {
            subject_and_predicate: MultiIndex::new(
                |_pk, triple| (triple.subject.key(), triple.predicate.key()),
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

impl Triple {
    pub fn namespaces(&self) -> Vec<u128> {
        let mut namespaces = Vec::with_capacity(3);
        if let Subject::Named(n) = &self.subject {
            namespaces.push(n.namespace);
        }

        namespaces.push(self.predicate.namespace);

        match &self.object {
            Object::Named(n) => namespaces.push(n.namespace),
            Object::Literal(Literal::Typed { datatype, .. }) => namespaces.push(datatype.namespace),
            _ => {}
        }

        namespaces
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Subject {
    Named(Node),
    Blank(BlankNode),
}

impl Subject {
    pub fn key(&self) -> Vec<u8> {
        match self {
            Subject::Named(n) => {
                let node = n.key();
                let mut key: Vec<u8> = Vec::with_capacity(node.len() + 1);
                key.push(b'n');
                key.extend(node);

                key
            }
            Subject::Blank(n) => {
                let val = n.to_be_bytes();
                let mut key: Vec<u8> = Vec::with_capacity(val.len() + 1);
                key.push(b'b');
                key.extend(val);

                key
            }
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

impl Object {
    pub fn as_hash(&self) -> Hash {
        let mut hasher = blake3::Hasher::new();
        match self {
            Object::Named(n) => {
                hasher
                    .update(&[b'n'])
                    .update(n.namespace.to_be_bytes().as_slice())
                    .update(n.value.as_bytes());
            }
            Object::Blank(n) => {
                hasher.update(&[b'b']).update(n.to_be_bytes().as_slice());
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

pub const BLANK_NODE_SIZE: usize = 16usize;
pub type BlankNode = u128;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub namespace: u128,
    pub value: String,
}

impl Node {
    pub fn key(&self) -> Vec<u8> {
        let val = self.value.as_bytes();
        let mut key: Vec<u8> = Vec::with_capacity(val.len() + 16);
        key.extend(self.namespace.to_be_bytes());
        key.extend(val);

        key
    }

    pub fn as_iri(&self, ns_solver: &mut dyn NamespaceSolver) -> StdResult<String> {
        Ok(ns_solver.resolve_from_key(self.namespace)?.value + &self.value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    Simple { value: String },
    I18NString { value: String, language: String },
    Typed { value: String, datatype: Node },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn object_hash() {
        let cases = vec![
            (
                Object::Named(Node {
                    namespace: 0,
                    value: "val1".to_string(),
                }),
                Object::Named(Node {
                    namespace: 0,
                    value: "val2".to_string(),
                }),
            ),
            (
                Object::Named(Node {
                    namespace: 1,
                    value: "val".to_string(),
                }),
                Object::Named(Node {
                    namespace: 2,
                    value: "val".to_string(),
                }),
            ),
            (Object::Blank(0u128), Object::Blank(1u128)),
            (
                Object::Literal(Literal::Simple {
                    value: "val1".to_string(),
                }),
                Object::Literal(Literal::Simple {
                    value: "val2".to_string(),
                }),
            ),
            (
                Object::Literal(Literal::I18NString {
                    language: "fr".to_string(),
                    value: "val1".to_string(),
                }),
                Object::Literal(Literal::I18NString {
                    language: "fr".to_string(),
                    value: "val2".to_string(),
                }),
            ),
            (
                Object::Literal(Literal::I18NString {
                    language: "fr".to_string(),
                    value: "val".to_string(),
                }),
                Object::Literal(Literal::I18NString {
                    language: "en".to_string(),
                    value: "val".to_string(),
                }),
            ),
            (
                Object::Literal(Literal::Typed {
                    datatype: Node {
                        namespace: 0,
                        value: "n".to_string(),
                    },
                    value: "val1".to_string(),
                }),
                Object::Literal(Literal::Typed {
                    datatype: Node {
                        namespace: 0,
                        value: "n".to_string(),
                    },
                    value: "val2".to_string(),
                }),
            ),
            (
                Object::Literal(Literal::Typed {
                    datatype: Node {
                        namespace: 0,
                        value: "n1".to_string(),
                    },
                    value: "val".to_string(),
                }),
                Object::Literal(Literal::Typed {
                    datatype: Node {
                        namespace: 0,
                        value: "n2".to_string(),
                    },
                    value: "val".to_string(),
                }),
            ),
            (
                Object::Literal(Literal::Typed {
                    datatype: Node {
                        namespace: 1,
                        value: "n".to_string(),
                    },
                    value: "val".to_string(),
                }),
                Object::Literal(Literal::Typed {
                    datatype: Node {
                        namespace: 2,
                        value: "n".to_string(),
                    },
                    value: "val".to_string(),
                }),
            ),
            (
                Object::Blank(0u128),
                Object::Literal(Literal::Simple {
                    value: "val".to_string(),
                }),
            ),
        ];

        for case in cases {
            assert_ne!(case.0.as_hash(), case.1.as_hash())
        }
    }
}
