use crate::error::StoreError;
use crate::rdf::TripleReader;
use crate::state::{
    namespaces, triples, Literal, Namespace, Node, Object, Store, Subject, Triple,
    NAMESPACE_KEY_INCREMENT, STORE,
};
use crate::{rdf, ContractError};
use blake3::Hash;
use cosmwasm_std::{StdError, StdResult, Storage, Uint128};
use rio_api::model;
use rio_api::model::Term;
use std::collections::BTreeMap;
use std::io::BufRead;

pub struct StoreEngine<'a> {
    storage: &'a mut dyn Storage,
    store: Store,
    ns_key_inc_offset: u128,
    ns_cache: BTreeMap<String, Namespace>,
    initial_triple_count: Uint128,
    initial_byte_size: Uint128,
}

impl<'a> StoreEngine<'a> {
    pub fn new(storage: &'a mut dyn Storage) -> StdResult<Self> {
        let store = STORE.load(storage)?;
        let ns_key_inc_offset = NAMESPACE_KEY_INCREMENT.load(storage)?;
        Ok(Self {
            storage,
            store: store.clone(),
            ns_key_inc_offset,
            ns_cache: BTreeMap::new(),
            initial_triple_count: store.stat.triple_count,
            initial_byte_size: store.stat.byte_size,
        })
    }

    pub fn store_all<R: BufRead>(
        &mut self,
        reader: &mut TripleReader<R>,
    ) -> Result<Uint128, ContractError> {
        reader.read_all(|t| self.store_triple(t))?;
        self.finish()
    }

    fn store_triple(&mut self, t: model::Triple<'_>) -> Result<(), ContractError> {
        self.store.stat.triple_count += Uint128::one();
        if self.store.stat.triple_count > self.store.limits.max_triple_count {
            Err(StoreError::TripleCount(self.store.limits.max_triple_count))?;
        }
        if self.store.stat.triple_count - self.initial_triple_count
            > self.store.limits.max_insert_data_triple_count
        {
            Err(StoreError::InsertDataTripleCount(
                self.store.limits.max_insert_data_triple_count,
            ))?;
        }

        let t_size = Uint128::from(Self::triple_size(t) as u128);
        if t_size > self.store.limits.max_triple_byte_size {
            Err(StoreError::TripleByteSize(
                t_size,
                self.store.limits.max_triple_byte_size,
            ))?;
        }

        self.store.stat.byte_size += t_size;
        if self.store.stat.byte_size > self.store.limits.max_byte_size {
            Err(StoreError::ByteSize(self.store.limits.max_byte_size))?;
        }
        if self.store.stat.byte_size - self.initial_byte_size
            > self.store.limits.max_insert_data_byte_size
        {
            Err(StoreError::InsertDataByteSize(
                self.store.limits.max_insert_data_byte_size,
            ))?;
        }

        let triple = self.rio_to_triple(t)?;
        let object_hash: Hash = triple.object.as_hash();
        triples()
            .save(
                self.storage,
                (
                    object_hash.as_bytes(),
                    triple.predicate.key(),
                    triple.subject.key(),
                ),
                &triple,
            )
            .map_err(ContractError::Std)
    }

    pub fn delete_all(&mut self, atoms: &[rdf::Atom]) -> Result<Uint128, ContractError> {
        for atom in atoms {
            self.delete_triple(atom)?;
        }
        self.finish()
    }

    pub fn delete_triple(&mut self, atom: &rdf::Atom) -> Result<(), ContractError> {
        let triple = self.rio_to_triple(atom.into())?;
        let object_hash: Hash = triple.object.as_hash();

        self.store.stat.triple_count -= Uint128::one();
        triples()
            .remove(
                self.storage,
                (
                    object_hash.as_bytes(),
                    triple.predicate.key(),
                    triple.subject.key(),
                ),
            )
            .map_err(ContractError::Std)
    }

    /// Flushes the store to the storage.
    /// Returns the number of triples added or removed (absolute value).
    pub fn finish(&mut self) -> Result<Uint128, ContractError> {
        STORE.save(self.storage, &self.store)?;
        NAMESPACE_KEY_INCREMENT.save(self.storage, &self.ns_key_inc_offset)?;
        for entry in &self.ns_cache {
            namespaces().save(self.storage, entry.0.to_string(), entry.1)?;
        }

        if self.store.stat.triple_count > self.initial_triple_count {
            Ok(self.store.stat.triple_count - self.initial_triple_count)
        } else {
            Ok(self.initial_triple_count - self.store.stat.triple_count)
        }
    }

    fn resolve_namespace_key(&mut self, ns_str: String) -> StdResult<u128> {
        if let Some(namespace) = self.ns_cache.get_mut(&ns_str) {
            namespace.counter += 1;
            Ok(namespace.key)
        } else {
            let mut namespace = match namespaces().load(self.storage, ns_str.clone()) {
                Err(StdError::NotFound { .. }) => Ok(self.allocate_namespace(ns_str.clone())),
                Ok(n) => Ok(n),
                Err(e) => Err(e),
            }?;

            namespace.counter += 1;
            self.ns_cache.insert(ns_str, namespace.clone());
            Ok(namespace.key)
        }
    }

    fn allocate_namespace(&mut self, value: String) -> Namespace {
        self.store.stat.namespace_count += Uint128::one();

        let ns = Namespace {
            value,
            key: self.ns_key_inc_offset,
            counter: 0u128,
        };
        self.ns_key_inc_offset += 1;

        ns
    }

    fn rio_to_triple(&mut self, triple: model::Triple<'_>) -> StdResult<Triple> {
        Ok(Triple {
            subject: self.rio_to_subject(triple.subject)?,
            predicate: self.rio_to_node(triple.predicate)?,
            object: self.rio_to_object(triple.object)?,
        })
    }

    fn rio_to_subject(&mut self, subject: model::Subject<'_>) -> StdResult<Subject> {
        match subject {
            model::Subject::NamedNode(node) => self.rio_to_node(node).map(Subject::Named),
            model::Subject::BlankNode(node) => Ok(Subject::Blank(node.id.to_string())),
            model::Subject::Triple(_) => Err(StdError::generic_err("RDF star syntax unsupported")),
        }
    }

    fn rio_to_node(&mut self, node: model::NamedNode<'_>) -> StdResult<Node> {
        let (ns, v) = rdf::explode_iri(node.iri)?;
        Ok(Node {
            namespace: self.resolve_namespace_key(ns)?,
            value: v,
        })
    }

    fn rio_to_object(&mut self, object: Term<'_>) -> StdResult<Object> {
        match object {
            Term::BlankNode(node) => Ok(Object::Blank(node.id.to_string())),
            Term::NamedNode(node) => self.rio_to_node(node).map(Object::Named),
            Term::Literal(literal) => self.rio_to_literal(literal).map(Object::Literal),
            Term::Triple(_) => Err(StdError::generic_err("RDF star syntax unsupported")),
        }
    }

    fn rio_to_literal(&mut self, literal: model::Literal<'_>) -> StdResult<Literal> {
        match literal {
            model::Literal::Simple { value } => Ok(Literal::Simple {
                value: value.to_string(),
            }),
            model::Literal::LanguageTaggedString { value, language } => Ok(Literal::I18NString {
                value: value.to_string(),
                language: language.to_string(),
            }),
            model::Literal::Typed { value, datatype } => {
                self.rio_to_node(datatype).map(|node| Literal::Typed {
                    value: value.to_string(),
                    datatype: node,
                })
            }
        }
    }

    fn triple_size(triple: model::Triple<'_>) -> usize {
        Self::subject_size(triple.subject)
            + Self::node_size(triple.predicate)
            + Self::object_size(triple.object)
    }

    fn subject_size(subject: model::Subject<'_>) -> usize {
        match subject {
            model::Subject::NamedNode(n) => Self::node_size(n),
            model::Subject::BlankNode(n) => n.id.len(),
            model::Subject::Triple(_) => 0,
        }
    }

    fn node_size(node: model::NamedNode<'_>) -> usize {
        node.iri.len()
    }

    fn object_size(term: Term<'_>) -> usize {
        match term {
            Term::NamedNode(n) => Self::node_size(n),
            Term::BlankNode(n) => n.id.len(),
            Term::Literal(l) => match l {
                model::Literal::Simple { value } => value.len(),
                model::Literal::LanguageTaggedString { value, language } => {
                    value.len() + language.len()
                }
                model::Literal::Typed { value, datatype } => {
                    value.len() + Self::node_size(datatype)
                }
            },
            Term::Triple(_) => 0,
        }
    }
}
