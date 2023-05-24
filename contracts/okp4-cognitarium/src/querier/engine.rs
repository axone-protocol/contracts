use crate::querier::plan::{PatternValue, QueryNode, QueryPlan};
use crate::querier::variable::{ResolvedVariable, ResolvedVariables};
use crate::state::{triples, Literal, Object, Predicate, Subject, Triple};
use cosmwasm_std::{Order, StdResult, Storage};
use std::collections::VecDeque;
use std::iter;
use std::rc::Rc;

pub struct QueryEngine<'a> {
    storage: &'a dyn Storage,
}

impl<'a> QueryEngine<'a> {
    pub fn new(storage: &'a dyn Storage) -> Self {
        Self { storage }
    }

    pub fn eval_plan(&'a self, plan: QueryPlan) -> ResolvedVariablesIterator {
        return self.eval_node(plan.entrypoint)(ResolvedVariables::with_capacity(
            plan.variables.len(),
        ));
    }

    fn eval_node(
        &'a self,
        node: Box<QueryNode>,
    ) -> Rc<dyn Fn(ResolvedVariables) -> ResolvedVariablesIterator<'a> + 'a> {
        match *node {
            QueryNode::TriplePattern {
                subject,
                predicate,
                object,
            } => Rc::new(move |vars| {
                Box::new(TriplePatternIterator::new(
                    self.storage,
                    vars,
                    subject.clone(),
                    predicate.clone(),
                    object.clone(),
                ))
            }),
            QueryNode::CartesianProductJoin { left, right } => {
                let left = self.eval_node(left);
                let right = self.eval_node(right);
                Rc::new(move |vars| {
                    let mut buffered_errors = VecDeque::new();
                    let values = right(vars.clone())
                        .filter_map(|res| match res {
                            Ok(v) => Some(v),
                            Err(e) => {
                                buffered_errors.push_back(Err(e));
                                None
                            }
                        })
                        .collect();
                    Box::new(CartesianProductJoinIterator::new(
                        values,
                        left(vars),
                        buffered_errors,
                    ))
                })
            }
            QueryNode::ForLoopJoin { left, right } => {
                let left = self.eval_node(left);
                let right = self.eval_node(right);
                Rc::new(move |vars| {
                    let right = Rc::clone(&right);
                    Box::new(ForLoopJoinIterator::new(left(vars), right))
                })
            }
            QueryNode::Skip { child, first } => {
                let upstream = self.eval_node(child);
                Rc::new(move |vars| Box::new(upstream(vars).skip(first)))
            }
            QueryNode::Limit { child, first } => {
                let upstream = self.eval_node(child);
                Rc::new(move |vars| Box::new(upstream(vars).take(first)))
            }
        }
    }
}

type ResolvedVariablesIterator<'a> = Box<dyn Iterator<Item = StdResult<ResolvedVariables>> + 'a>;

struct ForLoopJoinIterator<'a> {
    left: ResolvedVariablesIterator<'a>,
    right: Rc<dyn Fn(ResolvedVariables) -> ResolvedVariablesIterator<'a> + 'a>,
    current: ResolvedVariablesIterator<'a>,
}

impl<'a> ForLoopJoinIterator<'a> {
    fn new(
        left: ResolvedVariablesIterator<'a>,
        right: Rc<dyn Fn(ResolvedVariables) -> ResolvedVariablesIterator<'a> + 'a>,
    ) -> Self {
        Self {
            left,
            right,
            current: Box::new(iter::empty()),
        }
    }
}

impl<'a> Iterator for ForLoopJoinIterator<'a> {
    type Item = StdResult<ResolvedVariables>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(v) = self.current.next() {
                return Some(v);
            }

            match self.left.next() {
                None => None?,
                Some(v) => {
                    self.current = match v {
                        Ok(v) => (self.right)(v),
                        Err(e) => Box::new(iter::once(Err(e))),
                    }
                }
            }
        }
    }
}

struct CartesianProductJoinIterator<'a> {
    values: Vec<ResolvedVariables>,
    upstream_iter: ResolvedVariablesIterator<'a>,
    buffer: VecDeque<StdResult<ResolvedVariables>>,
}

impl<'a> CartesianProductJoinIterator<'a> {
    fn new(
        values: Vec<ResolvedVariables>,
        upstream_iter: ResolvedVariablesIterator<'a>,
        buffer: VecDeque<StdResult<ResolvedVariables>>,
    ) -> Self {
        Self {
            values,
            upstream_iter,
            buffer,
        }
    }
}

impl<'a> Iterator for CartesianProductJoinIterator<'a> {
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
                    for downstream_val in &self.values {
                        if let Some(value) = val.merge_with(downstream_val) {
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
    triple_iter: Box<dyn Iterator<Item = StdResult<Triple>> + 'a>,
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

    fn make_state_iter(
        storage: &'a dyn Storage,
        filters: (Option<Subject>, Option<Predicate>, Option<Object>),
    ) -> Box<dyn Iterator<Item = StdResult<Triple>> + 'a> {
        match filters {
            (Some(s), Some(p), Some(o)) => Box::new(iter::once(
                triples().load(storage, (o.as_hash().as_bytes(), p.key(), s.key())),
            )),
            (Some(s), Some(p), None) => Box::new(
                triples()
                    .idx
                    .subject_and_predicate
                    .prefix((s.key(), p.key()))
                    .range(storage, None, None, Order::Ascending)
                    .map(|res| res.map(|(_, t)| t)),
            ),
            (None, Some(p), Some(o)) => Box::new(
                triples()
                    .prefix((o.as_hash().as_bytes(), p.key()))
                    .range(storage, None, None, Order::Ascending)
                    .map(|res| res.map(|(_, t)| t)),
            ),
            (Some(s), None, Some(o)) => Box::new(
                triples()
                    .idx
                    .subject_and_predicate
                    .sub_prefix(s.key())
                    .range(storage, None, None, Order::Ascending)
                    .filter(move |res| match res {
                        Ok((_, triple)) => triple.object == o,
                        Err(_) => true,
                    })
                    .map(|res| res.map(|(_, t)| t)),
            ),
            (Some(s), None, None) => Box::new(
                triples()
                    .idx
                    .subject_and_predicate
                    .sub_prefix(s.key())
                    .range(storage, None, None, Order::Ascending)
                    .map(|res| res.map(|(_, t)| t)),
            ),
            (None, Some(p), None) => Box::new(
                triples()
                    .range(storage, None, None, Order::Ascending)
                    .filter(move |res| match res {
                        Ok((_, triple)) => triple.predicate == p,
                        Err(_) => true,
                    })
                    .map(|res| res.map(|(_, t)| t)),
            ),
            (None, None, Some(o)) => Box::new(
                triples()
                    .sub_prefix(o.as_hash().as_bytes())
                    .range(storage, None, None, Order::Ascending)
                    .map(|res| res.map(|(_, t)| t)),
            ),
            (None, None, None) => Box::new(
                triples()
                    .range(storage, None, None, Order::Ascending)
                    .map(|res| res.map(|(_, t)| t)),
            ),
        }
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
        M: FnOnce(&ResolvedVariable) -> Option<T>,
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

impl<'a> Iterator for TriplePatternIterator<'a> {
    type Item = StdResult<ResolvedVariables>;

    fn next(&mut self) -> Option<Self::Item> {
        self.triple_iter.next().map(|res| {
            res.map(|triple| -> ResolvedVariables {
                let mut vars: ResolvedVariables = self.input.clone();

                if let Some(v) = self.output_bindings.0 {
                    vars.set(v, ResolvedVariable::Subject(triple.subject.clone()));
                }
                if let Some(v) = self.output_bindings.1 {
                    vars.set(v, ResolvedVariable::Predicate(triple.predicate.clone()));
                }
                if let Some(v) = self.output_bindings.2 {
                    vars.set(v, ResolvedVariable::Object(triple.object.clone()));
                }

                vars
            })
        })
    }
}
