use crate::msg::{
    SimpleWhereCondition, TriplePattern, VarOrNode, VarOrNodeOrLiteral, WhereClause, WhereCondition,
};
use crate::querier::mapper::{
    literal_as_object, node_as_object, node_as_predicate, node_as_subject,
};
use crate::querier::plan::{PatternValue, QueryNode, QueryPlan};
use crate::state::{HasCachedNamespaces, Namespace, NamespaceResolver, Object, Predicate, Subject};
use cosmwasm_std::{StdError, StdResult, Storage};
use std::collections::HashMap;

pub struct PlanBuilder<'a> {
    storage: &'a dyn Storage,
    ns_resolver: NamespaceResolver,
    prefixes: &'a HashMap<String, String>,
    variables: Vec<String>,
    limit: Option<usize>,
    skip: Option<usize>,
}

impl<'a> PlanBuilder<'a> {
    pub fn new(
        storage: &'a dyn Storage,
        prefixes: &'a HashMap<String, String>,
        ns_cache: Option<Vec<Namespace>>,
    ) -> Self {
        Self {
            storage,
            ns_resolver: ns_cache.map_or_else(NamespaceResolver::new, Into::into),
            prefixes,
            variables: Vec::new(),
            skip: None,
            limit: None,
        }
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    #[allow(dead_code)]
    pub fn with_skip(mut self, skip: usize) -> Self {
        self.skip = Some(skip);
        self
    }

    pub fn build_plan(&mut self, where_clause: &WhereClause) -> StdResult<QueryPlan> {
        let bgp: Vec<QueryNode> = where_clause
            .iter()
            .map(|cond| {
                let WhereCondition::Simple(SimpleWhereCondition::TriplePattern(pattern)) = cond;
                self.build_triple_pattern(pattern)
            })
            .collect::<StdResult<Vec<QueryNode>>>()?;

        Self::build_from_bgp(bgp)
            .map(|mut node| {
                if let Some(skip) = self.skip {
                    node = QueryNode::Skip {
                        child: Box::new(node),
                        first: skip,
                    }
                }
                node
            })
            .map(|mut node| {
                if let Some(limit) = self.limit {
                    node = QueryNode::Limit {
                        child: Box::new(node),
                        first: limit,
                    }
                }
                node
            })
            .map(|node| QueryPlan {
                entrypoint: node,
                variables: self.variables.clone(),
            })
    }

    fn build_from_bgp(bgp: Vec<QueryNode>) -> StdResult<QueryNode> {
        bgp.into_iter()
            .reduce(|left: QueryNode, right: QueryNode| -> QueryNode {
                if left
                    .bound_variables()
                    .intersection(&right.bound_variables())
                    .next()
                    .is_some()
                {
                    return QueryNode::ForLoopJoin {
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                QueryNode::CartesianProductJoin {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            })
            .map_or_else(
                || Err(StdError::generic_err("Empty basic graph pattern")),
                Ok,
            )
    }

    fn build_triple_pattern(&mut self, pattern: &TriplePattern) -> StdResult<QueryNode> {
        Ok(QueryNode::TriplePattern {
            subject: self.build_subject_pattern(pattern.subject.clone())?,
            predicate: self.build_predicate_pattern(pattern.predicate.clone())?,
            object: self.build_object_pattern(pattern.object.clone())?,
        })
    }

    fn build_subject_pattern(&mut self, value: VarOrNode) -> StdResult<PatternValue<Subject>> {
        Ok(match value {
            VarOrNode::Variable(v) => PatternValue::Variable(self.resolve_variable(v)),
            VarOrNode::Node(n) => PatternValue::Constant(node_as_subject(
                &mut self.ns_resolver,
                self.storage,
                self.prefixes,
                n,
            )?),
        })
    }

    fn build_predicate_pattern(&mut self, value: VarOrNode) -> StdResult<PatternValue<Predicate>> {
        Ok(match value {
            VarOrNode::Variable(v) => PatternValue::Variable(self.resolve_variable(v)),
            VarOrNode::Node(n) => PatternValue::Constant(node_as_predicate(
                &mut self.ns_resolver,
                self.storage,
                self.prefixes,
                n,
            )?),
        })
    }

    fn build_object_pattern(
        &mut self,
        value: VarOrNodeOrLiteral,
    ) -> StdResult<PatternValue<Object>> {
        Ok(match value {
            VarOrNodeOrLiteral::Variable(v) => PatternValue::Variable(self.resolve_variable(v)),
            VarOrNodeOrLiteral::Node(n) => PatternValue::Constant(node_as_object(
                &mut self.ns_resolver,
                self.storage,
                self.prefixes,
                n,
            )?),
            VarOrNodeOrLiteral::Literal(l) => PatternValue::Constant(literal_as_object(
                &mut self.ns_resolver,
                self.storage,
                self.prefixes,
                l,
            )?),
        })
    }

    fn resolve_variable(&mut self, v: String) -> usize {
        if let Some(index) = self.variables.iter().position(|name| name == &v) {
            return index;
        }

        self.variables.push(v);
        self.variables.len() - 1
    }
}

impl<'a> HasCachedNamespaces for PlanBuilder<'a> {
    fn cached_namespaces(&self) -> Vec<Namespace> {
        self.ns_resolver.cached_namespaces()
    }

    fn clear_cache(&mut self) {
        self.ns_resolver.clear_cache();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::msg::{Literal, Node, Prefix, IRI};
    use crate::rdf::PrefixMap;
    use crate::state;
    use crate::state::{namespaces, Namespace};
    use cosmwasm_std::testing::mock_dependencies;

    #[test]
    fn proper_initialization() {
        let cases = vec![
            (vec![], HashMap::new()),
            (
                vec![
                    Prefix {
                        prefix: "owl".to_string(),
                        namespace: "http://www.w3.org/2002/07/owl#".to_string(),
                    },
                    Prefix {
                        prefix: "rdf".to_string(),
                        namespace: "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
                    },
                ],
                HashMap::from([
                    (
                        "owl".to_string(),
                        "http://www.w3.org/2002/07/owl#".to_string(),
                    ),
                    (
                        "rdf".to_string(),
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
                    ),
                ]),
            ),
            (
                vec![
                    Prefix {
                        prefix: "owl".to_string(),
                        namespace: "http://www.w3.org/2002/07/owl-will-be-overwritten#".to_string(),
                    },
                    Prefix {
                        prefix: "owl".to_string(),
                        namespace: "http://www.w3.org/2002/07/owl#".to_string(),
                    },
                    Prefix {
                        prefix: "rdf".to_string(),
                        namespace: "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
                    },
                ],
                HashMap::from([
                    (
                        "owl".to_string(),
                        "http://www.w3.org/2002/07/owl#".to_string(),
                    ),
                    (
                        "rdf".to_string(),
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
                    ),
                ]),
            ),
        ];
        let deps = mock_dependencies();

        for case in cases {
            let prefixes = &PrefixMap::from(case.0).into_inner();
            let builder = PlanBuilder::new(&deps.storage, prefixes, None);
            assert_eq!(builder.skip, None);
            assert_eq!(builder.limit, None);
            assert_eq!(builder.variables, Vec::<String>::new());
            assert_eq!(builder.prefixes, &case.1);
        }

        let prefixes = &PrefixMap::default().into_inner();
        let mut builder = PlanBuilder::new(&deps.storage, prefixes, None);
        builder = builder.with_skip(20usize).with_limit(50usize);
        assert_eq!(builder.skip, Some(20usize));
        assert_eq!(builder.limit, Some(50usize));

        builder = builder.with_skip(100usize).with_limit(5usize);
        assert_eq!(builder.skip, Some(100usize));
        assert_eq!(builder.limit, Some(5usize));
    }

    #[test]
    fn build_triple_pattern() {
        let cases = vec![
            (
                TriplePattern {
                    subject: VarOrNode::Variable("s".to_string()),
                    predicate: VarOrNode::Variable("p".to_string()),
                    object: VarOrNodeOrLiteral::Variable("o".to_string()),
                },
                Ok(QueryNode::TriplePattern {
                    subject: PatternValue::Variable(0usize),
                    predicate: PatternValue::Variable(1usize),
                    object: PatternValue::Variable(2usize),
                }),
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Node(Node::BlankNode("_".to_string())),
                    predicate: VarOrNode::Node(Node::NamedNode(IRI::Full(
                        "http://okp4.space/hasTitle".to_string(),
                    ))),
                    object: VarOrNodeOrLiteral::Node(Node::BlankNode("_".to_string())),
                },
                Ok(QueryNode::TriplePattern {
                    subject: PatternValue::Constant(Subject::Blank("_".to_string())),
                    predicate: PatternValue::Constant(state::Node {
                        namespace: 0u128,
                        value: "hasTitle".to_string(),
                    }),
                    object: PatternValue::Constant(Object::Blank("_".to_string())),
                }),
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Node(Node::NamedNode(IRI::Full(
                        "http://okp4.space/123456789".to_string(),
                    ))),
                    predicate: VarOrNode::Variable("p".to_string()),
                    object: VarOrNodeOrLiteral::Node(Node::NamedNode(IRI::Full(
                        "http://okp4.space/1234567892".to_string(),
                    ))),
                },
                Ok(QueryNode::TriplePattern {
                    subject: PatternValue::Constant(Subject::Named(state::Node {
                        namespace: 0u128,
                        value: "123456789".to_string(),
                    })),
                    predicate: PatternValue::Variable(1usize),
                    object: PatternValue::Constant(Object::Named(state::Node {
                        namespace: 0u128,
                        value: "1234567892".to_string(),
                    })),
                }),
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Variable("p".to_string()),
                    predicate: VarOrNode::Variable("s".to_string()),
                    object: VarOrNodeOrLiteral::Literal(Literal::Simple("simple".to_string())),
                },
                Ok(QueryNode::TriplePattern {
                    subject: PatternValue::Variable(1usize),
                    predicate: PatternValue::Variable(0usize),
                    object: PatternValue::Constant(Object::Literal(state::Literal::Simple {
                        value: "simple".to_string(),
                    })),
                }),
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Variable("s".to_string()),
                    predicate: VarOrNode::Variable("p".to_string()),
                    object: VarOrNodeOrLiteral::Literal(Literal::LanguageTaggedString {
                        value: "tagged".to_string(),
                        language: "en".to_string(),
                    }),
                },
                Ok(QueryNode::TriplePattern {
                    subject: PatternValue::Variable(0usize),
                    predicate: PatternValue::Variable(1usize),
                    object: PatternValue::Constant(Object::Literal(state::Literal::I18NString {
                        value: "tagged".to_string(),
                        language: "en".to_string(),
                    })),
                }),
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Variable("s".to_string()),
                    predicate: VarOrNode::Variable("p".to_string()),
                    object: VarOrNodeOrLiteral::Literal(Literal::TypedValue {
                        value: "typed".to_string(),
                        datatype: IRI::Full("http://okp4.space/type".to_string()),
                    }),
                },
                Ok(QueryNode::TriplePattern {
                    subject: PatternValue::Variable(0usize),
                    predicate: PatternValue::Variable(1usize),
                    object: PatternValue::Constant(Object::Literal(state::Literal::Typed {
                        value: "typed".to_string(),
                        datatype: state::Node {
                            namespace: 0u128,
                            value: "type".to_string(),
                        },
                    })),
                }),
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Node(Node::NamedNode(IRI::Full(
                        "notexisting#outch".to_string(),
                    ))),
                    predicate: VarOrNode::Variable("p".to_string()),
                    object: VarOrNodeOrLiteral::Variable("o".to_string()),
                },
                Err(StdError::not_found("Namespace")),
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Variable("s".to_string()),
                    predicate: VarOrNode::Node(Node::NamedNode(IRI::Full(
                        "notexisting#outch".to_string(),
                    ))),
                    object: VarOrNodeOrLiteral::Variable("o".to_string()),
                },
                Err(StdError::not_found("Namespace")),
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Variable("s".to_string()),
                    predicate: VarOrNode::Variable("p".to_string()),
                    object: VarOrNodeOrLiteral::Node(Node::NamedNode(IRI::Full(
                        "notexisting#outch".to_string(),
                    ))),
                },
                Err(StdError::not_found("Namespace")),
            ),
        ];

        let mut deps = mock_dependencies();
        namespaces()
            .save(
                deps.as_mut().storage,
                "http://okp4.space/".to_string(),
                &Namespace {
                    value: "http://okp4.space/".to_string(),
                    key: 0u128,
                    counter: 1u128,
                },
            )
            .unwrap();
        let prefixes = &PrefixMap::default().into_inner();
        let mut builder = PlanBuilder::new(&deps.storage, prefixes, None);

        for case in cases {
            assert_eq!(builder.build_triple_pattern(&case.0), case.1);
        }
    }

    #[test]
    fn build_plan() {
        let cases = vec![
            (
                None,
                None,
                vec![],
                Err(StdError::generic_err("Empty basic graph pattern")),
            ),
            (
                None,
                None,
                vec![TriplePattern {
                    subject: VarOrNode::Variable("subject".to_string()),
                    predicate: VarOrNode::Node(Node::BlankNode("_".to_string())),
                    object: VarOrNodeOrLiteral::Variable("object".to_string()),
                }],
                Err(StdError::generic_err(
                    "Predicate pattern must be a named node",
                )),
            ),
            (
                None,
                None,
                vec![TriplePattern {
                    subject: VarOrNode::Node(Node::NamedNode(IRI::Full(
                        "notexisting#outch".to_string(),
                    ))),
                    predicate: VarOrNode::Variable("predicate".to_string()),
                    object: VarOrNodeOrLiteral::Variable("object".to_string()),
                }],
                Err(StdError::not_found("Namespace")),
            ),
            (
                None,
                None,
                vec![TriplePattern {
                    subject: VarOrNode::Variable("subject".to_string()),
                    predicate: VarOrNode::Variable("predicate".to_string()),
                    object: VarOrNodeOrLiteral::Variable("object".to_string()),
                }],
                Ok(QueryPlan {
                    entrypoint: QueryNode::TriplePattern {
                        subject: PatternValue::Variable(0usize),
                        predicate: PatternValue::Variable(1usize),
                        object: PatternValue::Variable(2usize),
                    },
                    variables: vec![
                        "subject".to_string(),
                        "predicate".to_string(),
                        "object".to_string(),
                    ],
                }),
            ),
            (
                Some(20usize),
                None,
                vec![TriplePattern {
                    subject: VarOrNode::Variable("subject".to_string()),
                    predicate: VarOrNode::Variable("predicate".to_string()),
                    object: VarOrNodeOrLiteral::Variable("object".to_string()),
                }],
                Ok(QueryPlan {
                    entrypoint: QueryNode::Skip {
                        first: 20usize,
                        child: Box::new(QueryNode::TriplePattern {
                            subject: PatternValue::Variable(0usize),
                            predicate: PatternValue::Variable(1usize),
                            object: PatternValue::Variable(2usize),
                        }),
                    },
                    variables: vec![
                        "subject".to_string(),
                        "predicate".to_string(),
                        "object".to_string(),
                    ],
                }),
            ),
            (
                None,
                Some(20usize),
                vec![TriplePattern {
                    subject: VarOrNode::Variable("subject".to_string()),
                    predicate: VarOrNode::Variable("predicate".to_string()),
                    object: VarOrNodeOrLiteral::Variable("object".to_string()),
                }],
                Ok(QueryPlan {
                    entrypoint: QueryNode::Limit {
                        first: 20usize,
                        child: Box::new(QueryNode::TriplePattern {
                            subject: PatternValue::Variable(0usize),
                            predicate: PatternValue::Variable(1usize),
                            object: PatternValue::Variable(2usize),
                        }),
                    },
                    variables: vec![
                        "subject".to_string(),
                        "predicate".to_string(),
                        "object".to_string(),
                    ],
                }),
            ),
            (
                Some(20usize),
                Some(50usize),
                vec![TriplePattern {
                    subject: VarOrNode::Variable("subject".to_string()),
                    predicate: VarOrNode::Variable("predicate".to_string()),
                    object: VarOrNodeOrLiteral::Variable("object".to_string()),
                }],
                Ok(QueryPlan {
                    entrypoint: QueryNode::Limit {
                        first: 50usize,
                        child: Box::new(QueryNode::Skip {
                            first: 20usize,
                            child: Box::new(QueryNode::TriplePattern {
                                subject: PatternValue::Variable(0usize),
                                predicate: PatternValue::Variable(1usize),
                                object: PatternValue::Variable(2usize),
                            }),
                        }),
                    },
                    variables: vec![
                        "subject".to_string(),
                        "predicate".to_string(),
                        "object".to_string(),
                    ],
                }),
            ),
            (
                None,
                None,
                vec![
                    TriplePattern {
                        subject: VarOrNode::Variable("var1".to_string()),
                        predicate: VarOrNode::Variable("var2".to_string()),
                        object: VarOrNodeOrLiteral::Variable("var3".to_string()),
                    },
                    TriplePattern {
                        subject: VarOrNode::Variable("var4".to_string()),
                        predicate: VarOrNode::Variable("var5".to_string()),
                        object: VarOrNodeOrLiteral::Variable("var6".to_string()),
                    },
                    TriplePattern {
                        subject: VarOrNode::Variable("var1".to_string()),
                        predicate: VarOrNode::Variable("var5".to_string()),
                        object: VarOrNodeOrLiteral::Node(Node::BlankNode("blank".to_string())),
                    },
                ],
                Ok(QueryPlan {
                    entrypoint: QueryNode::ForLoopJoin {
                        left: Box::new(QueryNode::CartesianProductJoin {
                            left: Box::new(QueryNode::TriplePattern {
                                subject: PatternValue::Variable(0usize),
                                predicate: PatternValue::Variable(1usize),
                                object: PatternValue::Variable(2usize),
                            }),
                            right: Box::new(QueryNode::TriplePattern {
                                subject: PatternValue::Variable(3usize),
                                predicate: PatternValue::Variable(4usize),
                                object: PatternValue::Variable(5usize),
                            }),
                        }),
                        right: Box::new(QueryNode::TriplePattern {
                            subject: PatternValue::Variable(0usize),
                            predicate: PatternValue::Variable(4usize),
                            object: PatternValue::Constant(Object::Blank("blank".to_string())),
                        }),
                    },
                    variables: vec![
                        "var1".to_string(),
                        "var2".to_string(),
                        "var3".to_string(),
                        "var4".to_string(),
                        "var5".to_string(),
                        "var6".to_string(),
                    ],
                }),
            ),
        ];

        let mut deps = mock_dependencies();
        namespaces()
            .save(
                deps.as_mut().storage,
                "http://okp4.space/".to_string(),
                &Namespace {
                    value: "http://okp4.space/".to_string(),
                    key: 0u128,
                    counter: 1u128,
                },
            )
            .unwrap();

        for case in cases {
            let prefixes = &PrefixMap::default().into_inner();
            let mut builder = PlanBuilder::new(&deps.storage, prefixes, None);
            if let Some(skip) = case.0 {
                builder = builder.with_skip(skip);
            }
            if let Some(limit) = case.1 {
                builder = builder.with_limit(limit);
            }

            assert_eq!(
                builder.build_plan(
                    &case
                        .2
                        .into_iter()
                        .map(SimpleWhereCondition::TriplePattern)
                        .map(WhereCondition::Simple)
                        .collect()
                ),
                case.3
            )
        }
    }
}
