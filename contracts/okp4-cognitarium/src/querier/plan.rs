use crate::msg::{Prefix, SelectQuery, SimpleWhereCondition, TriplePattern, WhereCondition};
use crate::state::{Object, Predicate, Subject};
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Represents a querying plan.
pub struct QueryPlan {
    /// References the ending node of the plan, when evaluated others nodes will be invoked in
    /// cascade.
    pub entrypoint: QueryNode,

    /// Contains all the query variables, their index in this array are internally used as
    /// identifiers.
    pub variables: Vec<String>,
}

/// Represents a single part of the query plan processing. Each node is intended to provide a
/// specific behavior given an evaluation context.
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
    CartesianProductJoin { left: Self, right: Self },

    /// Join two nodes by using the variables values from the left node as replacement in the right
    /// node.
    ///
    /// This results to an inner join, but the underlying processing stream the variables from the
    /// left node to use them as right node values.
    ForLoopJoin { left: Self, right: Self },

    /// Skip the specified first elements from the child node.
    Skip { child: Self, first: usize },

    /// Limit to the specified first elements from the child node.
    Limit { child: Self, first: usize },
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
