use crate::msg::{Head, Results, SelectItem, SelectResponse, Value};
use crate::querier::plan::{PatternValue, QueryNode, QueryPlan};
use crate::querier::variable::{ResolvedVariable, ResolvedVariables};
use crate::state::{triples, NamespaceResolver, Object, Predicate, Subject, Triple};
use cosmwasm_std::{Order, StdError, StdResult, Storage};
use std::collections::{BTreeMap, VecDeque};
use std::iter;
use std::rc::Rc;

pub struct QueryEngine<'a> {
    storage: &'a dyn Storage,
}

impl<'a> QueryEngine<'a> {
    pub fn new(storage: &'a dyn Storage) -> Self {
        Self { storage }
    }

    pub fn select(
        &'a self,
        plan: QueryPlan,
        selection: Vec<SelectItem>,
    ) -> StdResult<SelectResponse> {
        let bindings = selection
            .iter()
            .map(|item| match item {
                SelectItem::Variable(v) => v,
            })
            .map(|name| -> StdResult<(String, usize)> {
                match plan.get_var_index(name) {
                    Some(index) => Ok((name.clone(), index)),
                    None => Err(StdError::generic_err(
                        "Selected variable not found in query",
                    )),
                }
            })
            .collect::<StdResult<BTreeMap<String, usize>>>()?;

        Ok(SelectResponse {
            head: Head {
                vars: bindings.keys().cloned().collect(),
            },
            results: Results {
                bindings: SolutionsIterator::new(self.storage, self.eval_plan(plan), bindings)
                    .collect::<StdResult<Vec<BTreeMap<String, Value>>>>()?,
            },
        })
    }

    pub fn eval_plan(&'a self, plan: QueryPlan) -> ResolvedVariablesIterator<'_> {
        return self.eval_node(plan.entrypoint)(ResolvedVariables::with_capacity(
            plan.variables.len(),
        ));
    }

    fn eval_node(
        &'a self,
        node: QueryNode,
    ) -> Rc<dyn Fn(ResolvedVariables) -> ResolvedVariablesIterator<'a> + 'a> {
        match node {
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
                let left = self.eval_node(*left);
                let right = self.eval_node(*right);
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
                let left = self.eval_node(*left);
                let right = self.eval_node(*right);
                Rc::new(move |vars| {
                    let right = Rc::clone(&right);
                    Box::new(ForLoopJoinIterator::new(left(vars), right))
                })
            }
            QueryNode::Skip { child, first } => {
                let upstream = self.eval_node(*child);
                Rc::new(move |vars| Box::new(upstream(vars).skip(first)))
            }
            QueryNode::Limit { child, first } => {
                let upstream = self.eval_node(*child);
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

type TriplePatternFilters = (Option<Subject>, Option<Predicate>, Option<Object>);
type TriplePatternBindings = (Option<usize>, Option<usize>, Option<usize>);

impl<'a> TriplePatternIterator<'a> {
    fn new(
        storage: &'a dyn Storage,
        input: ResolvedVariables,
        subject: PatternValue<Subject>,
        predicate: PatternValue<Predicate>,
        object: PatternValue<Object>,
    ) -> Self {
        if let Some((filters, output_bindings)) =
            Self::compute_iter_io(&input, subject, predicate, object)
        {
            return Self {
                input,
                output_bindings,
                triple_iter: Self::make_state_iter(storage, filters),
            };
        }

        Self {
            input,
            output_bindings: (None, None, None),
            triple_iter: Box::new(iter::empty()),
        }
    }

    fn make_state_iter(
        storage: &'a dyn Storage,
        filters: TriplePatternFilters,
    ) -> Box<dyn Iterator<Item = StdResult<Triple>> + 'a> {
        match filters {
            (Some(s), Some(p), Some(o)) => {
                let res = triples().load(storage, (o.as_hash().as_bytes(), p.key(), s.key()));
                match res {
                    Err(StdError::NotFound { .. }) => Box::new(iter::empty()),
                    _ => Box::new(iter::once(res)),
                }
            }
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
    ) -> Option<(TriplePatternFilters, TriplePatternBindings)> {
        let (s_filter, s_bind) =
            Self::resolve_pattern_part(subject, ResolvedVariable::as_subject, input)?;
        let (p_filter, p_bind) =
            Self::resolve_pattern_part(predicate, ResolvedVariable::as_predicate, input)?;
        let (o_filter, o_bind) =
            Self::resolve_pattern_part(object, ResolvedVariable::as_object, input)?;

        Some(((s_filter, p_filter, o_filter), (s_bind, p_bind, o_bind)))
    }

    fn resolve_pattern_part<T, M>(
        pattern_part: PatternValue<T>,
        map_fn: M,
        input: &ResolvedVariables,
    ) -> Option<(Option<T>, Option<usize>)>
    where
        M: FnOnce(&ResolvedVariable) -> Option<T>,
    {
        Some(match pattern_part {
            PatternValue::Constant(s) => (Some(s), None),
            PatternValue::Variable(v) => match input.get(v) {
                Some(var) => (Some(map_fn(var)?), None),
                None => (None, Some(v)),
            },
        })
    }
}

impl<'a> Iterator for TriplePatternIterator<'a> {
    type Item = StdResult<ResolvedVariables>;

    fn next(&mut self) -> Option<Self::Item> {
        self.triple_iter.next().map(|res| {
            res.map(|triple| -> ResolvedVariables {
                let mut vars: ResolvedVariables = self.input.clone();

                if let Some(v) = self.output_bindings.0 {
                    vars.set(v, ResolvedVariable::Subject(triple.subject));
                }
                if let Some(v) = self.output_bindings.1 {
                    vars.set(v, ResolvedVariable::Predicate(triple.predicate));
                }
                if let Some(v) = self.output_bindings.2 {
                    vars.set(v, ResolvedVariable::Object(triple.object));
                }

                vars
            })
        })
    }
}

struct SolutionsIterator<'a> {
    storage: &'a dyn Storage,
    ns_resolver: NamespaceResolver,
    iter: ResolvedVariablesIterator<'a>,
    bindings: BTreeMap<String, usize>,
}

impl<'a> SolutionsIterator<'a> {
    fn new(
        storage: &'a dyn Storage,
        iter: ResolvedVariablesIterator<'a>,
        bindings: BTreeMap<String, usize>,
    ) -> Self {
        Self {
            storage,
            ns_resolver: NamespaceResolver::new(),
            iter,
            bindings,
        }
    }
}

impl<'a> Iterator for SolutionsIterator<'a> {
    type Item = StdResult<BTreeMap<String, Value>>;

    fn next(&mut self) -> Option<Self::Item> {
        let resolved_variables = match self.iter.next() {
            None => None?,
            Some(res) => res,
        };

        resolved_variables
            .and_then(|variables| {
                self.bindings
                    .clone()
                    .into_iter()
                    .map(|(name, index)| (name, variables.get(index)))
                    .map(|(name, var)| match var {
                        None => Err(StdError::generic_err(
                            "Couldn't find variable in result set",
                        )),
                        Some(val) => Ok((name, val)),
                    })
                    .map(|res| {
                        res.and_then(|(name, var)| -> StdResult<(String, Value)> {
                            Ok((
                                name,
                                var.as_value(&mut |ns_key| {
                                    let res =
                                        self.ns_resolver.resolve_from_key(self.storage, ns_key);
                                    res.and_then(NamespaceResolver::none_as_error_middleware)
                                        .map(|ns| ns.value)
                                })?,
                            ))
                        })
                    })
                    .collect::<StdResult<BTreeMap<String, Value>>>()
            })
            .into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::msg::{DataFormat, StoreLimitsInput, IRI};
    use crate::rdf::TripleReader;
    use crate::state;
    use crate::state::{Literal, Store, StoreStat, NAMESPACE_KEY_INCREMENT, STORE};
    use crate::storer::StoreEngine;
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::{Addr, Uint128};
    use std::env;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::path::Path;

    fn read_test_data(file: &str) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        File::open(
            Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
                .join("testdata")
                .join(file),
        )
        .unwrap()
        .read_to_end(&mut bytes)
        .unwrap();

        bytes
    }

    fn fill_test_data(storage: &mut dyn Storage) {
        STORE
            .save(
                storage,
                &Store {
                    owner: Addr::unchecked("owner"),
                    limits: StoreLimitsInput::default().into(),
                    stat: StoreStat::default(),
                },
            )
            .unwrap();
        NAMESPACE_KEY_INCREMENT.save(storage, &0u128).unwrap();
        let data = read_test_data("sample.rdf.xml");
        let buf = BufReader::new(data.as_slice());
        let mut reader = TripleReader::new(&DataFormat::RDFXml, buf);
        let mut storer = StoreEngine::new(storage).unwrap();
        let count = storer.store_all(&mut reader).unwrap();

        assert_eq!(count, Uint128::new(40u128));
    }

    #[test]
    fn select() {
        let mut deps = mock_dependencies();
        fill_test_data(deps.as_mut().storage);

        struct TestCase {
            plan: QueryPlan,
            selection: Vec<SelectItem>,
            expects: StdResult<SelectResponse>,
        }

        let cases = vec![
            TestCase {
                plan: QueryPlan {
                    entrypoint: QueryNode::TriplePattern {
                        subject: PatternValue::Variable(0),
                        predicate: PatternValue::Variable(1),
                        object: PatternValue::Variable(2),
                    },
                    variables: vec![
                        "v1".to_string(),
                        "v2".to_string(),
                        "v3".to_string(),
                    ],
                },
                selection: vec![
                    SelectItem::Variable("v4".to_string()),
                ],
                expects: Err(StdError::generic_err("Selected variable not found in query")),
            },
            TestCase {
                plan: QueryPlan {
                    entrypoint: QueryNode::TriplePattern {
                        subject: PatternValue::Constant(Subject::Named(state::Node {
                            namespace: 0,
                            value: "97ff7e16-c08d-47be-8475-211016c82e33".to_string(),
                        })),
                        predicate: PatternValue::Constant(state::Node {
                            namespace: 3,
                            value: "hasRegistrar".to_string(),
                        }),
                        object: PatternValue::Variable(0),
                    },
                    variables: vec![
                        "registrar".to_string(),
                    ],
                },
                selection: vec![
                    SelectItem::Variable("registrar".to_string()),
                ],
                expects: Ok(SelectResponse {
                    head: Head {
                        vars: vec![
                            "registrar".to_string(),
                        ],
                    },
                    results: Results {
                        bindings: vec![
                            BTreeMap::from([
                                ("registrar".to_string(), Value::URI {value: IRI::Full("did:key:0x04d1f1b8f8a7a28f9a5a254c326a963a22f5a5b5d5f5e5d5c5b5a5958575655".to_string())}),
                            ]),
                        ],
                    },
                }),
            },
            TestCase {
                plan: QueryPlan {
                    entrypoint: QueryNode::Limit {
                        child: Box::new(QueryNode::Skip {
                            child: Box::new(QueryNode::TriplePattern {
                                subject: PatternValue::Variable(0),
                                predicate: PatternValue::Variable(1),
                                object: PatternValue::Variable(2),
                            }),
                            first: 10,
                        }),
                        first: 3,
                    },
                    variables: vec![
                        "subject".to_string(),
                        "predicate".to_string(),
                        "object".to_string(),
                    ],
                },
                selection: vec![
                    SelectItem::Variable("subject".to_string()),
                    SelectItem::Variable("predicate".to_string()),
                    SelectItem::Variable("object".to_string()),
                ],
                expects: Ok(SelectResponse {
                    head: Head {
                        vars: vec![
                            "object".to_string(),
                            "predicate".to_string(),
                            "subject".to_string(),
                        ],
                    },
                    results: Results {
                        bindings: vec![
                            BTreeMap::from([
                                ("subject".to_string(), Value::URI {value: IRI::Full("https://ontology.okp4.space/dataverse/dataset/metadata/d1615703-4ee1-4e2f-997e-15aecf1eea4e".to_string())}),
                                ("predicate".to_string(), Value::URI {value: IRI::Full("https://ontology.okp4.space/core/describes".to_string())}),
                                ("object".to_string(), Value::URI {value: IRI::Full("https://ontology.okp4.space/dataverse/dataset/0ea1fc7a-dd97-4adc-a10e-169c6597bcde".to_string())}),
                            ]),
                            BTreeMap::from([
                                ("subject".to_string(), Value::URI {value: IRI::Full("https://ontology.okp4.space/dataverse/dataset/metadata/d1615703-4ee1-4e2f-997e-15aecf1eea4e".to_string())}),
                                ("predicate".to_string(), Value::URI {value: IRI::Full("https://ontology.okp4.space/core/hasDescription".to_string())}),
                                ("object".to_string(), Value::Literal {value: "Un Dataset de test.".to_string(), lang: Some("fr".to_string()), datatype: None }),
                            ]),
                            BTreeMap::from([
                                ("subject".to_string(), Value::URI {value: IRI::Full("https://ontology.okp4.space/dataverse/dataset/metadata/d1615703-4ee1-4e2f-997e-15aecf1eea4e".to_string())}),
                                ("predicate".to_string(), Value::URI {value: IRI::Full("https://ontology.okp4.space/core/hasTitle".to_string())}),
                                ("object".to_string(), Value::Literal { value: "test Dataset".to_string(), lang: Some("en".to_string()), datatype: None }),
                            ]),
                        ],
                    },
                }),
            },
        ];

        for case in cases {
            let engine = QueryEngine::new(&deps.storage);
            assert_eq!(engine.select(case.plan, case.selection), case.expects);
        }
    }

    #[test]
    fn eval_plan() {
        let mut deps = mock_dependencies();
        fill_test_data(deps.as_mut().storage);

        struct TestCase {
            plan: QueryPlan,
            expects: usize,
        }

        let cases = vec![
            TestCase {
                plan: QueryPlan {
                    entrypoint: QueryNode::TriplePattern {
                        subject: PatternValue::Variable(0),
                        predicate: PatternValue::Variable(1),
                        object: PatternValue::Variable(2),
                    },
                    variables: vec!["v1".to_string(), "v2".to_string(), "v3".to_string()],
                },
                expects: 40,
            },
            TestCase {
                plan: QueryPlan {
                    entrypoint: QueryNode::Limit {
                        child: Box::new(QueryNode::TriplePattern {
                            subject: PatternValue::Variable(0),
                            predicate: PatternValue::Variable(1),
                            object: PatternValue::Variable(2),
                        }),
                        first: 30,
                    },
                    variables: vec!["v1".to_string(), "v2".to_string(), "v3".to_string()],
                },
                expects: 30,
            },
            TestCase {
                plan: QueryPlan {
                    entrypoint: QueryNode::Limit {
                        child: Box::new(QueryNode::Skip {
                            child: Box::new(QueryNode::TriplePattern {
                                subject: PatternValue::Variable(0),
                                predicate: PatternValue::Variable(1),
                                object: PatternValue::Variable(2),
                            }),
                            first: 20,
                        }),
                        first: 30,
                    },
                    variables: vec!["v1".to_string(), "v2".to_string(), "v3".to_string()],
                },
                expects: 20,
            },
            TestCase {
                plan: QueryPlan {
                    entrypoint: QueryNode::CartesianProductJoin {
                        left: Box::new(QueryNode::TriplePattern {
                            subject: PatternValue::Variable(0),
                            predicate: PatternValue::Constant(state::Node {
                                namespace: 1,
                                value: "type".to_string(),
                            }),
                            object: PatternValue::Constant(Object::Named(state::Node {
                                namespace: 2,
                                value: "NamedIndividual".to_string(),
                            })),
                        }),
                        right: Box::new(QueryNode::TriplePattern {
                            subject: PatternValue::Variable(1),
                            predicate: PatternValue::Constant(state::Node {
                                namespace: 3,
                                value: "hasPublisher".to_string(),
                            }),
                            object: PatternValue::Constant(Object::Literal(Literal::Simple {
                                value: "OKP4".to_string(),
                            })),
                        }),
                    },
                    variables: vec!["v1".to_string(), "v2".to_string()],
                },
                expects: 10,
            },
            TestCase {
                plan: QueryPlan {
                    entrypoint: QueryNode::ForLoopJoin {
                        left: Box::new(QueryNode::TriplePattern {
                            subject: PatternValue::Variable(0),
                            predicate: PatternValue::Constant(state::Node {
                                namespace: 1,
                                value: "type".to_string(),
                            }),
                            object: PatternValue::Constant(Object::Named(state::Node {
                                namespace: 2,
                                value: "NamedIndividual".to_string(),
                            })),
                        }),
                        right: Box::new(QueryNode::TriplePattern {
                            subject: PatternValue::Variable(0),
                            predicate: PatternValue::Constant(state::Node {
                                namespace: 3,
                                value: "hasTag".to_string(),
                            }),
                            object: PatternValue::Variable(1),
                        }),
                    },
                    variables: vec!["v1".to_string(), "v2".to_string()],
                },
                expects: 3,
            },
        ];

        let engine = QueryEngine::new(&deps.storage);
        for case in cases {
            assert_eq!(engine.eval_plan(case.plan).count(), case.expects);
        }
    }

    #[test]
    fn for_loop_join_iter() {
        struct TestCase {
            left: Vec<String>,
            right: Vec<String>,
            expects: Vec<(String, String)>,
        }

        let cases = vec![
            TestCase {
                left: vec![],
                right: vec!["1".to_string(), "2".to_string()],
                expects: vec![],
            },
            TestCase {
                left: vec!["A".to_string()],
                right: vec!["1".to_string(), "2".to_string()],
                expects: vec![
                    ("A".to_string(), "1".to_string()),
                    ("A".to_string(), "2".to_string()),
                ],
            },
            TestCase {
                left: vec!["A".to_string(), "B".to_string()],
                right: vec!["1".to_string(), "2".to_string()],
                expects: vec![
                    ("A".to_string(), "1".to_string()),
                    ("A".to_string(), "2".to_string()),
                    ("B".to_string(), "1".to_string()),
                    ("B".to_string(), "2".to_string()),
                ],
            },
        ];

        for case in cases {
            let result = ForLoopJoinIterator::new(
                Box::new(case.left.iter().map(|v| {
                    let mut vars = ResolvedVariables::with_capacity(3);
                    vars.set(1, ResolvedVariable::Subject(Subject::Blank(v.clone())));
                    Ok(vars)
                })),
                Rc::new(|input| {
                    Box::new(case.right.iter().map(move |v| {
                        let mut vars = input.clone();
                        vars.set(2, ResolvedVariable::Subject(Subject::Blank(v.clone())));
                        Ok(vars)
                    }))
                }),
            )
            .collect::<StdResult<Vec<ResolvedVariables>>>();
            assert!(result.is_ok());

            let expects: Vec<ResolvedVariables> = case
                .expects
                .iter()
                .map(|(v1, v2)| {
                    let mut vars = ResolvedVariables::with_capacity(3);
                    vars.set(1, ResolvedVariable::Subject(Subject::Blank(v1.clone())));
                    vars.set(2, ResolvedVariable::Subject(Subject::Blank(v2.clone())));
                    vars
                })
                .collect();

            assert_eq!(result.unwrap(), expects);
        }
    }

    #[test]
    fn cartesian_join_iter() {
        struct TestCase {
            left: Vec<String>,
            right: Vec<String>,
            expects: Vec<Vec<String>>,
        }

        let cases = vec![
            TestCase {
                left: vec![],
                right: vec!["1".to_string(), "2".to_string()],
                expects: vec![],
            },
            TestCase {
                left: vec!["1".to_string(), "2".to_string()],
                right: vec![],
                expects: vec![],
            },
            TestCase {
                left: vec!["A".to_string()],
                right: vec!["1".to_string(), "2".to_string()],
                expects: vec![
                    vec!["1".to_string(), "A".to_string()],
                    vec!["2".to_string(), "A".to_string()],
                ],
            },
            TestCase {
                left: vec!["A".to_string(), "B".to_string()],
                right: vec!["1".to_string(), "2".to_string()],
                expects: vec![
                    vec!["1".to_string(), "A".to_string()],
                    vec!["2".to_string(), "A".to_string()],
                    vec!["1".to_string(), "B".to_string()],
                    vec!["2".to_string(), "B".to_string()],
                ],
            },
        ];

        for case in cases {
            let result = CartesianProductJoinIterator::new(
                case.right
                    .iter()
                    .map(|v| {
                        let mut vars = ResolvedVariables::with_capacity(2);
                        vars.set(0, ResolvedVariable::Subject(Subject::Blank(v.clone())));
                        vars
                    })
                    .collect(),
                Box::new(case.left.iter().map(|v| {
                    let mut vars = ResolvedVariables::with_capacity(2);
                    vars.set(1, ResolvedVariable::Subject(Subject::Blank(v.clone())));
                    Ok(vars)
                })),
                VecDeque::new(),
            )
            .collect::<StdResult<Vec<ResolvedVariables>>>();
            assert!(result.is_ok());

            let expects: Vec<ResolvedVariables> = case
                .expects
                .iter()
                .map(|v| {
                    let mut vars = ResolvedVariables::with_capacity(2);
                    if let Some(val) = v.get(0) {
                        vars.set(0, ResolvedVariable::Subject(Subject::Blank(val.clone())));
                    }
                    if let Some(val) = v.get(1) {
                        vars.set(1, ResolvedVariable::Subject(Subject::Blank(val.clone())));
                    }
                    vars
                })
                .collect();

            assert_eq!(result.unwrap(), expects);
        }
    }

    #[test]
    fn triple_pattern_iter_compute_io() {
        let t_subject = Subject::Blank("s".to_string());
        let t_predicate = state::Node {
            namespace: 0u128,
            value: "whatever".to_string(),
        };
        let t_object = Object::Blank("o".to_string());

        let mut variables = ResolvedVariables::with_capacity(6);
        variables.set(1, ResolvedVariable::Subject(t_subject.clone()));
        variables.set(2, ResolvedVariable::Predicate(t_predicate.clone()));
        variables.set(3, ResolvedVariable::Object(t_object.clone()));

        struct TestCase {
            subject: PatternValue<Subject>,
            predicate: PatternValue<Predicate>,
            object: PatternValue<Object>,
            expects: Option<(TriplePatternFilters, TriplePatternBindings)>,
        }
        let cases = vec![
            TestCase {
                subject: PatternValue::Variable(0),
                predicate: PatternValue::Variable(4),
                object: PatternValue::Variable(5),
                expects: Some(((None, None, None), (Some(0), Some(4), Some(5)))),
            },
            TestCase {
                subject: PatternValue::Variable(1),
                predicate: PatternValue::Variable(4),
                object: PatternValue::Variable(5),
                expects: Some((
                    (Some(t_subject.clone()), None, None),
                    (None, Some(4), Some(5)),
                )),
            },
            TestCase {
                subject: PatternValue::Variable(1),
                predicate: PatternValue::Variable(2),
                object: PatternValue::Variable(5),
                expects: Some((
                    (Some(t_subject.clone()), Some(t_predicate.clone()), None),
                    (None, None, Some(5)),
                )),
            },
            TestCase {
                subject: PatternValue::Variable(1),
                predicate: PatternValue::Variable(2),
                object: PatternValue::Variable(3),
                expects: Some((
                    (Some(t_subject), Some(t_predicate), Some(t_object)),
                    (None, None, None),
                )),
            },
            TestCase {
                subject: PatternValue::Variable(3),
                predicate: PatternValue::Variable(4),
                object: PatternValue::Variable(5),
                expects: Some((
                    (Some(Subject::Blank("o".to_string())), None, None),
                    (None, Some(4), Some(5)),
                )),
            },
            TestCase {
                subject: PatternValue::Variable(3),
                predicate: PatternValue::Variable(1),
                object: PatternValue::Variable(5),
                expects: None,
            },
        ];

        for case in cases {
            assert_eq!(
                TriplePatternIterator::compute_iter_io(
                    &variables,
                    case.subject,
                    case.predicate,
                    case.object
                ),
                case.expects
            );
        }
    }

    #[test]
    fn triple_pattern_iter_make_state_iter() {
        let mut deps = mock_dependencies();
        fill_test_data(deps.as_mut().storage);

        struct TestCase {
            filters: TriplePatternFilters,
            expects: usize,
        }
        let cases = vec![
            TestCase {
                filters: (None, None, None),
                expects: 40,
            },
            TestCase {
                filters: (
                    Some(Subject::Named(state::Node {
                        namespace: 0u128,
                        value: "97ff7e16-c08d-47be-8475-211016c82e33".to_string(),
                    })),
                    None,
                    None,
                ),
                expects: 3,
            },
            TestCase {
                filters: (
                    None,
                    Some(state::Node {
                        namespace: 1u128,
                        value: "type".to_string(),
                    }),
                    None,
                ),
                expects: 10,
            },
            TestCase {
                filters: (
                    None,
                    None,
                    Some(Object::Named(state::Node {
                        namespace: 0u128,
                        value: "97ff7e16-c08d-47be-8475-211016c82e33".to_string(),
                    })),
                ),
                expects: 2,
            },
            TestCase {
                filters: (
                    Some(Subject::Named(state::Node {
                        namespace: 0u128,
                        value: "97ff7e16-c08d-47be-8475-211016c82e33".to_string(),
                    })),
                    Some(state::Node {
                        namespace: 1u128,
                        value: "type".to_string(),
                    }),
                    None,
                ),
                expects: 2,
            },
            TestCase {
                filters: (
                    None,
                    Some(state::Node {
                        namespace: 1u128,
                        value: "type".to_string(),
                    }),
                    Some(Object::Named(state::Node {
                        namespace: 2u128,
                        value: "NamedIndividual".to_string(),
                    })),
                ),
                expects: 5,
            },
            TestCase {
                filters: (
                    Some(Subject::Named(state::Node {
                        namespace: 0u128,
                        value: "97ff7e16-c08d-47be-8475-211016c82e33".to_string(),
                    })),
                    Some(state::Node {
                        namespace: 1u128,
                        value: "type".to_string(),
                    }),
                    Some(Object::Named(state::Node {
                        namespace: 2u128,
                        value: "NamedIndividual".to_string(),
                    })),
                ),
                expects: 1,
            },
            TestCase {
                filters: (
                    Some(Subject::Named(state::Node {
                        namespace: 0u128,
                        value: "not-existing".to_string(),
                    })),
                    Some(state::Node {
                        namespace: 1u128,
                        value: "type".to_string(),
                    }),
                    Some(Object::Named(state::Node {
                        namespace: 2u128,
                        value: "NamedIndividual".to_string(),
                    })),
                ),
                expects: 0,
            },
        ];

        for case in cases {
            assert_eq!(
                TriplePatternIterator::make_state_iter(&deps.storage, case.filters).count(),
                case.expects
            );
        }
    }
}
