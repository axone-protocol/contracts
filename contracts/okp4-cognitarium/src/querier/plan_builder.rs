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
    limit: Option<usize>,
    skip: Option<usize>,
}

impl<'a> PlanBuilder<'a> {
    pub fn new(storage: &'a dyn Storage, prefixes: Vec<Prefix>) -> Self {
        Self {
            storage,
            prefixes: Self::make_prefixes(prefixes),
            variables: Vec::new(),
            skip: None,
            limit: None,
        }
    }

    pub fn with_limit(&mut self, limit: usize) -> &Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_skip(&mut self, skip: usize) -> &Self {
        self.skip = Some(skip);
        self
    }

    pub fn build_plan(&mut self, where_clause: WhereClause) -> StdResult<QueryPlan> {
        let bgp: Vec<QueryNode> = where_clause
            .iter()
            .map(|cond| {
                let WhereCondition::Simple(SimpleWhereCondition::TriplePattern(pattern)) = cond;
                self.build_triple_pattern(pattern)
            })
            .collect::<StdResult<Vec<QueryNode>>>()?;

        self.build_from_bgp(bgp)
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
                if let Some(limit) = self.skip {
                    node = QueryNode::Limit {
                        child: Box::new(node),
                        first: limit,
                    }
                }
                node
            })
            .map(|node| QueryPlan {
                entrypoint: Box::new(node),
                variables: self.variables.clone(),
            })
    }

    fn build_from_bgp(&self, bgp: Vec<QueryNode>) -> StdResult<QueryNode> {
        bgp.into_iter()
            .reduce(|left: QueryNode, right: QueryNode| -> QueryNode {
                if left
                    .bound_variables()
                    .union(&right.bound_variables())
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
            .map(Ok)
            .unwrap_or(Err(StdError::generic_err("Empty basic graph pattern")))
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
            VarOrNode::Node(n) => match n {
                Node::NamedNode(iri) => {
                    PatternValue::Constant(Subject::Named(self.build_named_node(iri)?))
                }
                Node::BlankNode(blank) => PatternValue::Constant(Subject::Blank(blank)),
            },
        })
    }

    fn build_predicate_pattern(&mut self, value: VarOrNode) -> StdResult<PatternValue<Predicate>> {
        Ok(match value {
            VarOrNode::Variable(v) => PatternValue::Variable(self.resolve_variable(v)),
            VarOrNode::Node(n) => match n {
                Node::NamedNode(iri) => PatternValue::Constant(self.build_named_node(iri)?),
                Node::BlankNode(_) => Err(StdError::generic_err(
                    "Predicate pattern must be a named node",
                ))?,
            },
        })
    }

    fn build_object_pattern(
        &mut self,
        value: VarOrNodeOrLiteral,
    ) -> StdResult<PatternValue<Object>> {
        Ok(match value {
            VarOrNodeOrLiteral::Variable(v) => PatternValue::Variable(self.resolve_variable(v)),
            VarOrNodeOrLiteral::Node(n) => match n {
                Node::NamedNode(iri) => {
                    PatternValue::Constant(Object::Named(self.build_named_node(iri)?))
                }
                Node::BlankNode(blank) => PatternValue::Constant(Object::Blank(blank)),
            },
            VarOrNodeOrLiteral::Literal(l) => PatternValue::Constant(Object::Literal(match l {
                Literal::Simple(value) => state::Literal::Simple { value },
                Literal::LanguageTaggedString { value, language } => {
                    state::Literal::I18NString { value, language }
                }
                Literal::TypedValue { value, datatype } => state::Literal::Typed {
                    value,
                    datatype: self.build_named_node(datatype)?,
                },
            })),
        })
    }

    fn build_named_node(&mut self, value: IRI) -> StdResult<state::Node> {
        match value {
            IRI::Prefixed(prefixed) => prefixed
                .rfind(':')
                .map(Ok)
                .unwrap_or(Err(StdError::generic_err(
                    "Malformed prefixed IRI: no prefix delimiter found",
                )))
                .and_then(|index| {
                    self.prefixes
                        .get(&prefixed.as_str()[..index + 1])
                        .map(|resolved_prefix| {
                            [resolved_prefix, &prefixed.as_str()[index + 1..]].join("")
                        })
                        .map(Ok)
                        .unwrap_or(Err(StdError::generic_err(
                            "Malformed prefixed IRI: prefix not found",
                        )))
                }),
            IRI::Full(full) => Ok(full),
        }
        .and_then(|iri| rdf::explode_iri(iri.as_str()))
        .and_then(|(ns_key, v)| {
            namespaces()
                .load(self.storage, ns_key)
                .map(|ns| state::Node {
                    namespace: ns.key,
                    value: v,
                })
        })
    }

    fn resolve_variable(&mut self, v: String) -> usize {
        if let Some(index) = self.variables.iter().position(|name| name == &v) {
            return index;
        }

        self.variables.push(v);
        self.variables.len() - 1
    }

    fn make_prefixes(as_list: Vec<Prefix>) -> HashMap<String, String> {
        as_list.iter().fold(HashMap::new(), |mut map, prefix| {
            map.insert(prefix.prefix.clone(), prefix.namespace.clone());
            map
        })
    }
}
