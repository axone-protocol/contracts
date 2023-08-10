use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Uint128};
use derive_builder::Builder;
use std::collections::BTreeMap;

/// Instantiate message
#[cw_serde]
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
    ///         "predicate": { "node": { "namedNode": {"prefixed": "foaf:givenName"} } },
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
        /// The items to delete.
        delete: Vec<TriplePattern>,
        /// The WHERE clause to apply.
        /// If not provided, all the RDF triples are considered.
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
    Store,

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

/// # StoreLimitsInput
/// Contains requested limitations regarding store usages.
#[cw_serde]
#[derive(Builder)]
#[builder(default, setter(into, strip_option))]
pub struct StoreLimitsInput {
    /// The maximum number of triples the store can contains.
    /// Default to [Uint128::MAX] if not set, which can be considered as no limit.
    #[serde(default = "StoreLimitsInput::default_max_triple_count")]
    pub max_triple_count: Uint128,
    /// The maximum number of bytes the store can contains.
    /// The size of a triple is counted as the sum of the size of its subject, predicate and object,
    /// including the size of data types and language tags if any.
    /// Default to [Uint128::MAX] if not set, which can be considered as no limit.
    #[serde(default = "StoreLimitsInput::default_max_byte_size")]
    pub max_byte_size: Uint128,
    /// The maximum number of bytes the store can contains for a single triple.
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
    /// The maximum number of bytes an insert data query can contains.
    /// Default to [Uint128::MAX] if not set, which can be considered as no limit.
    #[serde(default = "StoreLimitsInput::default_max_insert_data_byte_size")]
    pub max_insert_data_byte_size: Uint128,
    /// The maximum number of triples an insert data query can contains (after parsing).
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
#[derive(Default, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct StoreLimits {
    /// The maximum number of triples the store can contains.
    pub max_triple_count: Uint128,

    /// The maximum number of bytes the store can contains.
    /// The size of a triple is counted as the sum of the size of its subject, predicate and object,
    /// including the size of data types and language tags if any.
    pub max_byte_size: Uint128,

    /// The maximum number of bytes the store can contains for a single triple.
    /// The size of a triple is counted as the sum of the size of its subject, predicate and object,
    /// including the size of data types and language tags if any. The limit is used to prevent
    /// storing very large triples, especially literals.
    pub max_triple_byte_size: Uint128,

    /// The maximum limit of a query, i.e. the maximum number of triples returned by a select query.
    pub max_query_limit: u32,

    /// The maximum number of variables a query can select.
    pub max_query_variable_count: u32,

    /// The maximum number of bytes an insert data query can contains.
    pub max_insert_data_byte_size: Uint128,

    /// The maximum number of triples an insert data query can contains (after parsing).
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

/// # IRI
/// Represents an IRI.
#[cw_serde]
pub enum IRI {
    /// # Prefixed
    /// An IRI prefixed with a prefix.
    /// The prefixed IRI is expanded to a full IRI using the prefix definition specified in the query.
    /// For example, the prefixed IRI `rdf:type` is expanded to `http://www.w3.org/1999/02/22-rdf-syntax-ns#type`.
    Prefixed(String),
    /// # Full
    /// A full IRI.
    Full(String),
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

/// # Value
#[cw_serde]
#[serde(tag = "type")]
pub enum Value {
    /// # URI
    /// Represents an IRI.
    #[serde(rename = "uri")]
    URI {
        /// The value of the IRI.
        value: IRI,
    },
    /// # Literal
    /// Represents a literal S with optional language tag L or datatype IRI D.
    Literal {
        /// The value of the literal.
        value: String,
        /// The language tag of the literal.
        #[serde(rename = "xml:lang")]
        lang: Option<String>,
        /// The datatype of the literal.
        datatype: Option<IRI>,
    },
    /// # BlankNode
    /// Represents a blank node.
    BlankNode {
        /// The identifier of the blank node.
        value: String,
    },
}

/// # SelectQuery
/// Represents a SELECT query over the triple store, allowing to select variables to return
/// and to filter the results.
#[cw_serde]
pub struct SelectQuery {
    /// The prefixes used in the query.
    pub prefixes: Vec<Prefix>,
    /// The items to select.
    /// Note: the number of items to select cannot exceed the maximum query variable count defined
    /// in the store limitations.
    pub select: Vec<SelectItem>,
    /// The WHERE clause.
    /// If `None`, there is no WHERE clause, i.e. all triples are returned without filtering.
    pub r#where: WhereClause,
    /// The maximum number of results to return.
    /// If `None`, there is no limit.
    /// Note: the value of the limit cannot exceed the maximum query limit defined in the store
    /// limitations.
    pub limit: Option<u32>,
}

/// # DescribeQuery
/// Represents a DESCRIBE query over the triple store, allowing to retrieve a description of a resource
/// as a set of triples serialized in a specific format.
#[cw_serde]
pub struct DescribeQuery {
    /// The prefixes used in the query.
    pub prefixes: Vec<Prefix>,
    /// The resource to describe given as a variable or a node.
    pub resource: VarOrNamedNode,
    /// The WHERE clause.
    /// This clause is used to specify the resource identifier to describe using variable bindings.
    pub r#where: WhereClause,
}

/// # ConstructQuery
/// Represents a CONSTRUCT query over the triple store, allowing to retrieve a set of triples
/// serialized in a specific format.
#[cw_serde]
pub struct ConstructQuery {
    /// The prefixes used in the query.
    pub prefixes: Vec<Prefix>,
    /// The triples to construct.
    /// If nothing is provided, the patterns from the `where` clause are used for construction.
    pub construct: Vec<TriplePattern>,
    /// The WHERE clause.
    /// This clause is used to specify the triples to construct using variable bindings.
    pub r#where: WhereClause,
}

/// # Prefix
/// Represents a prefix in a [SelectQuery]. A prefix is a shortcut for a namespace used in the query.
#[cw_serde]
pub struct Prefix {
    /// The prefix.
    pub prefix: String,
    /// The namespace associated with the prefix.
    pub namespace: String,
}

/// # SelectItem
/// Represents an item to select in a [SelectQuery].
#[cw_serde]
pub enum SelectItem {
    /// # Variable
    /// Represents a variable.
    Variable(String),
}

/// # WhereClause
/// Represents a WHERE clause in a [SelectQuery], i.e. a set of conditions to filter the results.
pub type WhereClause = Vec<WhereCondition>;

/// # WhereCondition
/// Represents a condition in a [WhereClause].
#[cw_serde]
pub enum WhereCondition {
    /// # Simple
    /// Represents a simple condition.
    Simple(SimpleWhereCondition),
}

/// # SimpleWhereCondition
/// Represents a simple condition in a [WhereCondition].
#[cw_serde]
pub enum SimpleWhereCondition {
    /// # TriplePattern
    /// Represents a triple pattern, i.e. a condition on a triple based on its subject, predicate and
    /// object.
    TriplePattern(TriplePattern),
}

/// # TriplePattern
/// Represents a triple pattern in a [SimpleWhereCondition].
#[cw_serde]
pub struct TriplePattern {
    /// The subject of the triple pattern.
    pub subject: VarOrNode,
    /// The predicate of the triple pattern.
    pub predicate: VarOrNode,
    /// The object of the triple pattern.
    pub object: VarOrNodeOrLiteral,
}

impl TriplePattern {
    /// Returns the variables used in the triple pattern.
    pub fn variables(&self) -> Vec<String> {
        let mut variables: Vec<String> = vec![];

        if let VarOrNode::Variable(var) = &self.subject {
            variables.push(var.clone());
        }

        if let VarOrNode::Variable(var) = &self.predicate {
            variables.push(var.clone());
        }

        if let VarOrNodeOrLiteral::Variable(var) = &self.object {
            variables.push(var.clone());
        }

        variables
    }
}

/// # VarOrNode
/// Represents either a variable or a node.
#[cw_serde]
pub enum VarOrNode {
    /// # Variable
    /// A variable.
    Variable(String),
    /// # Node
    /// A node, i.e. an IRI or a blank node.
    Node(Node),
}

/// # VarOrNamedNode {
/// Represents either a variable or a named node (IRI).
#[cw_serde]
pub enum VarOrNamedNode {
    /// # Variable
    /// A variable.
    Variable(String),
    /// # NamedNode
    /// An RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri).
    NamedNode(IRI),
}

/// # VarOrNodeOrLiteral
/// Represents either a variable, a node or a literal.
#[cw_serde]
pub enum VarOrNodeOrLiteral {
    /// # Variable
    /// A variable.
    Variable(String),
    /// # Node
    /// A node, i.e. an IRI or a blank node.
    Node(Node),
    /// # Literal
    /// An RDF [literal](https://www.w3.org/TR/rdf11-concepts/#dfn-literal), i.e. a simple literal,
    /// a language-tagged string or a typed value.
    Literal(Literal),
}

/// # Literal
/// An RDF [literal](https://www.w3.org/TR/rdf11-concepts/#dfn-literal).
#[cw_serde]
pub enum Literal {
    /// # Simple
    /// A [simple literal](https://www.w3.org/TR/rdf11-concepts/#dfn-simple-literal) without datatype or language form.
    Simple(String),
    /// # LanguageTaggedString
    /// A [language-tagged string](https://www.w3.org/TR/rdf11-concepts/#dfn-language-tagged-string)
    LanguageTaggedString {
        /// The [lexical form](https://www.w3.org/TR/rdf11-concepts/#dfn-lexical-form).
        value: String,
        /// The [language tag](https://www.w3.org/TR/rdf11-concepts/#dfn-language-tag).
        language: String,
    },
    /// # TypedValue
    /// A value with a datatype.
    TypedValue {
        /// The [lexical form](https://www.w3.org/TR/rdf11-concepts/#dfn-lexical-form).
        value: String,
        /// The [datatype IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-datatype-iri).
        datatype: IRI,
    },
}

/// # Node
/// Represents either an IRI (named node) or a blank node.
#[cw_serde]
pub enum Node {
    /// # NamedNode
    /// An RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri).
    NamedNode(IRI),
    /// # BlankNode
    /// An RDF [blank node](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node).
    BlankNode(String),
}

#[cfg(test)]
mod tests {
    use crate::msg::Literal::Simple;
    use crate::msg::Node::{BlankNode, NamedNode};
    use crate::msg::IRI::{Full, Prefixed};
    use crate::msg::{
        InstantiateMsg, StoreLimitsInput, TriplePattern, VarOrNode, VarOrNodeOrLiteral,
    };
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

    #[test]
    fn variables_from_triple_pattern() {
        let (s, p, o) = ("s".to_string(), "p".to_string(), "o".to_string());
        let (node, prefixed, literal) = (
            "node".to_string(),
            "a:node".to_string(),
            "literal".to_string(),
        );

        let cases = vec![
            (
                TriplePattern {
                    subject: VarOrNode::Variable(s.clone()),
                    predicate: VarOrNode::Variable(p.clone()),
                    object: VarOrNodeOrLiteral::Variable(o.clone()),
                },
                vec![s.clone(), p.clone(), o.clone()],
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Node(NamedNode(Full(node.clone()))),
                    predicate: VarOrNode::Variable(p.clone()),
                    object: VarOrNodeOrLiteral::Variable(o.clone()),
                },
                vec![p.clone(), o.clone()],
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Node(NamedNode(Prefixed(prefixed.clone()))),
                    predicate: VarOrNode::Variable(p.clone()),
                    object: VarOrNodeOrLiteral::Variable(o.clone()),
                },
                vec![p.clone(), o.clone()],
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Node(BlankNode(node.clone())),
                    predicate: VarOrNode::Variable(p.clone()),
                    object: VarOrNodeOrLiteral::Variable(o.clone()),
                },
                vec![p.clone(), o.clone()],
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Variable(s.clone()),
                    predicate: VarOrNode::Node(NamedNode(Full(node.clone()))),
                    object: VarOrNodeOrLiteral::Variable(o.clone()),
                },
                vec![s.clone(), o],
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Variable(s.clone()),
                    predicate: VarOrNode::Variable(p.clone()),
                    object: VarOrNodeOrLiteral::Literal(Simple(literal.clone())),
                },
                vec![s, p],
            ),
            (
                TriplePattern {
                    subject: VarOrNode::Node(BlankNode(node)),
                    predicate: VarOrNode::Node(NamedNode(Prefixed(prefixed))),
                    object: VarOrNodeOrLiteral::Literal(Simple(literal)),
                },
                vec![],
            ),
        ];

        for (triple_pattern, expected) in cases {
            assert_eq!(triple_pattern.variables(), expected);
        }
    }
}
