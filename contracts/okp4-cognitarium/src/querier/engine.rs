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
                let left = self.eval_node(left);
                let right = self.eval_node(right);
                Box::new(move |vars| -> ResolvedVariablesIterator {
                    Box::new(CartesianProductJoinIterator::new(
                        right(vars.clone()).collect(),
                        left(vars),
                    ))
                })
            }
            QueryNode::ForLoopJoin { left, right } => {
                let left = self.eval_node(left);
                let right = self.eval_node(right);
                Box::new(move |vars| -> ResolvedVariablesIterator {
                    Box::new(left(vars).flat_map(move |v| right(v)))
                })
            }
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

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ResolvedVariable {
    Subject(Subject),
    Predicate(Predicate),
    Object(Object),
}

pub struct ResolvedVariables {
    variables: Vec<Option<ResolvedVariable>>,
}

impl ResolvedVariables {
    pub fn with_capacity(cap: usize) -> Self {
        let mut variables = Vec::with_capacity(cap);
        for i in 0..usize {
            variables.insert(i, None);
        }

        Self { variables }
    }

    /// Merge with another set of resolved variables, returns None if a variable is set on both side
    /// with different values.
    pub fn merge_with(&self, other: Self) -> Option<Self> {
        let mut merged = other.variables.clone();

        for (key, var) in self.variables.iter().enumerate() {
            if let Some(val) = var {
                match &other.variables[key] {
                    Some(other_val) => {
                        if val != other_val {
                            return None;
                        }
                    }
                    None => merged[key] = Some(val.clone()),
                }
            }
        }

        Some(Self { variables: merged })
    }

struct CartesianProductJoinIterator {
    values: Vec<ResolvedVariables>,
    upstream_iter: ResolvedVariablesIterator,
    buffer: VecDeque<StdResult<ResolvedVariables>>,
}

impl CartesianProductJoinIterator {
    fn new(values: Vec<ResolvedVariables>, upstream_iter: ResolvedVariablesIterator) -> Self {
        Self {
            values,
            upstream_iter,
            buffer: VecDeque::with_capacity(values.len()),
        }
    }
}

impl Iterator for CartesianProductJoinIterator {
    type Item = StdResult<ResolvedVariables>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(val) = self.buffer.pop_front() {
                return Some(val);
            }

            let upstream_res = match self.upstream_iter.next() {
                None => None?,
                Some(res) => res,
            };

            match upstream_res {
                Err(err) => {
                    self.buffer.push_back(Err(err));
                }
                Ok(val) => {
                    for downstream_val in self.values {
                        if Some(value) = val.merge_with(downstream_val) {
                            self.buffer.push_back(Ok(value));
                        }
                    }
                }
            }
        }
    }
}
