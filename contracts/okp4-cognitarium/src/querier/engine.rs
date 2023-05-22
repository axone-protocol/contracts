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
            } => Box::new(move |vars| {
                Box::new(TriplePatternIterator::new(
                    self.storage,
                    vars,
                    subject,
                    predicate,
                    object,
                ))
            }),
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

impl ResolvedVariable {
    fn as_subject(&self) -> Option<Subject> {
        Some(match self {
            ResolvedVariable::Subject(s) => s.clone(),
            ResolvedVariable::Predicate(p) => Subject::Named(p.clone()),
            ResolvedVariable::Object(o) => match o {
                Object::Named(node) => Subject::Named(node.clone()),
                Object::Blank(node) => Subject::Blank(node.clone()),
                Object::Literal(_) => None?,
            },
        })
    }

    fn as_predicate(&self) -> Option<Predicate> {
        Some(match self {
            ResolvedVariable::Subject(s) => match s {
                Subject::Named(node) => node.clone(),
                Subject::Blank(_) => None?,
            },
            ResolvedVariable::Predicate(p) => p.clone(),
            ResolvedVariable::Object(o) => match o {
                Object::Named(node) => node.clone(),
                Object::Blank(_) => None,
                Object::Literal(_) => None?,
            },
        })
    }

    fn as_object(&self) -> Option<Object> {
        Some(match self {
            ResolvedVariable::Subject(s) => match s {
                Subject::Named(node) => Object::Named(node.clone()),
                Subject::Blank(node) => Object::Blank(node.clone()),
            },
            ResolvedVariable::Predicate(p) => Object::Named(p.clone()),
            ResolvedVariable::Object(o) => o.clone(),
        })
    }
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

    fn set(&mut self, index: usize, var: ResolvedVariable) {
        self.variables[index] = Some(var)
    }

    fn get(&self, index: usize) -> &Option<ResolvedVariable> {
        self.variables
            .get(index)
            .unwrap_or((None as &Option<ResolvedVariable>))
    }
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

struct TriplePatternIterator<'a> {
    input: ResolvedVariables,
    output_bindings: (Option<usize>, Option<usize>, Option<usize>),
    triple_iter: Box<dyn Iterator<Item = StdResult<Triple>>>,
}

impl<'a> TriplePatternIterator<'a> {
    fn new(
        storage: &'a dyn Storage,
        input: ResolvedVariables,
        subject: PatternValue<Subject>,
        predicate: PatternValue<Predicate>,
        object: PatternValue<Object>,
    ) -> Self {
        let (filters, output_bindings) = Self::compute_iter_io(&input, subject, predicate, object);

        Self {
            input,
            output_bindings,
            triple_iter: Self::make_state_iter(storage, filters),
        }
    }

    fn make_state_iter<'a>(
        storage: &'a dyn Storage,
        filters: (Option<Subject>, Option<Predicate>, Option<Object>),
    ) -> Box<dyn Iterator<Item = StdResult<Triple>> + 'a> {
        Box::new(match filters {
            (Some(s), Some(p), Some(o)) => {
                iter::once(triples().load(storage, (o.as_hash().as_bytes(), p, s)))
            }
            (Some(s), Some(p), None) => triples()
                .idx
                .subject_and_predicate
                .prefix((s, p))
                .range(storage, None, None, Order::Ascending)
                .map(|(_, t)| t),
            (None, Some(p), Some(o)) => triples()
                .prefix((o.as_hash().as_bytes(), p))
                .range(storage, None, None, Order::Ascending)
                .map(|(_, t)| t),
            (Some(s), None, Some(o)) => triples()
                .idx
                .subject_and_predicate
                .sub_prefix(s)
                .range(storage, None, None, Order::Ascending)
                .filter(|res| match res {
                    Ok((_, triple)) => triple.object == o,
                    Err(_) => true,
                })
                .map(|(_, t)| t),
            (Some(s), None, None) => triples()
                .idx
                .subject_and_predicate
                .sub_prefix(s)
                .range(storage, None, None, Order::Ascending)
                .map(|(_, t)| t),
            (None, Some(p), None) => triples()
                .range(storage, None, None, Order::Ascending)
                .filter(|res| match res {
                    Ok((_, triple)) => triple.predicate == p,
                    Err(_) => true,
                })
                .map(|(_, t)| t),
            (None, None, Some(o)) => triples()
                .sub_prefix(o.as_hash().as_bytes())
                .range(storage, None, None, Order::Ascending)
                .map(|(_, t)| t),
            (None, None, None) => triples()
                .range(storage, None, None, Order::Ascending)
                .map(|(_, t)| t),
        })
    }

    fn compute_iter_io(
        input: &ResolvedVariables,
        subject: PatternValue<Subject>,
        predicate: PatternValue<Predicate>,
        object: PatternValue<Object>,
    ) -> (
        (Option<Subject>, Option<Predicate>, Option<Object>),
        (Option<usize>, Option<usize>, Option<usize>),
    ) {
        let (s_filter, s_bind) =
            Self::resolve_pattern_part(subject, ResolvedVariable::as_subject, input);
        let (p_filter, p_bind) =
            Self::resolve_pattern_part(predicate, ResolvedVariable::as_predicate, input);
        let (o_filter, o_bind) =
            Self::resolve_pattern_part(object, ResolvedVariable::as_object, input);

        ((s_filter, p_filter, o_filter), (s_bind, p_bind, o_bind))
    }

    fn resolve_pattern_part<T, M>(
        pattern_part: PatternValue<T>,
        map_fn: M,
        input: &ResolvedVariables,
    ) -> (Option<T>, Option<usize>)
    where
        M: FnOnce(&ResolvedVariable) -> Some(T),
    {
        match pattern_part {
            PatternValue::Constant(s) => (Some(s), None),
            PatternValue::Variable(v) => match input.get(v) {
                Some(var) => match map_fn(var) {
                    None => (None, Some(v)),
                    Some(val) => (Some(val), None),
                },
                None => (None, Some(v)),
            },
        }
    }
}

impl Iterator for TriplePatternIterator {
    type Item = StdResult<ResolvedVariables>;

    fn next(&mut self) -> Option<Self::Item> {
        self.triple_iter.next().map(|res| {
            res.map(|triple| -> ResolvedVariables {
                let mut vars: ResolvedVariables = self.input.clone();
                for (part, var_idx) in self.output {
                    let var = match part {
                        TriplePart::Subject => ResolvedVariable::Subject(triple.subject.clone()),
                        TriplePart::Predicate => {
                            ResolvedVariable::Predicate(triple.predicate.clone())
                        }
                        TriplePart::Object => ResolvedVariable::Object(triple.object.clone()),
                    };
                    vars.set(var_idx, var);
                }
                vars
            })
        })
    }
}
