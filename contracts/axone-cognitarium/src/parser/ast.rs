use cosmwasm_schema::cw_serde;

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
    pub r#where: Option<WhereClause>,
}

/// # ConstructQuery
/// Represents a CONSTRUCT query over the triple store, allowing to retrieve a set of triples
/// serialized in a specific format.
#[cw_serde]
pub struct ConstructQuery {
    /// The prefixes used in the query.
    pub prefixes: Vec<Prefix>,
    /// The triples to construct.
    /// If nothing is provided and the `where` clause is a single Bgp, the patterns are used for
    /// construction.
    pub construct: Vec<TripleConstructTemplate>,
    /// The WHERE clause.
    /// This clause is used to specify the triples to construct using variable bindings.
    pub r#where: WhereClause,
}

/// # Prefix
/// Represents a prefix, i.e. a shortcut for a namespace used in a query.
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
/// Represents a WHERE clause, i.e. a set of conditions to filter the results.
#[cw_serde]
pub enum WhereClause {
    /// # Bgp
    /// Represents a basic graph pattern expressed as a set of triple patterns.
    Bgp { patterns: Vec<TriplePattern> },

    /// # LateralJoin
    /// Evaluates right for all result row of left
    LateralJoin { left: Box<Self>, right: Box<Self> },

    /// # Filter
    /// Filters the inner clause matching the expression.
    /// The solutions coming from the inner clause that do not match the expression are discarded.
    /// The variables provided in the inner clause are available in the filter expression.
    Filter { expr: Expression, inner: Box<Self> },
}

/// # Expression
/// Represents a logical combination of operations whose evaluation results in a term.
#[cw_serde]
pub enum Expression {
    /// A named node constant.
    NamedNode(IRI),
    /// A literal constant.
    Literal(Literal),
    /// A variable that must be bound for evaluation.
    Variable(String),
    /// Logical conjunction of expressions.
    /// All expressions must evaluate to true for the conjunction to be true.
    /// If the conjunction is empty, it is considered true.
    And(Vec<Self>),
    /// Logical disjunction of expressions.
    /// At least one expression must evaluate to true for the disjunction to be true.
    /// If the disjunction is empty, it is considered false.
    Or(Vec<Self>),
    /// Equality comparison.
    Equal(Box<Self>, Box<Self>),
    /// Greater than comparison.
    Greater(Box<Self>, Box<Self>),
    /// Greater or equal comparison.
    GreaterOrEqual(Box<Self>, Box<Self>),
    /// Less than comparison.
    Less(Box<Self>, Box<Self>),
    /// Less or equal comparison.
    LessOrEqual(Box<Self>, Box<Self>),
    /// Negation of an expression.
    Not(Box<Self>),
}

/// # TripleDeleteTemplate
/// Represents a triple template to be deleted.
#[cw_serde]
pub struct TripleDeleteTemplate {
    /// The subject of the triple pattern.
    pub subject: VarOrNamedNode,
    /// The predicate of the triple pattern.
    pub predicate: VarOrNamedNode,
    /// The object of the triple pattern.
    pub object: VarOrNamedNodeOrLiteral,
}

/// # TripleConstructTemplate
/// Represents a triple template to be forged for a construct query.
#[cw_serde]
pub struct TripleConstructTemplate {
    /// The subject of the triple pattern.
    pub subject: VarOrNode,
    /// The predicate of the triple pattern.
    pub predicate: VarOrNamedNode,
    /// The object of the triple pattern.
    pub object: VarOrNodeOrLiteral,
}

/// # TriplePattern
/// Represents a triple pattern in a [SimpleWhereCondition].
#[cw_serde]
pub struct TriplePattern {
    /// The subject of the triple pattern.
    pub subject: VarOrNode,
    /// The predicate of the triple pattern.
    pub predicate: VarOrNamedNode,
    /// The object of the triple pattern.
    pub object: VarOrNodeOrLiteral,
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

/// # VarOrNamedNode
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

/// # VarOrNamedNodeOrLiteral
/// Represents either a variable, a named node or a literal.
#[cw_serde]
pub enum VarOrNamedNodeOrLiteral {
    /// # Variable
    /// A variable.
    Variable(String),
    /// # NamedNode
    /// An RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri).
    NamedNode(IRI),
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
