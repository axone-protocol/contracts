use crate::msg::{Literal, Node, IRI};
use crate::state;
use crate::state::{NamespaceResolver, Object, Predicate, Subject};
use cosmwasm_std::{StdError, StdResult, Storage};
use okp4_rdf::uri::{expand_uri, explode_iri};
use std::collections::HashMap;

pub fn node_as_subject(
    ns_resolver: &mut NamespaceResolver,
    storage: &dyn Storage,
    prefixes: &HashMap<String, String>,
    node: Node,
) -> StdResult<Subject> {
    Ok(match node {
        Node::NamedNode(iri) => Subject::Named(iri_as_node(ns_resolver, storage, prefixes, iri)?),
        Node::BlankNode(blank) => Subject::Blank(blank),
    })
}

pub fn node_as_predicate(
    ns_resolver: &mut NamespaceResolver,
    storage: &dyn Storage,
    prefixes: &HashMap<String, String>,
    node: Node,
) -> StdResult<Predicate> {
    match node {
        Node::NamedNode(iri) => iri_as_node(ns_resolver, storage, prefixes, iri),
        Node::BlankNode(_) => Err(StdError::generic_err(
            "Predicate pattern must be a named node",
        )),
    }
}

pub fn node_as_object(
    ns_resolver: &mut NamespaceResolver,
    storage: &dyn Storage,
    prefixes: &HashMap<String, String>,
    node: Node,
) -> StdResult<Object> {
    Ok(match node {
        Node::NamedNode(iri) => Object::Named(iri_as_node(ns_resolver, storage, prefixes, iri)?),
        Node::BlankNode(blank) => Object::Blank(blank),
    })
}

pub fn literal_as_object(
    ns_resolver: &mut NamespaceResolver,
    storage: &dyn Storage,
    prefixes: &HashMap<String, String>,
    literal: Literal,
) -> StdResult<Object> {
    Ok(Object::Literal(match literal {
        Literal::Simple(value) => state::Literal::Simple { value },
        Literal::LanguageTaggedString { value, language } => {
            state::Literal::I18NString { value, language }
        }
        Literal::TypedValue { value, datatype } => state::Literal::Typed {
            value,
            datatype: iri_as_node(ns_resolver, storage, prefixes, datatype)?,
        },
    }))
}

pub fn iri_as_node(
    ns_resolver: &mut NamespaceResolver,
    storage: &dyn Storage,
    prefixes: &HashMap<String, String>,
    iri: IRI,
) -> StdResult<state::Node> {
    match iri {
        IRI::Prefixed(prefixed) => expand_uri(&prefixed, prefixes),
        IRI::Full(full) => Ok(full),
    }
    .and_then(|iri| explode_iri(&iri))
    .and_then(|(ns_key, v)| {
        ns_resolver
            .resolve_from_val(storage, ns_key)
            .and_then(NamespaceResolver::none_as_error_middleware)
            .map(|ns| state::Node {
                namespace: ns.key,
                value: v,
            })
    })
}
