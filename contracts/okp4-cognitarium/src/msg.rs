use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Uint128};

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
    /// For already existing triples it act as no-op.
    ///
    /// Only the smart contract owner (i.e. the address who instantiated it) is authorized to perform
    /// this action.
    Insert { input: GraphInput },

    /// # Remove
    /// Remove all the Tuples linked to the resources matching the criteria defined in the provided
    /// queries.
    ///
    /// Only the smart contract owner (i.e. the address who instantiated it) is authorized to perform
    /// this action.
    Remove {
        /// The queries act as the logical disjunction of each single query, a resource shall match
        /// at least one query.
        queries: Vec<ResourceQuery>,
    },
}

/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// # Resources
    ///
    /// Returns the resources matching the criteria defined in the provided queries formatted according
    /// to the provided format.
    #[returns(ResourcesResponse)]
    Resources {
        /// The queries act as the logical disjunction of each single query, a resource shall match
        /// at least one query.
        queries: Vec<ResourceQuery>,

        /// The expected output format. Its value shape the way the response shall be interpreted.
        format: ResourcesOutputFormat,
    },
}

/// # StoreLimits
/// Contains limitations regarding store usages.
#[cw_serde]
pub struct StoreLimits {
    /// The maximum number of triples the store can contains.
    /// If `None`, there is no limit on the number of triples.
    pub max_triple_count: Option<Uint128>,
}

/// # GraphInput
/// Represents an RDF Graph as input supporting multiple formats.
#[cw_serde]
pub enum GraphInput {
    /// Input in [RDF/XML](https://www.w3.org/TR/rdf-syntax-grammar/) format.
    XML(Binary),

    /// Input in [Turtle](https://www.w3.org/TR/turtle/) format with support of the [Turtle star](https://w3c.github.io/rdf-star/cg-spec/2021-12-17.html#turtle-star) syntax.
    Turtle(Binary),

    /// Input in [N-Triples](https://www.w3.org/TR/n-triples/) format with support of the [N-Triples star](https://w3c.github.io/rdf-star/cg-spec/2021-12-17.html#n-triples-star) syntax.
    NTriples(Binary),
}

/// # ResourcesOutputFormat
/// Supported output formats for [QueryMsg::Resources] query.
#[cw_serde]
pub enum ResourcesOutputFormat {
    /// TODO: remove me once there are proper output formats..
    Dummy,
}

/// # ResourcesResponse
/// Response to the [QueryMsg::Resources] query, its content depends on the specified [ResourcesOutputFormat].
#[cw_serde]
pub enum ResourcesResponse {
    /// TODO: remove me once there are proper output formats..
    Dummy,
}

/// # ResourceQuery
/// A named query targeting resources.
///
/// As the contained [ResourceCriteria] can rely on other [ResourceQuery] it is possible to build
/// circular queries, which is forbidden and will result in an error.
#[cw_serde]
pub struct ResourceQuery {
    /// The query name, can be used to reference another query to allow join.
    /// Must be unique.
    pub name: String,

    /// The set of criteria a resource must meet to validate the query, it act as the logical
    /// conjunction of all the criteria.
    pub criteria: Vec<ResourceCriteria>,
}

/// # ResourceCriteria
/// Represents a single query criteria on a resource.
///
/// It can rely on another query referencing it by its name to express conditions on links between
/// resources (e.g. the `subject` of a resource shall be referenced in a resource of another query).
/// It behaves as a right join, the resources of the referenced query aren't filtered.
#[cw_serde]
pub enum ResourceCriteria {
    /// Subject match a resource containing the provided node as subject.
    Subject(Node),

    /// Property match a resource matching the pair of (`predicate`, `object`).
    Property {
        /// The predicate to match.
        predicate: Node,

        /// The object to match, which may be joined on another query.
        object: ValueOrJoin<ObjectValue>,
    },

    /// Referenced match a resource whose `subject` is referenced in another resource.
    Referenced {
        /// The `subject` the referencing resource shall have, which may be joined on another query.
        referer: ValueOrJoin<Node>,

        /// The predicate through which the referencing resource shall express the reference.
        property: Node,
    },
}

/// # Node
/// Node denotes, among RDF elements, either a named or blank node, for instance:
///
/// A named node can be represented given its IRI: `http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral`.
///
/// A blank node given its id: `9af906ad-d3b1-4a05-ae4b-9288df593d5b`.
pub type Node = String;

/// # Literal
/// Literal represents the possible form an object literal value can have.
#[cw_serde]
pub enum Literal {
    /// A simple string literal value.
    Value(String),

    /// An internationalized string value.
    I18NValue { value: String, language: String },

    /// A typed value.
    Typed { value: String, datatype: Node },
}

/// # ObjectValue
/// Represents the different value an object can take.
#[cw_serde]
pub enum ObjectValue {
    /// A literal value.
    Literal(Literal),

    /// A node to another resource.
    Node(Node),
}

/// # ValueOrJoin
/// Represents an expected value in a [ResourceCriteria], which can be either provided static value
/// or a join on another [ResourceQuery].
#[cw_serde]
pub enum ValueOrJoin<T> {
    /// A static value.
    Value(T),

    /// A reference to another [ResourceQuery], identified by its name.
    JoinQuery(String),
}
