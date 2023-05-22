use crate::msg::{
    Literal, Node, Prefix, SimpleWhereCondition, TriplePattern, VarOrNode, VarOrNodeOrLiteral,
    WhereClause, WhereCondition, IRI,
};
use crate::querier::plan::{PatternValue, QueryNode, QueryPlan};
use crate::state::{namespaces, Object, Predicate, Subject};
use crate::{rdf, state};
use cosmwasm_std::{StdError, StdResult, Storage};
use std::collections::HashMap;

pub struct PlanBuilder<'a> {
    storage: &'a dyn Storage,
    prefixes: HashMap<String, String>,
    variables: Vec<String>,
}

impl<'a> PlanBuilder<'a> {
    pub fn new(storage: &'a dyn Storage, prefixes: Vec<Prefix>) -> Self {
        Self {
            storage,
            prefixes: Self::make_prefixes(prefixes),
            variables: Vec::new(),
        }
    }

    pub fn build_plan(&mut self, where_clause: WhereClause) -> StdResult<QueryPlan> {
        Err(StdError::generic_err("not implemented"))
    }

    fn make_prefixes(as_list: Vec<Prefix>) -> HashMap<String, String> {
        as_list.iter().fold(HashMap::new(), |mut map, prefix| {
            map.insert(prefix.prefix.clone(), prefix.namespace.clone());
            map
        })
    }
}
