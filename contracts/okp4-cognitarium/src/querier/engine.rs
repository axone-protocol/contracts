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
        return self.eval_node(plan.entrypoint)(ResolvedVariables::with_capacity(
            plan.variables.len(),
        ));
    }

    fn eval_node(
        &self,
        node: QueryNode,
    ) -> Box<dyn Fn(ResolvedVariables) -> ResolvedVariablesIterator> {
        match node {
            QueryNode::TriplePattern {
                subject,
                predicate,
                object,
            } => Box::new(move |_| Box::new(iter::empty())),
            QueryNode::CartesianProductJoin { left, right } => {
                Box::new(move |_| Box::new(iter::empty()))
            }
            QueryNode::ForLoopJoin { left, right } => Box::new(move |_| Box::new(iter::empty())),
            QueryNode::Skip { child, first } => {
                let upstream = self.eval_node(child);
                Box::new(move |vars| -> ResolvedVariablesIterator {
                    Box::new(upstream(vars).skip(first))
                })
            }
            QueryNode::Limit { child, first } => {
                let upstream = self.eval_node(child);
                Box::new(move |vars| -> ResolvedVariablesIterator {
                    Box::new(upstream(vars).take(first))
                })
            }
        }
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

impl ResolvedVariables {
    pub fn with_capacity(cap: usize) -> Self {
        let mut variables = Vec::with_capacity(cap);
        for i in 0..usize {
            variables.insert(i, None);
        }

        Self { variables }
    }
}
