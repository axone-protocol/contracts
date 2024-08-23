use crate::querier::expression::Expression;
use crate::querier::variable::HasBoundVariables;
use crate::state::{Object, Predicate, Subject};

/// Represents a querying plan.
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct QueryPlan {
    /// References the ending node of the plan, when evaluated others nodes will be invoked in
    /// cascade.
    pub entrypoint: QueryNode,

    /// Contains all the query variables, their index in this array are internally used as
    /// identifiers.
    pub variables: Vec<PlanVariable>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum PlanVariable {
    Basic(String),
    BlankNode(String),
}

impl QueryPlan {
    pub fn empty_plan() -> Self {
        Self {
            entrypoint: QueryNode::noop(),
            variables: Vec::new(),
        }
    }

    /// Resolve the index corresponding to the variable name, if not attached to a blank node.
    pub fn get_var_index(&self, var_name: &str) -> Option<usize> {
        self.variables.iter().enumerate().find_map(|(index, it)| {
            matches!(it, PlanVariable::Basic(name) if name == var_name).then_some(index)
        })
    }

    /// Resolve the index corresponding to blank node name.
    pub fn get_bnode_index(&self, bnode_name: &str) -> Option<usize> {
        self.variables.iter().enumerate().find_map(|(index, it)| {
            matches!(it, PlanVariable::BlankNode(name) if name == bnode_name).then_some(index)
        })
    }
}

/// Represents a single part of the query plan processing. Each node is intended to provide a
/// specific behavior given an evaluation context.
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum QueryNode {
    /// Match the triple pattern against the state. The triple elements can be either a variable or
    /// a constant value, in the case of a variable it'll be either provided by the context of
    /// previous evaluation or calculated and present in output.
    TriplePattern {
        subject: PatternValue<Subject>,
        predicate: PatternValue<Predicate>,
        object: PatternValue<Object>,
    },

    /// Results in no solutions, this special node is used when we know before plan execution that a node
    /// will end up with no possible solutions. For example, using a triple pattern filtering with a constant
    /// named node containing a non-existing namespace.
    Noop { bound_variables: Vec<usize> },

    /// Join two nodes by applying the cartesian product of the nodes variables.
    ///
    /// This should be used when the nodes don't have variables in common, and can be seen as a
    /// full join of disjoint datasets.  
    CartesianProductJoin { left: Box<Self>, right: Box<Self> },

    /// Join two nodes by using the variables values from the left node as replacement in the right
    /// node.
    ///
    /// This results to an inner join, but the underlying processing stream the variables from the
    /// left node to use them as right node values.
    ForLoopJoin { left: Box<Self>, right: Box<Self> },

    /// Filter the results of the inner node by applying the expression.
    Filter { expr: Expression, inner: Box<Self> },

    /// Skip the specified first elements from the child node.
    Skip { child: Box<Self>, first: usize },

    /// Limit to the specified first elements from the child node.
    Limit { child: Box<Self>, first: usize },
}

impl QueryNode {
    pub fn noop() -> Self {
        QueryNode::Noop {
            bound_variables: Vec::new(),
        }
    }
}

impl HasBoundVariables for QueryNode {
    fn lookup_bound_variables(&self, callback: &mut impl FnMut(usize)) {
        match self {
            QueryNode::TriplePattern {
                subject,
                predicate,
                object,
            } => {
                subject.lookup_bound_variable(callback);
                predicate.lookup_bound_variable(callback);
                object.lookup_bound_variable(callback);
            }
            QueryNode::Noop { bound_variables } => {
                bound_variables.iter().for_each(|v| callback(*v));
            }
            QueryNode::CartesianProductJoin { left, right }
            | QueryNode::ForLoopJoin { left, right } => {
                left.lookup_bound_variables(callback);
                right.lookup_bound_variables(callback);
            }
            QueryNode::Filter { expr, inner } => {
                expr.lookup_bound_variables(callback);
                inner.lookup_bound_variables(callback);
            }
            QueryNode::Skip { child, .. } | QueryNode::Limit { child, .. } => {
                child.lookup_bound_variables(callback);
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum PatternValue<V> {
    Constant(V),
    Variable(usize),
    /// Special variable that is expected to resolve as a blank node.
    BlankVariable(usize),
}

impl<V> PatternValue<V> {
    pub fn lookup_bound_variable(&self, callback: &mut impl FnMut(usize)) {
        if let PatternValue::Variable(v) | PatternValue::BlankVariable(v) = self {
            callback(*v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn bound_variables() {
        let cases = vec![
            (
                QueryNode::TriplePattern {
                    subject: PatternValue::Variable(0usize),
                    predicate: PatternValue::Variable(1usize),
                    object: PatternValue::Variable(2usize),
                },
                BTreeSet::from([0usize, 1usize, 2usize]),
            ),
            (
                QueryNode::Noop {
                    bound_variables: vec![0usize, 1usize],
                },
                BTreeSet::from([0usize, 1usize]),
            ),
            (
                QueryNode::Limit {
                    first: 20usize,
                    child: Box::new(QueryNode::Skip {
                        first: 20usize,
                        child: Box::new(QueryNode::ForLoopJoin {
                            left: Box::new(QueryNode::CartesianProductJoin {
                                left: Box::new(QueryNode::TriplePattern {
                                    subject: PatternValue::BlankVariable(4usize),
                                    predicate: PatternValue::Variable(5usize),
                                    object: PatternValue::Variable(0usize),
                                }),
                                right: Box::new(QueryNode::TriplePattern {
                                    subject: PatternValue::Variable(3usize),
                                    predicate: PatternValue::Variable(1usize),
                                    object: PatternValue::BlankVariable(4usize),
                                }),
                            }),
                            right: Box::new(QueryNode::TriplePattern {
                                subject: PatternValue::Variable(0usize),
                                predicate: PatternValue::Variable(1usize),
                                object: PatternValue::Variable(2usize),
                            }),
                        }),
                    }),
                },
                BTreeSet::from([0usize, 1usize, 2usize, 3usize, 4usize, 5usize]),
            ),
        ];

        for case in cases {
            assert_eq!(case.0.bound_variables(), case.1)
        }
    }

    #[test]
    fn get_var_index() {
        let plan = QueryPlan {
            entrypoint: QueryNode::TriplePattern {
                subject: PatternValue::Variable(0usize),
                predicate: PatternValue::Variable(1usize),
                object: PatternValue::BlankVariable(2usize),
            },
            variables: vec![
                PlanVariable::Basic("1".to_string()),
                PlanVariable::Basic("2".to_string()),
                PlanVariable::BlankNode("3".to_string()),
            ],
        };

        assert_eq!(plan.get_var_index("1"), Some(0usize));
        assert_eq!(plan.get_var_index("2"), Some(1usize));
        assert_eq!(plan.get_var_index("3"), None);
    }
}
