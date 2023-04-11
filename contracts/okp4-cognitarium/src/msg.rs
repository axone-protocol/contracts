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
    /// Insert the Tuples extracting from the provided RDF graph.
    /// It fails if a subject already exists.
    Insert { input: GraphInput },
}

/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

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
