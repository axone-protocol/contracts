use crate::parser::{
    ConstructQuery, DescribeQuery, Prefix, SelectQuery, TripleDeleteTemplate, Value, WhereClause,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Uint128};
use derive_builder::Builder;
use std::collections::BTreeMap;

/// Instantiate message
#[cw_serde]
#[derive(Default)]
pub struct InstantiateMsg {
    /// Limitations regarding store usage.
    #[serde(default)]
    pub limits: StoreLimitsInput,
}

/// Execute messages
#[cw_serde]
pub enum ExecuteMsg {
    /// # InsertData
    /// Insert the data as RDF triples in the store.
    /// For already existing triples it acts as no-op.
    ///
    /// Only the smart contract owner (i.e. the address who instantiated it) is authorized to perform
    /// this action.
    InsertData {
        /// The data format in which the triples are serialized.
        /// If not provided, the default format is [Turtle](https://www.w3.org/TR/turtle/) format.
        format: Option<DataFormat>,
        /// The data to insert.
        /// The data must be serialized in the format specified by the `format` field. And the data
        /// are subject to the limitations defined by the `limits` specified at contract instantiation.
        data: Binary,
    },

    /// # DeleteData
    /// Delete the data (RDF triples) from the store matching the patterns defined by the provided
    /// query. For non-existing triples it acts as no-op.
    ///
    /// Example:
    /// ```json
    /// {
    ///   "prefixes": [
    ///     { "prefix": "foaf", "namespace": "http://xmlns.com/foaf/0.1/" }
    ///   ],
    ///   "delete": [
    ///     {
    ///         "subject": { "variable": "s" },
    ///         "predicate": { "variable": "p" },
    ///         "object": { "variable": "o" }
    ///     }
    ///   ],
    ///   "where": [
    ///     { "simple": { "triplePattern": {
    ///         "subject": { "variable": "s" },
    ///         "predicate": { "namedNode": {"prefixed": "foaf:givenName"} },
    ///         "object": { "literal": { "simple": "Myrddin" } }
    ///     } } },
    ///     { "simple": { "triplePattern": {
    ///         "subject": { "variable": "s" },
    ///         "predicate": { "variable": "p" },
    ///         "object": { "variable": "o" }
    ///     } } }
    ///  ]
    /// ```
    ///
    /// Only the smart contract owner (i.e. the address who instantiated it) is authorized to perform
    /// this action.
    DeleteData {
        /// The prefixes used in the operation.
        prefixes: Vec<Prefix>,
        /// Specifies the specific triple templates to delete.
        /// If nothing is provided and the `where` clause is a single Bgp, the patterns are used for
        /// deletion.
        delete: Vec<TripleDeleteTemplate>,
        /// Defines the patterns that data (RDF triples) should match in order for it to be
        /// considered for deletion, if any.
        r#where: Option<WhereClause>,
    },
}

/// # SelectQuery
/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// # Store
    ///
    /// Returns information about the triple store.
    #[returns(StoreResponse)]
    Store {},

    /// # Select
    ///
    /// Returns the resources matching the criteria defined by the provided query.
    ///
    #[returns(SelectResponse)]
    Select {
        /// The query to execute.
        query: SelectQuery,
    },

    /// # Describe
    ///
    /// Returns a description of the resource identified by the provided IRI as a set of RDF triples
    /// serialized in the provided format.
    #[returns(DescribeResponse)]
    Describe {
        /// The query to execute.
        query: DescribeQuery,
        /// The format in which the triples are serialized.
        /// If not provided, the default format is [Turtle](https://www.w3.org/TR/turtle/) format.
        format: Option<DataFormat>,
    },

    /// # Construct
    ///
    /// Returns the resources matching the criteria defined by the provided query as a set of RDF
    /// triples serialized in the provided format.
    #[returns(ConstructResponse)]
    Construct {
        /// The query to execute.
        query: ConstructQuery,
        /// The format in which the triples are serialized.
        /// If not provided, the default format is [Turtle](https://www.w3.org/TR/turtle/) format.
        format: Option<DataFormat>,
    },
}

/// # DataFormat
/// Represents the format in which the data are serialized, for example when returned by a query or
/// when inserted in the store.
#[cw_serde]
#[derive(Default)]
pub enum DataFormat {
    /// # RDF XML
    /// Output in [RDF/XML](https://www.w3.org/TR/rdf-syntax-grammar/) format.
    #[serde(rename = "rdf_xml")]
    RDFXml,
    /// # Turtle
    /// Output in [Turtle](https://www.w3.org/TR/turtle/) format.
    #[serde(rename = "turtle")]
    #[default]
    Turtle,
    /// # N-Triples
    /// Output in [N-Triples](https://www.w3.org/TR/n-triples/) format.
    #[serde(rename = "n_triples")]
    NTriples,
    /// # N-Quads
    /// Output in [N-Quads](https://www.w3.org/TR/n-quads/) format.
    #[serde(rename = "n_quads")]
    NQuads,
}

impl From<&DataFormat> for axone_rdf::serde::DataFormat {
    fn from(value: &DataFormat) -> Self {
        match value {
            DataFormat::RDFXml => Self::RDFXml,
            DataFormat::Turtle => Self::Turtle,
            DataFormat::NTriples => Self::NTriples,
            DataFormat::NQuads => Self::NQuads,
        }
    }
}

/// # StoreLimitsInput
/// Contains requested limitations regarding store usages.
#[cw_serde]
#[derive(Builder)]
#[builder(default, setter(into, strip_option))]
pub struct StoreLimitsInput {
    /// The maximum number of triples the store can contain.
    /// Default to [Uint128::MAX] if not set, which can be considered as no limit.
    #[serde(default = "StoreLimitsInput::default_max_triple_count")]
    pub max_triple_count: Uint128,
    /// The maximum number of bytes the store can contain.
    /// The size of a triple is counted as the sum of the size of its subject, predicate and object,
    /// including the size of data types and language tags if any.
    /// Default to [Uint128::MAX] if not set, which can be considered as no limit.
    #[serde(default = "StoreLimitsInput::default_max_byte_size")]
    pub max_byte_size: Uint128,
    /// The maximum number of bytes the store can contain for a single triple.
    /// The size of a triple is counted as the sum of the size of its subject, predicate and object,
    /// including the size of data types and language tags if any. The limit is used to prevent
    /// storing very large triples, especially literals.
    /// Default to [Uint128::MAX] if not set, which can be considered as no limit.
    #[serde(default = "StoreLimitsInput::default_max_triple_byte_size")]
    pub max_triple_byte_size: Uint128,
    /// The maximum limit of a query, i.e. the maximum number of triples returned by a select query.
    /// Default to 30 if not set.
    #[serde(default = "StoreLimitsInput::default_max_query_limit")]
    pub max_query_limit: u32,
    /// The maximum number of variables a query can select.
    /// Default to 30 if not set.
    #[serde(default = "StoreLimitsInput::default_max_query_variable_count")]
    pub max_query_variable_count: u32,
    /// The maximum number of bytes an insert data query can contain.
    /// Default to [Uint128::MAX] if not set, which can be considered as no limit.
    #[serde(default = "StoreLimitsInput::default_max_insert_data_byte_size")]
    pub max_insert_data_byte_size: Uint128,
    /// The maximum number of triples an insert data query can contain (after parsing).
    /// Default to [Uint128::MAX] if not set, which can be considered as no limit.
    #[serde(default = "StoreLimitsInput::default_max_insert_data_triple_count")]
    pub max_insert_data_triple_count: Uint128,
}

impl StoreLimitsInput {
    const fn default_max_query_limit() -> u32 {
        30
    }
    const fn default_max_query_variable_count() -> u32 {
        30
    }
    const fn default_max_triple_count() -> Uint128 {
        Uint128::MAX
    }
    const fn default_max_byte_size() -> Uint128 {
        Uint128::MAX
    }
    const fn default_max_triple_byte_size() -> Uint128 {
        Uint128::MAX
    }
    const fn default_max_insert_data_byte_size() -> Uint128 {
        Uint128::MAX
    }
    const fn default_max_insert_data_triple_count() -> Uint128 {
        Uint128::MAX
    }
}

impl Default for StoreLimitsInput {
    fn default() -> Self {
        Self {
            max_triple_count: Self::default_max_triple_count(),
            max_byte_size: Self::default_max_byte_size(),
            max_triple_byte_size: Self::default_max_triple_byte_size(),
            max_query_limit: Self::default_max_query_limit(),
            max_query_variable_count: Self::default_max_query_variable_count(),
            max_insert_data_byte_size: Self::default_max_insert_data_byte_size(),
            max_insert_data_triple_count: Self::default_max_insert_data_triple_count(),
        }
    }
}

/// # StoreResponse
///
/// Contains information related to triple store.
#[cw_serde]
pub struct StoreResponse {
    /// The store owner.
    pub owner: String,

    /// The store limits.
    pub limits: StoreLimits,

    /// The store current usage.
    pub stat: StoreStat,
}

/// # StoreLimits
/// Contains limitations regarding store usages.
#[cw_serde]
#[derive(Builder, Default)]
#[builder(default, setter(into, strip_option))]
pub struct StoreLimits {
    /// The maximum number of triples the store can contain.
    pub max_triple_count: Uint128,

    /// The maximum number of bytes the store can contain.
    /// The size of a triple is counted as the sum of the size of its subject, predicate and object,
    /// including the size of data types and language tags if any.
    pub max_byte_size: Uint128,

    /// The maximum number of bytes the store can contain for a single triple.
    /// The size of a triple is counted as the sum of the size of its subject, predicate and object,
    /// including the size of data types and language tags if any. The limit is used to prevent
    /// storing very large triples, especially literals.
    pub max_triple_byte_size: Uint128,

    /// The maximum limit of a query, i.e. the maximum number of triples returned by a select query.
    pub max_query_limit: u32,

    /// The maximum number of variables a query can select.
    pub max_query_variable_count: u32,

    /// The maximum number of bytes an insert data query can contain.
    pub max_insert_data_byte_size: Uint128,

    /// The maximum number of triples an insert data query can contain (after parsing).
    pub max_insert_data_triple_count: Uint128,
}

/// # StoreStat
///
/// Contains usage information about the triple store.
#[cw_serde]
pub struct StoreStat {
    /// The total number of triple present in the store.
    pub triple_count: Uint128,

    /// The total number of IRI namespace present in the store.
    pub namespace_count: Uint128,

    /// The total triple size in the store, in bytes.
    pub byte_size: Uint128,
}

/// # SelectResponse
/// Represents the response of a [QueryMsg::Select] query.
#[cw_serde]
pub struct SelectResponse {
    /// The head of the response, i.e. the set of variables mentioned in the results.
    pub head: Head,
    /// The results of the select query.
    pub results: Results,
}

/// # DescribeResponse
/// Represents the response of a [QueryMsg::Describe] query.
#[cw_serde]
pub struct DescribeResponse {
    /// The format of the data.
    pub format: DataFormat,
    /// The data serialized in the specified format.
    pub data: Binary,
}

/// # ConstructResponse
/// Represents the response of a [QueryMsg::Construct] query.
#[cw_serde]
pub struct ConstructResponse {
    /// The format of the data.
    pub format: DataFormat,
    /// The data serialized in the specified format.
    pub data: Binary,
}

/// # Head
/// Represents the head of a [SelectResponse].
#[cw_serde]
pub struct Head {
    /// The variables selected in the query.
    pub vars: Vec<String>,
}

/// # Results
/// Represents the results of a [SelectResponse].
#[cw_serde]
pub struct Results {
    /// The bindings of the results.
    pub bindings: Vec<BTreeMap<String, Value>>,
}

#[cfg(test)]
mod tests {
    use crate::msg::{InstantiateMsg, StoreLimitsInput};
    use cosmwasm_std::Uint128;
    use schemars::_serde_json;

    #[test]
    fn store_limit_default_deserialization() {
        let json = r#"
          {}
    "#;

        let input: StoreLimitsInput = _serde_json::from_str(json).unwrap();
        assert_eq!(input.max_query_limit, 30);
        assert_eq!(input.max_query_variable_count, 30);
        assert_eq!(input.max_byte_size, Uint128::MAX);
        assert_eq!(input.max_triple_count, Uint128::MAX);
        assert_eq!(input.max_triple_byte_size, Uint128::MAX);
        assert_eq!(input.max_insert_data_byte_size, Uint128::MAX);
        assert_eq!(input.max_insert_data_triple_count, Uint128::MAX);
    }

    #[test]
    fn instantiate_default_deserialization() {
        let json = r#"
          {}
    "#;
        let msg: InstantiateMsg = _serde_json::from_str(json).unwrap();

        assert_eq!(msg.limits.max_query_limit, 30);
        assert_eq!(msg.limits.max_query_variable_count, 30);
        assert_eq!(msg.limits.max_byte_size, Uint128::MAX);
        assert_eq!(msg.limits.max_triple_count, Uint128::MAX);
        assert_eq!(msg.limits.max_triple_byte_size, Uint128::MAX);
        assert_eq!(msg.limits.max_insert_data_byte_size, Uint128::MAX);
        assert_eq!(msg.limits.max_insert_data_triple_count, Uint128::MAX);
    }
}
