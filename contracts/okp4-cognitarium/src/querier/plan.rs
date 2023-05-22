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

pub enum PatternValue<V> {
    Constant(V),
    Variable(usize),
}
