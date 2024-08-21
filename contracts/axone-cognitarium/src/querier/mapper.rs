use crate::msg::{Literal, IRI};
use crate::state;
use crate::state::{NamespaceSolver, Object};
use axone_rdf::uri::{expand_uri, explode_iri};
use cosmwasm_std::StdResult;
use std::collections::HashMap;

pub fn literal_as_object(
    ns_solver: &mut dyn NamespaceSolver,
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
            datatype: iri_as_node(ns_solver, prefixes, datatype)?,
        },
    }))
}

pub fn iri_as_node(
    ns_solver: &mut dyn NamespaceSolver,
    prefixes: &HashMap<String, String>,
    iri: IRI,
) -> StdResult<state::Node> {
    match iri {
        IRI::Prefixed(prefixed) => expand_uri(&prefixed, prefixes),
        IRI::Full(full) => Ok(full),
    }
    .and_then(|iri| explode_iri(&iri))
    .and_then(|(ns_key, v)| {
        ns_solver.resolve_from_val(ns_key).map(|ns| state::Node {
            namespace: ns.key,
            value: v,
        })
    })
}

pub fn iri_as_string(iri: IRI, prefixes: &HashMap<String, String>) -> StdResult<String> {
    match iri {
        IRI::Prefixed(prefixed) => expand_uri(&prefixed, prefixes),
        IRI::Full(full) => Ok(full),
    }
}
