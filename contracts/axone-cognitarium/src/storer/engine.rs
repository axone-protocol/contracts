use crate::error::StoreError;
use crate::state::{
    triples, Literal, NamespaceBatchService, NamespaceQuerier, Node, Object, Store, Subject,
    Triple, BLANK_NODE_IDENTIFIER_COUNTER, BLANK_NODE_SIZE, STORE,
};
use crate::ContractError;
use axone_rdf::normalize::IdentifierIssuer;
use axone_rdf::serde::TripleReader;
use axone_rdf::uri::explode_iri;
use cosmwasm_std::{StdError, StdResult, Storage, Uint128};
use rio_api::model;
use rio_api::model::Term;
use std::io::BufRead;
use std::ops::Neg;

pub struct StoreEngine<'a> {
    storage: &'a mut dyn Storage,
    store: Store,
    ns_batch_svc: NamespaceBatchService,
    blank_node_id_issuer: IdentifierIssuer,
    initial_triple_count: Uint128,
    initial_byte_size: Uint128,
}

impl<'a> StoreEngine<'a> {
    pub fn new(storage: &'a mut dyn Storage) -> StdResult<Self> {
        let store = STORE.load(storage)?;
        let blank_node_id_counter = BLANK_NODE_IDENTIFIER_COUNTER.load(storage)?;
        let ns_batch_svc = NamespaceBatchService::new(storage)?;
        Ok(Self {
            storage,
            store: store.clone(),
            ns_batch_svc,
            blank_node_id_issuer: IdentifierIssuer::new("", blank_node_id_counter),
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

        let triple = Self::rio_to_triple(
            t,
            &mut |ns_str| {
                self.ns_batch_svc
                    .resolve_or_allocate(self.storage, ns_str)
                    .map(|ns| ns.key)
            },
            &mut self.blank_node_id_issuer,
        )?;
        let t_size = Uint128::from(self.triple_size(&triple).map_err(ContractError::Std)? as u128);
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

        let mut new_ns_refs = Vec::new();
        triples()
            .update(
                self.storage,
                (
                    triple.object.as_hash().as_bytes(),
                    triple.predicate.key(),
                    triple.subject.key(),
                ),
                |maybe_triple| {
                    if let Some(t) = maybe_triple {
                        self.store.stat.triple_count -= Uint128::one();
                        self.store.stat.byte_size -= t_size;
                        Ok(t)
                    } else {
                        new_ns_refs.append(&mut triple.namespaces());
                        Ok(triple)
                    }
                },
            )
            .map_err(ContractError::Std)?;

        for ns_key in new_ns_refs {
            self.ns_batch_svc.count_ref(self.storage, ns_key)?;
        }
        Ok(())
    }

    pub fn delete_all(&mut self, triples: &[Triple]) -> Result<Uint128, ContractError> {
        for triple in triples {
            self.delete_triple(triple)?;
        }
        self.finish()
    }

    fn delete_triple(&mut self, triple: &Triple) -> Result<(), ContractError> {
        let old = triples()
            .may_load(
                self.storage,
                (
                    triple.object.as_hash().as_bytes(),
                    triple.predicate.key(),
                    triple.subject.key(),
                ),
            )
            .map_err(ContractError::Std)?;

        if old.is_some() {
            triples().replace(
                self.storage,
                (
                    triple.object.as_hash().as_bytes(),
                    triple.predicate.key(),
                    triple.subject.key(),
                ),
                None,
                old.as_ref(),
            )?;
            self.store.stat.triple_count -= Uint128::one();
            let triple_size = self.triple_size(triple).map_err(ContractError::Std)?;
            self.store.stat.byte_size -= Uint128::from(triple_size as u128);

            for ns_key in triple.namespaces() {
                self.ns_batch_svc
                    .free_ref(self.storage, ns_key)
                    .map_err(ContractError::Std)?;
            }
        }
        Ok(())
    }

    /// Flushes the store to the storage.
    /// Returns the number of triples added or removed (absolute value).
    fn finish(&mut self) -> Result<Uint128, ContractError> {
        let ns_diff = self.ns_batch_svc.flush(self.storage)?;
        if ns_diff > 0 {
            self.store.stat.namespace_count += Uint128::new(ns_diff as u128);
        } else {
            self.store.stat.namespace_count -= Uint128::new(ns_diff.neg() as u128);
        }

        BLANK_NODE_IDENTIFIER_COUNTER.save(self.storage, &self.blank_node_id_issuer.counter)?;

        STORE.save(self.storage, &self.store)?;

        let count_diff = self
            .store
            .stat
            .triple_count
            .abs_diff(self.initial_triple_count);

        self.initial_triple_count = self.store.stat.triple_count;
        self.initial_byte_size = self.store.stat.byte_size;

        Ok(count_diff)
    }

    fn rio_to_triple<F>(
        triple: model::Triple<'_>,
        ns_fn: &mut F,
        id_issuer: &mut IdentifierIssuer,
    ) -> StdResult<Triple>
    where
        F: FnMut(String) -> StdResult<u128>,
    {
        Ok(Triple {
            subject: Self::rio_to_subject(triple.subject, ns_fn, id_issuer)?,
            predicate: Self::rio_to_node(triple.predicate, ns_fn)?,
            object: Self::rio_to_object(triple.object, ns_fn, id_issuer)?,
        })
    }

    fn rio_to_subject<F>(
        subject: model::Subject<'_>,
        ns_fn: &mut F,
        id_issuer: &mut IdentifierIssuer,
    ) -> StdResult<Subject>
    where
        F: FnMut(String) -> StdResult<u128>,
    {
        match subject {
            model::Subject::NamedNode(node) => Self::rio_to_node(node, ns_fn).map(Subject::Named),
            model::Subject::BlankNode(node) => Ok(Subject::Blank(
                id_issuer.get_n_or_issue(node.id.to_string()),
            )),
            model::Subject::Triple(_) => Err(StdError::generic_err("RDF star syntax unsupported")),
        }
    }

    fn rio_to_node<F>(node: model::NamedNode<'_>, ns_fn: &mut F) -> StdResult<Node>
    where
        F: FnMut(String) -> StdResult<u128>,
    {
        let (ns, v) = explode_iri(node.iri)?;
        Ok(Node {
            namespace: ns_fn(ns)?,
            value: v,
        })
    }

    fn rio_to_object<F>(
        object: Term<'_>,
        ns_fn: &mut F,
        id_issuer: &mut IdentifierIssuer,
    ) -> StdResult<Object>
    where
        F: FnMut(String) -> StdResult<u128>,
    {
        match object {
            Term::BlankNode(node) => {
                Ok(Object::Blank(id_issuer.get_n_or_issue(node.id.to_string())))
            }
            Term::NamedNode(node) => Self::rio_to_node(node, ns_fn).map(Object::Named),
            Term::Literal(literal) => Self::rio_to_literal(literal, ns_fn).map(Object::Literal),
            Term::Triple(_) => Err(StdError::generic_err("RDF star syntax unsupported")),
        }
    }

    fn rio_to_literal<F>(literal: model::Literal<'_>, ns_fn: &mut F) -> StdResult<Literal>
    where
        F: FnMut(String) -> StdResult<u128>,
    {
        match literal {
            model::Literal::Simple { value } => Ok(Literal::Simple {
                value: value.to_string(),
            }),
            model::Literal::LanguageTaggedString { value, language } => Ok(Literal::I18NString {
                value: value.to_string(),
                language: language.to_string(),
            }),
            model::Literal::Typed { value, datatype } => {
                Self::rio_to_node(datatype, ns_fn).map(|node| Literal::Typed {
                    value: value.to_string(),
                    datatype: node,
                })
            }
        }
    }

    fn triple_size(&mut self, triple: &Triple) -> StdResult<usize> {
        Ok(self.subject_size(&triple.subject)?
            + self.node_size(&triple.predicate)?
            + self.object_size(&triple.object)?)
    }

    fn subject_size(&mut self, subject: &Subject) -> StdResult<usize> {
        match subject {
            Subject::Named(n) => self.node_size(n),
            Subject::Blank(_) => Ok(BLANK_NODE_SIZE),
        }
    }

    fn node_size(&mut self, node: &Node) -> StdResult<usize> {
        self.ns_batch_svc
            .resolve_from_key(self.storage, node.namespace)
            .and_then(NamespaceQuerier::none_as_error_middleware)
            .map(|ns| ns.value.len() + node.value.len())
    }

    fn object_size(&mut self, object: &Object) -> StdResult<usize> {
        Ok(match object {
            Object::Blank(_) => BLANK_NODE_SIZE,
            Object::Named(n) => self.node_size(n)?,
            Object::Literal(l) => match l {
                Literal::Simple { value } => value.len(),
                Literal::I18NString { value, language } => value.len() + language.len(),
                Literal::Typed { value, datatype } => value.len() + self.node_size(datatype)?,
            },
        })
    }
}
