use crate::querier::plan::{PatternValue, QueryNode, QueryPlan};
use crate::state::{triples, Object, Predicate, Subject, Triple};
use cosmwasm_std::{Order, StdError, StdResult, Storage};
use cw_storage_plus::IndexList;
use std::collections::VecDeque;
use std::iter;

pub struct QueryEngine<'a> {
    storage: &'a dyn Storage,
}

impl<'a> QueryEngine<'a> {
    pub fn new(storage: &'a dyn Storage) -> Self {
        Self { storage }
    }

    pub fn eval_plan(&self, plan: QueryPlan) -> ResolvedVariablesIterator {
        Box::new(iter::empty())
    }
}

type ResolvedVariablesIterator = Box<dyn Iterator<Item = StdResult<ResolvedVariables>>>;

pub enum ResolvedVariable {
    Subject(Subject),
    Predicate(Predicate),
    Object(Object),
}

pub struct ResolvedVariables {
    pub variables: Vec<Option<ResolvedVariable>>,
}
