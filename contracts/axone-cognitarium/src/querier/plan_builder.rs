use crate::msg;
use crate::msg::{Node, TriplePattern, VarOrNamedNode, VarOrNode, VarOrNodeOrLiteral, WhereClause};
use crate::querier::expression::{Expression, Term};
use crate::querier::mapper::{iri_as_node, literal_as_object};
use crate::querier::plan::{PatternValue, PlanVariable, QueryNode, QueryPlan};
use crate::querier::variable::HasBoundVariables;
use crate::state::{
    HasCachedNamespaces, Namespace, NamespaceQuerier, NamespaceResolver, Object, Predicate, Subject,
};
use cosmwasm_std::{StdError, StdResult, Storage};
use std::collections::HashMap;

pub struct PlanBuilder<'a> {
    ns_resolver: NamespaceResolver<'a>,
    prefixes: &'a HashMap<String, String>,
    variables: Vec<PlanVariable>,
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
            ns_resolver: NamespaceResolver::new(storage, ns_cache.unwrap_or_default()),
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
        let mut node = self.build_node(where_clause)?;

        if let Some(skip) = self.skip {
            node = QueryNode::Skip {
                child: Box::new(node),
                first: skip,
            }
        }
        if let Some(limit) = self.limit {
            node = QueryNode::Limit {
                child: Box::new(node),
                first: limit,
            }
        }
        Ok(QueryPlan {
            entrypoint: node,
            variables: self.variables.clone(),
        })
    }

    fn build_node(&mut self, where_clause: &WhereClause) -> StdResult<QueryNode> {
        match where_clause {
            WhereClause::Bgp { patterns } => self.build_from_bgp(patterns.iter()),
            WhereClause::LateralJoin { left, right } => Ok(QueryNode::ForLoopJoin {
                left: Box::new(self.build_node(left)?),
                right: Box::new(self.build_node(right)?),
            }),
            WhereClause::Filter { expr, inner } => {
                let inner = Box::new(self.build_node(inner)?);
                let expr = self.build_expression(expr)?;

                if !expr.bound_variables().is_subset(&inner.bound_variables()) {
                    return Err(StdError::generic_err(
                        "Unbound variable in filter expression",
                    ));
                }

                Ok(QueryNode::Filter { expr, inner })
            }
        }
    }

    fn build_from_bgp<'b>(
        &mut self,
        bgp: impl Iterator<Item = &'b TriplePattern>,
    ) -> StdResult<QueryNode> {
        bgp.map(|pattern| self.build_triple_pattern(pattern))
            .reduce(|acc, item| {
                let acc = acc?;
                let item = item?;

                if acc
                    .bound_variables()
                    .intersection(&item.bound_variables())
                    .next()
                    .is_some()
                {
                    Ok(QueryNode::ForLoopJoin {
                        left: Box::new(acc),
                        right: Box::new(item),
                    })
                } else {
                    Ok(QueryNode::CartesianProductJoin {
                        left: Box::new(acc),
                        right: Box::new(item),
                    })
                }
            })
            .unwrap_or(Ok(QueryNode::noop()))
    }

    fn build_expression(&mut self, expr: &msg::Expression) -> StdResult<Expression> {
        match expr {
            msg::Expression::NamedNode(iri) => {
                Term::from_iri(iri.clone(), self.prefixes).map(Expression::Constant)
            }
            msg::Expression::Literal(literal) => {
                Term::from_literal(literal.clone(), self.prefixes).map(Expression::Constant)
            }
            msg::Expression::Variable(v) => Ok(Expression::Variable(
                self.resolve_basic_variable(v.to_string()),
            )),
            msg::Expression::And(exprs) => exprs
                .iter()
                .map(|e| self.build_expression(e))
                .collect::<StdResult<Vec<Expression>>>()
                .map(Expression::And),
            msg::Expression::Or(exprs) => exprs
                .iter()
                .map(|e| self.build_expression(e))
                .collect::<StdResult<Vec<Expression>>>()
                .map(Expression::Or),
            msg::Expression::Equal(left, right) => Ok(Expression::Equal(
                Box::new(self.build_expression(left)?),
                Box::new(self.build_expression(right)?),
            )),
            msg::Expression::Greater(left, right) => Ok(Expression::Greater(
                Box::new(self.build_expression(left)?),
                Box::new(self.build_expression(right)?),
            )),
            msg::Expression::GreaterOrEqual(left, right) => Ok(Expression::GreaterOrEqual(
                Box::new(self.build_expression(left)?),
                Box::new(self.build_expression(right)?),
            )),
            msg::Expression::Less(left, right) => Ok(Expression::Less(
                Box::new(self.build_expression(left)?),
                Box::new(self.build_expression(right)?),
            )),
            msg::Expression::LessOrEqual(left, right) => Ok(Expression::LessOrEqual(
                Box::new(self.build_expression(left)?),
                Box::new(self.build_expression(right)?),
            )),
            msg::Expression::Not(child) => self
                .build_expression(child)
                .map(Box::new)
                .map(Expression::Not),
        }
    }

    fn build_triple_pattern(&mut self, pattern: &TriplePattern) -> StdResult<QueryNode> {
        let subject_res = self.build_subject_pattern(pattern.subject.clone());
        let predicate_res = self.build_predicate_pattern(pattern.predicate.clone());
        let object_res = self.build_object_pattern(pattern.object.clone());

        let mut bound_variables: Vec<usize> = vec![];
        let maybe_subject =
            Self::recover_ns_not_found_pattern_res(subject_res, &mut bound_variables)?;
        let maybe_predicate =
            Self::recover_ns_not_found_pattern_res(predicate_res, &mut bound_variables)?;
        let maybe_object =
            Self::recover_ns_not_found_pattern_res(object_res, &mut bound_variables)?;

        Ok(match (maybe_subject, maybe_predicate, maybe_object) {
            (Some(subject), Some(predicate), Some(object)) => QueryNode::TriplePattern {
                subject,
                predicate,
                object,
            },
            _ => QueryNode::Noop { bound_variables },
        })
    }

    fn recover_ns_not_found_pattern_res<T>(
        pattern_res: StdResult<PatternValue<T>>,
        bound_variables: &mut Vec<usize>,
    ) -> StdResult<Option<PatternValue<T>>> {
        Ok(match pattern_res {
            Ok(value) => {
                value.lookup_bound_variable(&mut |v| bound_variables.push(v));
                Some(value)
            }
            Err(err) if NamespaceQuerier::is_ns_not_found_error(&err) => None,
            _ => Some(pattern_res?),
        })
    }

    fn build_subject_pattern(&mut self, value: VarOrNode) -> StdResult<PatternValue<Subject>> {
        Ok(match value {
            VarOrNode::Variable(v) => PatternValue::Variable(self.resolve_basic_variable(v)),
            VarOrNode::Node(Node::BlankNode(b)) => {
                PatternValue::BlankVariable(self.resolve_blank_variable(b))
            }
            VarOrNode::Node(Node::NamedNode(iri)) => PatternValue::Constant(Subject::Named(
                iri_as_node(&mut self.ns_resolver, self.prefixes, iri)?,
            )),
        })
    }

    fn build_predicate_pattern(
        &mut self,
        value: VarOrNamedNode,
    ) -> StdResult<PatternValue<Predicate>> {
        Ok(match value {
            VarOrNamedNode::Variable(v) => PatternValue::Variable(self.resolve_basic_variable(v)),
            VarOrNamedNode::NamedNode(iri) => {
                PatternValue::Constant(iri_as_node(&mut self.ns_resolver, self.prefixes, iri)?)
            }
        })
    }

    fn build_object_pattern(
        &mut self,
        value: VarOrNodeOrLiteral,
    ) -> StdResult<PatternValue<Object>> {
        Ok(match value {
            VarOrNodeOrLiteral::Variable(v) => {
                PatternValue::Variable(self.resolve_basic_variable(v))
            }
            VarOrNodeOrLiteral::Node(Node::BlankNode(b)) => {
                PatternValue::BlankVariable(self.resolve_blank_variable(b))
            }
            VarOrNodeOrLiteral::Node(Node::NamedNode(iri)) => PatternValue::Constant(
                Object::Named(iri_as_node(&mut self.ns_resolver, self.prefixes, iri)?),
            ),
            VarOrNodeOrLiteral::Literal(l) => {
                PatternValue::Constant(literal_as_object(&mut self.ns_resolver, self.prefixes, l)?)
            }
        })
    }

    fn resolve_basic_variable(&mut self, v: String) -> usize {
        if let Some(index) = self.variables.iter().position(|var| match var {
            PlanVariable::Basic(name) => name == &v,
            PlanVariable::BlankNode(_) => false,
        }) {
            return index;
        }

        self.variables.push(PlanVariable::Basic(v));
        self.variables.len() - 1
    }

    fn resolve_blank_variable(&mut self, v: String) -> usize {
        if let Some(index) = self.variables.iter().position(|var| match var {
            PlanVariable::BlankNode(name) => name == &v,
            PlanVariable::Basic(_) => false,
        }) {
            return index;
        }

        self.variables.push(PlanVariable::BlankNode(v));
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
            assert_eq!(builder.variables, Vec::<PlanVariable>::new());
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
                    predicate: VarOrNamedNode::Variable("p".to_string()),
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
                    subject: VarOrNode::Node(Node::BlankNode("1".to_string())),
                    predicate: VarOrNamedNode::NamedNode(IRI::Full(
                        "http://axone.space/hasTitle".to_string(),
                    )),
                    object: VarOrNodeOrLiteral::Node(Node::BlankNode("2".to_string())),
                },
                Ok(QueryNode::TriplePattern {
                    subject: PatternValue::BlankVariable(0usize),
                    predicate: PatternValue::Constant(state::Node {
                        namespace: 0u128,
                        value: "hasTitle".to_string(),
                    }),
                    object: PatternValue::BlankVariable(1usize),
                }),
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Node(Node::NamedNode(IRI::Full(
                        "http://axone.space/123456789".to_string(),
                    ))),
                    predicate: VarOrNamedNode::Variable("p".to_string()),
                    object: VarOrNodeOrLiteral::Node(Node::NamedNode(IRI::Full(
                        "http://axone.space/1234567892".to_string(),
                    ))),
                },
                Ok(QueryNode::TriplePattern {
                    subject: PatternValue::Constant(Subject::Named(state::Node {
                        namespace: 0u128,
                        value: "123456789".to_string(),
                    })),
                    predicate: PatternValue::Variable(0usize),
                    object: PatternValue::Constant(Object::Named(state::Node {
                        namespace: 0u128,
                        value: "1234567892".to_string(),
                    })),
                }),
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Variable("p".to_string()),
                    predicate: VarOrNamedNode::Variable("s".to_string()),
                    object: VarOrNodeOrLiteral::Literal(Literal::Simple("simple".to_string())),
                },
                Ok(QueryNode::TriplePattern {
                    subject: PatternValue::Variable(0usize),
                    predicate: PatternValue::Variable(1usize),
                    object: PatternValue::Constant(Object::Literal(state::Literal::Simple {
                        value: "simple".to_string(),
                    })),
                }),
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Variable("s".to_string()),
                    predicate: VarOrNamedNode::Variable("p".to_string()),
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
                    predicate: VarOrNamedNode::Variable("p".to_string()),
                    object: VarOrNodeOrLiteral::Literal(Literal::TypedValue {
                        value: "typed".to_string(),
                        datatype: IRI::Full("http://axone.space/type".to_string()),
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
                    predicate: VarOrNamedNode::Variable("p".to_string()),
                    object: VarOrNodeOrLiteral::Variable("o".to_string()),
                },
                Ok(QueryNode::Noop {
                    bound_variables: vec![0usize, 1usize],
                }),
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Variable("s".to_string()),
                    predicate: VarOrNamedNode::NamedNode(IRI::Full(
                        "notexisting#outch".to_string(),
                    )),
                    object: VarOrNodeOrLiteral::Variable("o".to_string()),
                },
                Ok(QueryNode::Noop {
                    bound_variables: vec![0usize, 1usize],
                }),
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Variable("s".to_string()),
                    predicate: VarOrNamedNode::Variable("p".to_string()),
                    object: VarOrNodeOrLiteral::Node(Node::NamedNode(IRI::Full(
                        "notexisting#outch".to_string(),
                    ))),
                },
                Ok(QueryNode::Noop {
                    bound_variables: vec![0usize, 1usize],
                }),
            ),
        ];

        let mut deps = mock_dependencies();

        namespaces()
            .save(
                deps.as_mut().storage,
                "http://axone.space/".to_string(),
                &Namespace {
                    value: "http://axone.space/".to_string(),
                    key: 0u128,
                    counter: 1u128,
                },
            )
            .unwrap();
        for case in cases {
            let prefixes = &PrefixMap::default().into_inner();
            let mut builder = PlanBuilder::new(&deps.storage, prefixes, None);

            assert_eq!(builder.build_triple_pattern(&case.0), case.1);
        }
    }

    #[test]
    fn build_bgp() {
        let cases = vec![
            (
                vec![],
                Ok(QueryNode::Noop {
                    bound_variables: vec![],
                }),
            ),
            (
                vec![TriplePattern {
                    subject: VarOrNode::Node(Node::NamedNode(IRI::Full(
                        "notexisting#outch".to_string(),
                    ))),
                    predicate: VarOrNamedNode::Variable("predicate".to_string()),
                    object: VarOrNodeOrLiteral::Variable("object".to_string()),
                }],
                Ok(QueryNode::Noop {
                    bound_variables: vec![0usize, 1usize],
                }),
            ),
            (
                vec![TriplePattern {
                    subject: VarOrNode::Variable("subject".to_string()),
                    predicate: VarOrNamedNode::Variable("predicate".to_string()),
                    object: VarOrNodeOrLiteral::Variable("object".to_string()),
                }],
                Ok(QueryNode::TriplePattern {
                    subject: PatternValue::Variable(0usize),
                    predicate: PatternValue::Variable(1usize),
                    object: PatternValue::Variable(2usize),
                }),
            ),
            (
                vec![TriplePattern {
                    subject: VarOrNode::Variable("subject".to_string()),
                    predicate: VarOrNamedNode::Variable("n".to_string()),
                    object: VarOrNodeOrLiteral::Variable("n".to_string()),
                }],
                Ok(QueryNode::TriplePattern {
                    subject: PatternValue::Variable(0usize),
                    predicate: PatternValue::Variable(1usize),
                    object: PatternValue::Variable(1usize),
                }),
            ),
            (
                vec![
                    TriplePattern {
                        subject: VarOrNode::Variable("var1".to_string()),
                        predicate: VarOrNamedNode::Variable("var2".to_string()),
                        object: VarOrNodeOrLiteral::Variable("var3".to_string()),
                    },
                    TriplePattern {
                        subject: VarOrNode::Variable("var4".to_string()),
                        predicate: VarOrNamedNode::Variable("var5".to_string()),
                        object: VarOrNodeOrLiteral::Variable("var6".to_string()),
                    },
                    TriplePattern {
                        subject: VarOrNode::Variable("var1".to_string()),
                        predicate: VarOrNamedNode::Variable("var5".to_string()),
                        object: VarOrNodeOrLiteral::Node(Node::BlankNode("blank".to_string())),
                    },
                ],
                Ok(QueryNode::ForLoopJoin {
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
                        object: PatternValue::BlankVariable(6usize),
                    }),
                }),
            ),
            (
                vec![
                    TriplePattern {
                        subject: VarOrNode::Node(Node::BlankNode("1".to_string())),
                        predicate: VarOrNamedNode::Variable("1".to_string()),
                        object: VarOrNodeOrLiteral::Node(Node::BlankNode("2".to_string())),
                    },
                    TriplePattern {
                        subject: VarOrNode::Node(Node::BlankNode("1".to_string())),
                        predicate: VarOrNamedNode::Variable("1".to_string()),
                        object: VarOrNodeOrLiteral::Variable("2".to_string()),
                    },
                ],
                Ok(QueryNode::ForLoopJoin {
                    left: Box::new(QueryNode::TriplePattern {
                        subject: PatternValue::BlankVariable(0usize),
                        predicate: PatternValue::Variable(1usize),
                        object: PatternValue::BlankVariable(2usize),
                    }),
                    right: Box::new(QueryNode::TriplePattern {
                        subject: PatternValue::BlankVariable(0usize),
                        predicate: PatternValue::Variable(1usize),
                        object: PatternValue::Variable(3usize),
                    }),
                }),
            ),
        ];

        let mut deps = mock_dependencies();
        namespaces()
            .save(
                deps.as_mut().storage,
                "http://axone.space/".to_string(),
                &Namespace {
                    value: "http://axone.space/".to_string(),
                    key: 0u128,
                    counter: 1u128,
                },
            )
            .unwrap();

        for case in cases {
            let prefixes = &PrefixMap::default().into_inner();
            let mut builder = PlanBuilder::new(&deps.storage, prefixes, None);

            assert_eq!(builder.build_from_bgp(case.0.iter()), case.1)
        }
    }

    #[test]
    fn build_expression() {
        let cases = vec![
            (
                msg::Expression::NamedNode(IRI::Full("http://axone.space/test".to_string())),
                Ok(Expression::Constant(Term::String(
                    "http://axone.space/test".to_string(),
                ))),
            ),
            (
                msg::Expression::NamedNode(IRI::Prefixed("oups:test".to_string())),
                Err(StdError::generic_err("Prefix not found: oups")),
            ),
            (
                msg::Expression::Literal(Literal::Simple("simple".to_string())),
                Ok(Expression::Constant(Term::String("simple".to_string()))),
            ),
            (
                msg::Expression::Literal(Literal::TypedValue {
                    value: "typed".to_string(),
                    datatype: IRI::Prefixed("oups:type".to_string()),
                }),
                Err(StdError::generic_err("Prefix not found: oups")),
            ),
            (
                msg::Expression::Variable("variable".to_string()),
                Ok(Expression::Variable(0usize)),
            ),
            (
                msg::Expression::And(vec![msg::Expression::Variable("variable".to_string())]),
                Ok(Expression::And(vec![Expression::Variable(0usize)])),
            ),
            (
                msg::Expression::Or(vec![msg::Expression::Variable("variable".to_string())]),
                Ok(Expression::Or(vec![Expression::Variable(0usize)])),
            ),
            (
                msg::Expression::Equal(
                    Box::new(msg::Expression::Variable("v1".to_string())),
                    Box::new(msg::Expression::Variable("v2".to_string())),
                ),
                Ok(Expression::Equal(
                    Box::new(Expression::Variable(0usize)),
                    Box::new(Expression::Variable(1usize)),
                )),
            ),
            (
                msg::Expression::Greater(
                    Box::new(msg::Expression::Variable("v1".to_string())),
                    Box::new(msg::Expression::Variable("v2".to_string())),
                ),
                Ok(Expression::Greater(
                    Box::new(Expression::Variable(0usize)),
                    Box::new(Expression::Variable(1usize)),
                )),
            ),
            (
                msg::Expression::GreaterOrEqual(
                    Box::new(msg::Expression::Variable("v1".to_string())),
                    Box::new(msg::Expression::Variable("v2".to_string())),
                ),
                Ok(Expression::GreaterOrEqual(
                    Box::new(Expression::Variable(0usize)),
                    Box::new(Expression::Variable(1usize)),
                )),
            ),
            (
                msg::Expression::Less(
                    Box::new(msg::Expression::Variable("v1".to_string())),
                    Box::new(msg::Expression::Variable("v2".to_string())),
                ),
                Ok(Expression::Less(
                    Box::new(Expression::Variable(0usize)),
                    Box::new(Expression::Variable(1usize)),
                )),
            ),
            (
                msg::Expression::LessOrEqual(
                    Box::new(msg::Expression::Variable("v1".to_string())),
                    Box::new(msg::Expression::Variable("v2".to_string())),
                ),
                Ok(Expression::LessOrEqual(
                    Box::new(Expression::Variable(0usize)),
                    Box::new(Expression::Variable(1usize)),
                )),
            ),
            (
                msg::Expression::Not(Box::new(msg::Expression::Variable("v1".to_string()))),
                Ok(Expression::Not(Box::new(Expression::Variable(0usize)))),
            ),
        ];

        let deps = mock_dependencies();

        for case in cases {
            let prefixes = &PrefixMap::default().into_inner();
            let mut builder = PlanBuilder::new(&deps.storage, prefixes, None);

            assert_eq!(builder.build_expression(&case.0), case.1)
        }
    }

    #[test]
    fn build_plan() {
        let cases = vec![
            (
                None,
                None,
                WhereClause::Bgp { patterns: vec![] },
                Ok(QueryPlan {
                    entrypoint: QueryNode::Noop {
                        bound_variables: vec![],
                    },
                    variables: vec![],
                }),
            ),
            (
                Some(10usize),
                None,
                WhereClause::Bgp { patterns: vec![] },
                Ok(QueryPlan {
                    entrypoint: QueryNode::Skip {
                        child: Box::new(QueryNode::Noop {
                            bound_variables: vec![],
                        }),
                        first: 10usize,
                    },
                    variables: vec![],
                }),
            ),
            (
                None,
                Some(10usize),
                WhereClause::Bgp { patterns: vec![] },
                Ok(QueryPlan {
                    entrypoint: QueryNode::Limit {
                        child: Box::new(QueryNode::Noop {
                            bound_variables: vec![],
                        }),
                        first: 10usize,
                    },
                    variables: vec![],
                }),
            ),
            (
                Some(10usize),
                Some(20usize),
                WhereClause::Bgp { patterns: vec![] },
                Ok(QueryPlan {
                    entrypoint: QueryNode::Limit {
                        child: Box::new(QueryNode::Skip {
                            child: Box::new(QueryNode::Noop {
                                bound_variables: vec![],
                            }),
                            first: 10usize,
                        }),
                        first: 20usize,
                    },
                    variables: vec![],
                }),
            ),
            (
                None,
                None,
                WhereClause::Bgp {
                    patterns: vec![TriplePattern {
                        subject: VarOrNode::Variable("subject".to_string()),
                        predicate: VarOrNamedNode::Variable("predicate".to_string()),
                        object: VarOrNodeOrLiteral::Variable("object".to_string()),
                    }],
                },
                Ok(QueryPlan {
                    entrypoint: QueryNode::TriplePattern {
                        subject: PatternValue::Variable(0usize),
                        predicate: PatternValue::Variable(1usize),
                        object: PatternValue::Variable(2usize),
                    },
                    variables: vec![
                        PlanVariable::Basic("subject".to_string()),
                        PlanVariable::Basic("predicate".to_string()),
                        PlanVariable::Basic("object".to_string()),
                    ],
                }),
            ),
            (
                None,
                None,
                WhereClause::Bgp {
                    patterns: vec![TriplePattern {
                        subject: VarOrNode::Variable("subject".to_string()),
                        predicate: VarOrNamedNode::Variable("n".to_string()),
                        object: VarOrNodeOrLiteral::Variable("n".to_string()),
                    }],
                },
                Ok(QueryPlan {
                    entrypoint: QueryNode::TriplePattern {
                        subject: PatternValue::Variable(0usize),
                        predicate: PatternValue::Variable(1usize),
                        object: PatternValue::Variable(1usize),
                    },
                    variables: vec![
                        PlanVariable::Basic("subject".to_string()),
                        PlanVariable::Basic("n".to_string()),
                    ],
                }),
            ),
            (
                None,
                None,
                WhereClause::LateralJoin {
                    left: Box::new(WhereClause::Bgp {
                        patterns: vec![TriplePattern {
                            subject: VarOrNode::Node(Node::BlankNode("1".to_string())),
                            predicate: VarOrNamedNode::Variable("n".to_string()),
                            object: VarOrNodeOrLiteral::Node(Node::BlankNode("2".to_string())),
                        }],
                    }),
                    right: Box::new(WhereClause::Bgp { patterns: vec![] }),
                },
                Ok(QueryPlan {
                    entrypoint: QueryNode::ForLoopJoin {
                        left: Box::new(QueryNode::TriplePattern {
                            subject: PatternValue::BlankVariable(0usize),
                            predicate: PatternValue::Variable(1usize),
                            object: PatternValue::BlankVariable(2usize),
                        }),
                        right: Box::new(QueryNode::Noop {
                            bound_variables: vec![],
                        }),
                    },
                    variables: vec![
                        PlanVariable::BlankNode("1".to_string()),
                        PlanVariable::Basic("n".to_string()),
                        PlanVariable::BlankNode("2".to_string()),
                    ],
                }),
            ),
            (
                None,
                None,
                WhereClause::Filter {
                    inner: Box::new(WhereClause::Bgp {
                        patterns: vec![TriplePattern {
                            subject: VarOrNode::Variable("1".to_string()),
                            predicate: VarOrNamedNode::Variable("2".to_string()),
                            object: VarOrNodeOrLiteral::Variable("2".to_string()),
                        }],
                    }),
                    expr: msg::Expression::Variable("1".to_string()),
                },
                Ok(QueryPlan {
                    entrypoint: QueryNode::Filter {
                        inner: Box::new(QueryNode::TriplePattern {
                            subject: PatternValue::Variable(0usize),
                            predicate: PatternValue::Variable(1usize),
                            object: PatternValue::Variable(1usize),
                        }),
                        expr: Expression::Variable(0usize),
                    },
                    variables: vec![
                        PlanVariable::Basic("1".to_string()),
                        PlanVariable::Basic("2".to_string()),
                    ],
                }),
            ),
            (
                None,
                None,
                WhereClause::Filter {
                    inner: Box::new(WhereClause::Bgp {
                        patterns: vec![TriplePattern {
                            subject: VarOrNode::Variable("1".to_string()),
                            predicate: VarOrNamedNode::Variable("2".to_string()),
                            object: VarOrNodeOrLiteral::Variable("2".to_string()),
                        }],
                    }),
                    expr: msg::Expression::Variable("oups".to_string()),
                },
                Err(StdError::generic_err(
                    "Unbound variable in filter expression",
                )),
            ),
        ];

        let mut deps = mock_dependencies();
        namespaces()
            .save(
                deps.as_mut().storage,
                "http://axone.space/".to_string(),
                &Namespace {
                    value: "http://axone.space/".to_string(),
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

            assert_eq!(builder.build_plan(&case.2), case.3)
        }
    }
}
