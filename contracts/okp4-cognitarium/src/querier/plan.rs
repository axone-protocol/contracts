use crate::state::{Object, Predicate, Subject};
use std::collections::BTreeSet;

/// Represents a querying plan.
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct QueryPlan {
    /// References the ending node of the plan, when evaluated others nodes will be invoked in
    /// cascade.
    pub entrypoint: QueryNode,

    /// Contains all the query variables, their index in this array are internally used as
    /// identifiers.
    pub variables: Vec<String>,
}

impl QueryPlan {
    pub fn get_var_index(&self, var_name: &str) -> Option<usize> {
        self.variables.iter().enumerate().find_map(|(index, it)| {
            if it.as_str() == var_name {
                return Some(index);
            }
            None
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

    /// Join two nodes by applying the cartesian product of the nodes variables.
    ///
    /// This should be used when the nodes doesn't have variables in common, and can be seen as a
    /// full join of disjoint datasets.  
    CartesianProductJoin { left: Box<Self>, right: Box<Self> },

    /// Join two nodes by using the variables values from the left node as replacement in the right
    /// node.
    ///
    /// This results to an inner join, but the underlying processing stream the variables from the
    /// left node to use them as right node values.
    ForLoopJoin { left: Box<Self>, right: Box<Self> },

    /// Skip the specified first elements from the child node.
    Skip { child: Box<Self>, first: usize },

    /// Limit to the specified first elements from the child node.
    Limit { child: Box<Self>, first: usize },
}

impl QueryNode {
    pub fn bound_variables(&self) -> BTreeSet<usize> {
        let mut vars = BTreeSet::new();
        self.lookup_bound_variables(&mut |v| {
            vars.insert(v);
        });
        vars
    }

    pub fn lookup_bound_variables(&self, callback: &mut impl FnMut(usize)) {
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
            QueryNode::CartesianProductJoin { left, right } => {
                left.lookup_bound_variables(callback);
                right.lookup_bound_variables(callback);
            }
            QueryNode::ForLoopJoin { left, right } => {
                left.lookup_bound_variables(callback);
                right.lookup_bound_variables(callback);
            }
            QueryNode::Skip { child, .. } => child.lookup_bound_variables(callback),
            QueryNode::Limit { child, .. } => child.lookup_bound_variables(callback),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum PatternValue<V> {
    Constant(V),
    Variable(usize),
}

impl<V> PatternValue<V> {
    pub fn lookup_bound_variable(&self, callback: &mut impl FnMut(usize)) {
        if let PatternValue::Variable(v) = self {
            callback(*v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bound_variables() {
        let query = QueryNode::Limit {
            first: 20usize,
            child: Box::new(QueryNode::Skip {
                first: 20usize,
                child: Box::new(QueryNode::ForLoopJoin {
                    left: Box::new(QueryNode::CartesianProductJoin {
                        left: Box::new(QueryNode::TriplePattern {
                            subject: PatternValue::Constant(Subject::Blank("_".to_string())),
                            predicate: PatternValue::Variable(4usize),
                            object: PatternValue::Variable(0usize),
                        }),
                        right: Box::new(QueryNode::TriplePattern {
                            subject: PatternValue::Variable(3usize),
                            predicate: PatternValue::Variable(1usize),
                            object: PatternValue::Constant(Object::Blank("_".to_string())),
                        }),
                    }),
                    right: Box::new(QueryNode::TriplePattern {
                        subject: PatternValue::Variable(0usize),
                        predicate: PatternValue::Variable(1usize),
                        object: PatternValue::Variable(2usize),
                    }),
                }),
            }),
        };

        assert_eq!(
            query.bound_variables(),
            BTreeSet::from([0usize, 1usize, 2usize, 3usize, 4usize])
        )
    }
}
