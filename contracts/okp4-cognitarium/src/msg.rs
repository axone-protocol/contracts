use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Uint128};
use std::collections::HashMap;

/// Instantiate message
#[cw_serde]
pub struct InstantiateMsg {
    /// Limitations regarding store usage.
    pub limits: StoreLimits,
}

/// Execute messages
#[cw_serde]
pub enum ExecuteMsg {
    /// # Insert
    /// Insert the Tuples extracted from the provided RDF graph.
    /// It fails if a subject already exists.
    Insert { input: GraphInput },

    /// # Remove
    /// Remove the resources matching the constraints expressed in [resource_sets].
    Remove {
        /// resource_sets represents multiple sets of resources matching the provided constraints
        /// and identified by a key, the key can be used in other constraints to expressed links
        /// between resources.
        resource_sets: HashMap<String, ResourceConstraints>,
    },
}

/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// # SelectResources
    ///
    /// Returns the resources matching the constraints expressed in [resource_sets] formatted according
    /// to the provided [format].
    ///
    /// A resource being considered as a set of triples (i.e. subject - predicate - object) having the
    /// same subject.
    ///
    /// The provided [resource_sets] being able to depend on each other, circular dependencies are
    /// considered as an error.
    #[returns(SelectResourcesResponse)]
    SelectResources {
        /// resource_sets represents multiple sets of resources matching the provided constraints
        /// and identified by a key, the key can be used in other constraints to expressed links
        /// between resources.
        resource_sets: HashMap<String, ResourceConstraints>,

        /// format denotes the expected output format. Its value shape the way the response shall be
        /// interpreted.
        format: ResourcesOutputFormat,
    },
}

/// # StoreLimits
/// Contains limitations regarding store usages.
#[cw_serde]
pub struct StoreLimits {
    /// max_triple_count denotes the maximum number of triples the store can contains.
    /// If None, there is no limit on the number of triples.
    pub max_triple_count: Option<Uint128>,
}

/// # GraphInput
/// Represents an RDF Graph as input supporting multiple formats.
#[cw_serde]
pub enum GraphInput {
    /// Input in [RDF/XML](https://www.w3.org/TR/rdf-syntax-grammar/) format.
    XML { data: Binary },

    /// Input in [N-Triples](https://www.w3.org/TR/turtle/) format with support of the [N-Triples star](https://w3c.github.io/rdf-star/cg-spec/2021-12-17.html#turtle-star) syntax.
    Turtle { data: Binary },

    /// Input in [N-Triples](https://www.w3.org/TR/n-triples/) format with support of the [N-Triples star](https://w3c.github.io/rdf-star/cg-spec/2021-12-17.html#n-triples-star) syntax.
    NTriples { data: Binary },
}

#[cw_serde]
pub enum ResourcesOutputFormat {
    Json {
        project: HashMap<String, Vec<String>>,
    },
    Noop,
}

#[cw_serde]
pub enum SelectResourcesResponse {}

/// # ResourceConstraints
/// ResourceConstraints contains a set of resource constraints behaving as a logical conjunction, it
/// means that a resource must match all the constraints.
pub type ResourceConstraints = Vec<ResourceConstraint>;

/// # ResourceConstraint
/// ResourceConstraint represents a constraint a resource shall match, it can depends on another
/// resource set to express links between different resources.
#[cw_serde]
pub enum ResourceConstraint {
    /// Subject match a resource containing the provided value as subject.
    Subject(String),

    /// Property match a resource containing the provided pair of ([predicate], [object]).
    Property {
        /// predicate denotes the predicate to match.
        predicate: String,

        /// object denotes the object value associated with the predicate.
        /// It can reference either a literal value or a reference to another resource set.
        object: ValueOrRef,
    },

    /// Referenced match a resource if referenced by another resource. For instance, if the resource's
    /// subject under constraint is referenced as the object in another resource's triple, the triple
    /// containing the provided pair of ([subject], [predicate]).
    Referenced {
        /// subject denotes the resource referencing the resource under constraint.
        /// It can reference either a literal value or a reference to another resource set.
        subject: ValueOrRef,

        /// The predicate on the referencing resource that shall express the reference.
        predicate: String,
    },
}

/// # ValueOrRef
/// ValueOrRef represents an expected value in a resource constraint being either literal or a reference
/// to another resource set.
///
/// When referencing a resource set the value will be considered as logical disjunction of every
/// resource's subject it contains.
#[cw_serde]
pub enum ValueOrRef {
    /// The literal value.
    Value(String),

    /// The reference to another resource set, identified by its key.
    ResourceSet(String),
}
