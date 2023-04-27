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
use std::collections::BTreeMap;
use std::io::BufRead;

pub struct TripleStorer<'a> {
    storage: &'a mut dyn Storage,
    store: Store,
    ns_key_inc_offset: u128,
    ns_cache: BTreeMap<String, Namespace>,
    initial_triple_count: Uint128,
}

impl<'a> TripleStorer<'a> {
    pub fn new(storage: &'a mut dyn Storage) -> StdResult<Self> {
        let store = STORE.load(storage)?;
        let ns_key_inc_offset = NAMESPACE_KEY_INCREMENT.load(storage)?;
        Ok(Self {
            storage,
            store: store.clone(),
            ns_key_inc_offset,
            ns_cache: BTreeMap::new(),
            initial_triple_count: store.stat.triples_count,
        })
    }

    pub fn store_all<R: BufRead>(
        &mut self,
        reader: &mut TripleReader<R>,
    ) -> Result<Uint128, ContractError> {
        reader.read_all(|t| self.store_triple(t))?;
        self.finish()
    }

    pub fn store_triple(&mut self, t: model::Triple) -> Result<(), ContractError> {
        let triple = self.triple(t)?;
        self.store.stat.triples_count += Uint128::one();

        if self.store.stat.triples_count > self.store.limits.max_triple_count {
            Err(StoreError::MaxTriplesLimitExceeded(
                self.store.limits.max_triple_count,
            ))?
        }

        let object_hash: Hash = triple.object.as_hash();
        triples()
            .save(
                self.storage,
                (
                    object_hash.as_bytes(),
                    triple.predicate.clone(),
                    triple.subject.clone(),
                ),
                &triple,
            )
            .map_err(ContractError::Std)
    }

    pub fn finish(&mut self) -> Result<Uint128, ContractError> {
        STORE.save(self.storage, &self.store)?;
        NAMESPACE_KEY_INCREMENT.save(self.storage, &self.ns_key_inc_offset)?;
        for entry in &self.ns_cache {
            namespaces().save(self.storage, entry.0.to_string(), entry.1)?;
        }

        Ok(self.store.stat.triples_count - self.initial_triple_count)
    }

    fn resolve_namespace_key(&mut self, ns_str: String) -> StdResult<u128> {
        match self.ns_cache.get_mut(ns_str.as_str()) {
            Some(namespace) => {
                namespace.counter += 1;
                Ok(namespace.key)
            }
            None => {
                let mut namespace = match namespaces().load(self.storage, ns_str.clone()) {
                    Err(StdError::NotFound { .. }) => {
                        let n = Namespace {
                            key: self.ns_key_inc_offset,
                            counter: 0u128,
                        };
                        self.ns_key_inc_offset += 1;
                        Ok(n)
                    }
                    Ok(n) => Ok(n),
                    Err(e) => Err(e),
                }?;

                namespace.counter += 1;
                self.ns_cache.insert(ns_str, namespace.clone());
                Ok(namespace.key)
            }
        }
    }

    fn triple(&mut self, triple: model::Triple) -> StdResult<Triple> {
        Ok(Triple {
            subject: self.subject(triple.subject)?,
            predicate: self.node(triple.predicate)?,
            object: self.object(triple.object)?,
        })
    }

    fn subject(&mut self, subject: model::Subject) -> StdResult<Subject> {
        match subject {
            model::Subject::NamedNode(node) => self.node(node).map(|n| Subject::Named(n)),
            model::Subject::BlankNode(node) => Ok(Subject::Blank(node.id.to_string())),
            _ => Err(StdError::generic_err("RDF star syntax unsupported")),
        }
    }

    fn node(&mut self, node: model::NamedNode) -> StdResult<Node> {
        let (ns, v) = rdf::explode_iri(node.iri)?;
        Ok(Node {
            namespace: self.resolve_namespace_key(ns)?,
            value: v,
        })
    }

    fn object(&mut self, object: model::Term) -> StdResult<Object> {
        match object {
            model::Term::BlankNode(node) => Ok(Object::Blank(node.id.to_string())),
            model::Term::NamedNode(node) => self.node(node).map(|n| Object::Named(n)),
            model::Term::Literal(literal) => self.literal(literal).map(|l| Object::Literal(l)),
            _ => Err(StdError::generic_err("RDF star syntax unsupported")),
        }
    }

    fn literal(&mut self, literal: model::Literal) -> StdResult<Literal> {
        match literal {
            model::Literal::Simple { value } => Ok(Literal::Simple {
                value: value.to_string(),
            }),
            model::Literal::LanguageTaggedString { value, language } => Ok(Literal::I18NString {
                value: value.to_string(),
                language: language.to_string(),
            }),
            model::Literal::Typed { value, datatype } => {
                self.node(datatype).map(|node| Literal::Typed {
                    value: value.to_string(),
                    datatype: node,
                })
            }
        }
    }
}
