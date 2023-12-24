# Cognitarium

## Overview

The `okp4-cognitarium` smart contract enables the storage of [RDF graphs triples](https://en.wikipedia.org/wiki/Semantic_triple) (i.e. `subject`-`predicate`-`object`) in any [Cosmos blockchains](https://cosmos.network/) using the [CosmWasm](https://cosmwasm.com/) framework.

➡️ Checkout the [examples](https://github.com/okp4/contracts/tree/main/contracts/okp4-cognitarium/examples/) for usage information.

## InstantiateMsg

Instantiate message

| parameter                             | description                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `limits`                              | **[StoreLimitsInput](#storelimitsinput)**. Limitations regarding store usage.                                                                                                                                                                                                                                                                                                                                                                                                  |
| `limits.max_byte_size`                | **[Uint128](#uint128)**. The maximum number of bytes the store can contain. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. Default to [Uint128::MAX] if not set, which can be considered as no limit.<br />**Default:** `"340282366920938463463374607431768211455"`                                                                                                   |
| `limits.max_insert_data_byte_size`    | **[Uint128](#uint128)**. The maximum number of bytes an insert data query can contain. Default to [Uint128::MAX] if not set, which can be considered as no limit.<br />**Default:** `"340282366920938463463374607431768211455"`                                                                                                                                                                                                                                                |
| `limits.max_insert_data_triple_count` | **[Uint128](#uint128)**. The maximum number of triples an insert data query can contain (after parsing). Default to [Uint128::MAX] if not set, which can be considered as no limit.<br />**Default:** `"340282366920938463463374607431768211455"`                                                                                                                                                                                                                              |
| `limits.max_query_limit`              | **integer**. The maximum limit of a query, i.e. the maximum number of triples returned by a select query. Default to 30 if not set.<br />**Default:** `30`                                                                                                                                                                                                                                                                                                                     |
| `limits.max_query_variable_count`     | **integer**. The maximum number of variables a query can select. Default to 30 if not set.<br />**Default:** `30`                                                                                                                                                                                                                                                                                                                                                              |
| `limits.max_triple_byte_size`         | **[Uint128](#uint128)**. The maximum number of bytes the store can contain for a single triple. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. The limit is used to prevent storing very large triples, especially literals. Default to [Uint128::MAX] if not set, which can be considered as no limit.<br />**Default:** `"340282366920938463463374607431768211455"` |
| `limits.max_triple_count`             | **[Uint128](#uint128)**. The maximum number of triples the store can contain. Default to [Uint128::MAX] if not set, which can be considered as no limit.<br />**Default:** `"340282366920938463463374607431768211455"`                                                                                                                                                                                                                                                         |

## ExecuteMsg

Execute messages

### ExecuteMsg::InsertData

Insert the data as RDF triples in the store. For already existing triples it acts as no-op.

Only the smart contract owner (i.e. the address who instantiated it) is authorized to perform this action.

| parameter            | description                                                                                                                                                                                                                                   |
| -------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `insert_data`        | _(Required.) _ **object**.                                                                                                                                                                                                                    |
| `insert_data.data`   | _(Required.) _ **[Binary](#binary)**. The data to insert. The data must be serialized in the format specified by the `format` field. And the data are subject to the limitations defined by the `limits` specified at contract instantiation. |
| `insert_data.format` | **[DataFormat](#dataformat)\|null**. The data format in which the triples are serialized. If not provided, the default format is [Turtle](https://www.w3.org/TR/turtle/) format.                                                              |

### ExecuteMsg::DeleteData

Delete the data (RDF triples) from the store matching the patterns defined by the provided query. For non-existing triples it acts as no-op.

Example: `json { "prefixes": [ { "prefix": "foaf", "namespace": "http://xmlns.com/foaf/0.1/" } ], "delete": [ { "subject": { "variable": "s" }, "predicate": { "variable": "p" }, "object": { "variable": "o" } } ], "where": [ { "simple": { "triplePattern": { "subject": { "variable": "s" }, "predicate": { "node": { "namedNode": {"prefixed": "foaf:givenName"} } }, "object": { "literal": { "simple": "Myrddin" } } } } }, { "simple": { "triplePattern": { "subject": { "variable": "s" }, "predicate": { "variable": "p" }, "object": { "variable": "o" } } } } ] `

Only the smart contract owner (i.e. the address who instantiated it) is authorized to perform this action.

| parameter              | description                                                                                                                                                                                            |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `delete_data`          | _(Required.) _ **object**.                                                                                                                                                                             |
| `delete_data.delete`   | _(Required.) _ **Array&lt;[TriplePattern](#triplepattern)&gt;**. Specifies the specific triple patterns to delete. If nothing is provided, the patterns from the `where` clause are used for deletion. |
| `delete_data.prefixes` | _(Required.) _ **Array&lt;[Prefix](#prefix)&gt;**. The prefixes used in the operation.                                                                                                                 |
| `delete_data.where`    | _(Required.) _ **Array&lt;[WhereCondition](#wherecondition)&gt;**. Defines the patterns that data (RDF triples) should match in order for it to be considered for deletion.                            |

## QueryMsg

Query messages

### QueryMsg::Store

Returns information about the triple store.

| literal   |
| --------- |
| `"store"` |

### QueryMsg::Select

Returns the resources matching the criteria defined by the provided query.

| parameter      | description                                                           |
| -------------- | --------------------------------------------------------------------- |
| `select`       | _(Required.) _ **object**.                                            |
| `select.query` | _(Required.) _ **[SelectQuery](#selectquery)**. The query to execute. |

### QueryMsg::Describe

Returns a description of the resource identified by the provided IRI as a set of RDF triples serialized in the provided format.

| parameter         | description                                                                                                                                                                 |
| ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `describe`        | _(Required.) _ **object**.                                                                                                                                                  |
| `describe.format` | **[DataFormat](#dataformat)\|null**. The format in which the triples are serialized. If not provided, the default format is [Turtle](https://www.w3.org/TR/turtle/) format. |
| `describe.query`  | _(Required.) _ **[DescribeQuery](#describequery)**. The query to execute.                                                                                                   |

### QueryMsg::Construct

Returns the resources matching the criteria defined by the provided query as a set of RDF triples serialized in the provided format.

| parameter          | description                                                                                                                                                                 |
| ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `construct`        | _(Required.) _ **object**.                                                                                                                                                  |
| `construct.format` | **[DataFormat](#dataformat)\|null**. The format in which the triples are serialized. If not provided, the default format is [Turtle](https://www.w3.org/TR/turtle/) format. |
| `construct.query`  | _(Required.) _ **[ConstructQuery](#constructquery)**. The query to execute.                                                                                                 |

## Responses

### construct

Represents the response of a [QueryMsg::Construct] query.

| property | description                                                                        |
| -------- | ---------------------------------------------------------------------------------- |
| `data`   | _(Required.) _ **[Binary](#binary)**. The data serialized in the specified format. |
| `format` | _(Required.) _ **[DataFormat](#dataformat)**. The format of the data.              |

### describe

Represents the response of a [QueryMsg::Describe] query.

| property | description                                                                        |
| -------- | ---------------------------------------------------------------------------------- |
| `data`   | _(Required.) _ **[Binary](#binary)**. The data serialized in the specified format. |
| `format` | _(Required.) _ **[DataFormat](#dataformat)**. The format of the data.              |

### select

Represents the response of a [QueryMsg::Select] query.

| property           | description                                                                                                     |
| ------------------ | --------------------------------------------------------------------------------------------------------------- |
| `head`             | _(Required.) _ **[Head](#head)**. The head of the response, i.e. the set of variables mentioned in the results. |
| `head.vars`        | **Array&lt;string&gt;**. The variables selected in the query.                                                   |
| `results`          | _(Required.) _ **[Results](#results)**. The results of the select query.                                        |
| `results.bindings` | **Array&lt;object&gt;**. The bindings of the results.                                                           |

### store

Contains information related to triple store.

| property                              | description                                                                                                                                                                                                                                                                                                                           |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `limits`                              | _(Required.) _ **[StoreLimits](#storelimits)**. The store limits.                                                                                                                                                                                                                                                                     |
| `limits.max_byte_size`                | **[Uint128](#uint128)**. The maximum number of bytes the store can contain. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any.                                                                                                   |
| `limits.max_insert_data_byte_size`    | **[Uint128](#uint128)**. The maximum number of bytes an insert data query can contain.                                                                                                                                                                                                                                                |
| `limits.max_insert_data_triple_count` | **[Uint128](#uint128)**. The maximum number of triples an insert data query can contain (after parsing).                                                                                                                                                                                                                              |
| `limits.max_query_limit`              | **integer**. The maximum limit of a query, i.e. the maximum number of triples returned by a select query.                                                                                                                                                                                                                             |
| `limits.max_query_variable_count`     | **integer**. The maximum number of variables a query can select.                                                                                                                                                                                                                                                                      |
| `limits.max_triple_byte_size`         | **[Uint128](#uint128)**. The maximum number of bytes the store can contain for a single triple. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. The limit is used to prevent storing very large triples, especially literals. |
| `limits.max_triple_count`             | **[Uint128](#uint128)**. The maximum number of triples the store can contain.                                                                                                                                                                                                                                                         |
| `owner`                               | _(Required.) _ **string**. The store owner.                                                                                                                                                                                                                                                                                           |
| `stat`                                | _(Required.) _ **[StoreStat](#storestat)**. The store current usage.                                                                                                                                                                                                                                                                  |
| `stat.byte_size`                      | **[Uint128](#uint128)**. The total triple size in the store, in bytes.                                                                                                                                                                                                                                                                |
| `stat.namespace_count`                | **[Uint128](#uint128)**. The total number of IRI namespace present in the store.                                                                                                                                                                                                                                                      |
| `stat.triple_count`                   | **[Uint128](#uint128)**. The total number of triple present in the store.                                                                                                                                                                                                                                                             |

## Definitions

### Binary

A string containing Base64-encoded data.

| type        |
| ----------- |
| **string**. |

### BlankNode

An RDF [blank node](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node).

| property     | description                |
| ------------ | -------------------------- |
| `blank_node` | _(Required.) _ **string**. |

### ConstructQuery

Represents a CONSTRUCT query over the triple store, allowing to retrieve a set of triples serialized in a specific format.

| property    | description                                                                                                                                                                        |
| ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `construct` | _(Required.) _ **Array&lt;[TriplePattern](#triplepattern)&gt;**. The triples to construct. If nothing is provided, the patterns from the `where` clause are used for construction. |
| `prefixes`  | _(Required.) _ **Array&lt;[Prefix](#prefix)&gt;**. The prefixes used in the query.                                                                                                 |
| `where`     | _(Required.) _ **Array&lt;[WhereCondition](#wherecondition)&gt;**. The WHERE clause. This clause is used to specify the triples to construct using variable bindings.              |

### DataFormat

Represents the format in which the data are serialized, for example when returned by a query or when inserted in the store.

| variant                 | description                                                                                   |
| ----------------------- | --------------------------------------------------------------------------------------------- |
| [RDF XML](#rdf-xml)     | **string**: `rdf_xml`. Output in [RDF/XML](https://www.w3.org/TR/rdf-syntax-grammar/) format. |
| [Turtle](#turtle)       | **string**: `turtle`. Output in [Turtle](https://www.w3.org/TR/turtle/) format.               |
| [N-Triples](#n-triples) | **string**: `n_triples`. Output in [N-Triples](https://www.w3.org/TR/n-triples/) format.      |
| [N-Quads](#n-quads)     | **string**: `n_quads`. Output in [N-Quads](https://www.w3.org/TR/n-quads/) format.            |

### DescribeQuery

Represents a DESCRIBE query over the triple store, allowing to retrieve a description of a resource as a set of triples serialized in a specific format.

| property   | description                                                                                                                                                                      |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prefixes` | _(Required.) _ **Array&lt;[Prefix](#prefix)&gt;**. The prefixes used in the query.                                                                                               |
| `resource` | _(Required.) _ **[VarOrNamedNode](#varornamednode)**. The resource to describe given as a variable or a node.                                                                    |
| `where`    | _(Required.) _ **Array&lt;[WhereCondition](#wherecondition)&gt;**. The WHERE clause. This clause is used to specify the resource identifier to describe using variable bindings. |

### Full

A full IRI.

| property | description                |
| -------- | -------------------------- |
| `full`   | _(Required.) _ **string**. |

### Head

Represents the head of a [SelectResponse].

| property | description                                                                  |
| -------- | ---------------------------------------------------------------------------- |
| `vars`   | _(Required.) _ **Array&lt;string&gt;**. The variables selected in the query. |

### IRI

Represents an IRI.

| variant               | description                                                                                                                                                                                                                                          |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Prefixed](#prefixed) | **object**. An IRI prefixed with a prefix. The prefixed IRI is expanded to a full IRI using the prefix definition specified in the query. For example, the prefixed IRI `rdf:type` is expanded to `http://www.w3.org/1999/02/22-rdf-syntax-ns#type`. |
| [Full](#full)         | **object**. A full IRI.                                                                                                                                                                                                                              |

### LanguageTaggedString

A [language-tagged string](https://www.w3.org/TR/rdf11-concepts/#dfn-language-tagged-string)

| property                          | description                                                                                            |
| --------------------------------- | ------------------------------------------------------------------------------------------------------ |
| `language_tagged_string`          | _(Required.) _ **object**.                                                                             |
| `language_tagged_string.language` | _(Required.) _ **string**. The [language tag](https://www.w3.org/TR/rdf11-concepts/#dfn-language-tag). |
| `language_tagged_string.value`    | _(Required.) _ **string**. The [lexical form](https://www.w3.org/TR/rdf11-concepts/#dfn-lexical-form). |

### Literal

An RDF [literal](https://www.w3.org/TR/rdf11-concepts/#dfn-literal).

| variant                                       | description                                                                                                                 |
| --------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| [Simple](#simple)                             | **object**. A [simple literal](https://www.w3.org/TR/rdf11-concepts/#dfn-simple-literal) without datatype or language form. |
| [LanguageTaggedString](#languagetaggedstring) | **object**. A [language-tagged string](https://www.w3.org/TR/rdf11-concepts/#dfn-language-tagged-string)                    |
| [TypedValue](#typedvalue)                     | **object**. A value with a datatype.                                                                                        |

### N-Quads

Output in [N-Quads](https://www.w3.org/TR/n-quads/) format.

| literal     |
| ----------- |
| `"n_quads"` |

### N-Triples

Output in [N-Triples](https://www.w3.org/TR/n-triples/) format.

| literal       |
| ------------- |
| `"n_triples"` |

### NamedNode

An RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri).

| property     | description                                              |
| ------------ | -------------------------------------------------------- |
| `named_node` | _(Required.) _ **[Prefixed](#prefixed)\|[Full](#full)**. |

### Node

Represents either an IRI (named node) or a blank node.

| variant                 | description                                                                            |
| ----------------------- | -------------------------------------------------------------------------------------- |
| [NamedNode](#namednode) | **object**. An RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri).               |
| [BlankNode](#blanknode) | **object**. An RDF [blank node](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node). |

### Prefix

Represents a prefix, i.e. a shortcut for a namespace used in a query.

| property    | description                                                          |
| ----------- | -------------------------------------------------------------------- |
| `namespace` | _(Required.) _ **string**. The namespace associated with the prefix. |
| `prefix`    | _(Required.) _ **string**. The prefix.                               |

### Prefixed

An IRI prefixed with a prefix. The prefixed IRI is expanded to a full IRI using the prefix definition specified in the query. For example, the prefixed IRI `rdf:type` is expanded to `http://www.w3.org/1999/02/22-rdf-syntax-ns#type`.

| property   | description                |
| ---------- | -------------------------- |
| `prefixed` | _(Required.) _ **string**. |

### RDF XML

Output in [RDF/XML](https://www.w3.org/TR/rdf-syntax-grammar/) format.

| literal     |
| ----------- |
| `"rdf_xml"` |

### Results

Represents the results of a [SelectResponse].

| property   | description                                                          |
| ---------- | -------------------------------------------------------------------- |
| `bindings` | _(Required.) _ **Array&lt;object&gt;**. The bindings of the results. |

### SelectItem

Represents an item to select in a [SelectQuery].

| variant               | description                        |
| --------------------- | ---------------------------------- |
| [Variable](#variable) | **object**. Represents a variable. |

### SelectQuery

Represents a SELECT query over the triple store, allowing to select variables to return and to filter the results.

| property   | description                                                                                                                                                                                          |
| ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `limit`    | **integer\|null**. The maximum number of results to return. If `None`, there is no limit. Note: the value of the limit cannot exceed the maximum query limit defined in the store limitations.       |
| `prefixes` | _(Required.) _ **Array&lt;[Prefix](#prefix)&gt;**. The prefixes used in the query.                                                                                                                   |
| `select`   | _(Required.) _ **Array&lt;[SelectItem](#selectitem)&gt;**. The items to select. Note: the number of items to select cannot exceed the maximum query variable count defined in the store limitations. |
| `where`    | _(Required.) _ **Array&lt;[WhereCondition](#wherecondition)&gt;**. The WHERE clause. If `None`, there is no WHERE clause, i.e. all triples are returned without filtering.                           |

### Simple

A [simple literal](https://www.w3.org/TR/rdf11-concepts/#dfn-simple-literal) without datatype or language form.

| property | description                |
| -------- | -------------------------- |
| `simple` | _(Required.) _ **string**. |

### SimpleWhereCondition

Represents a simple condition in a [WhereCondition].

| variant                         | description                                                                                                       |
| ------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| [TriplePattern](#triplepattern) | **object**. Represents a triple pattern, i.e. a condition on a triple based on its subject, predicate and object. |

### StoreLimits

Contains limitations regarding store usages.

| property                       | description                                                                                                                                                                                                                                                                                                                                          |
| ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `max_byte_size`                | _(Required.) _ **[Uint128](#uint128)**. The maximum number of bytes the store can contain. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any.                                                                                                   |
| `max_insert_data_byte_size`    | _(Required.) _ **[Uint128](#uint128)**. The maximum number of bytes an insert data query can contain.                                                                                                                                                                                                                                                |
| `max_insert_data_triple_count` | _(Required.) _ **[Uint128](#uint128)**. The maximum number of triples an insert data query can contain (after parsing).                                                                                                                                                                                                                              |
| `max_query_limit`              | _(Required.) _ **integer**. The maximum limit of a query, i.e. the maximum number of triples returned by a select query.                                                                                                                                                                                                                             |
| `max_query_variable_count`     | _(Required.) _ **integer**. The maximum number of variables a query can select.                                                                                                                                                                                                                                                                      |
| `max_triple_byte_size`         | _(Required.) _ **[Uint128](#uint128)**. The maximum number of bytes the store can contain for a single triple. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. The limit is used to prevent storing very large triples, especially literals. |
| `max_triple_count`             | _(Required.) _ **[Uint128](#uint128)**. The maximum number of triples the store can contain.                                                                                                                                                                                                                                                         |

### StoreLimitsInput

Contains requested limitations regarding store usages.

| property                       | description                                                                                                                                                                                                                                                                                                                                                                                                      |
| ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `max_byte_size`                | **[Uint128](#uint128)**. The maximum number of bytes the store can contain. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. Default to [Uint128::MAX] if not set, which can be considered as no limit.                                                                                                   |
| `max_insert_data_byte_size`    | **[Uint128](#uint128)**. The maximum number of bytes an insert data query can contain. Default to [Uint128::MAX] if not set, which can be considered as no limit.                                                                                                                                                                                                                                                |
| `max_insert_data_triple_count` | **[Uint128](#uint128)**. The maximum number of triples an insert data query can contain (after parsing). Default to [Uint128::MAX] if not set, which can be considered as no limit.                                                                                                                                                                                                                              |
| `max_query_limit`              | **integer**. The maximum limit of a query, i.e. the maximum number of triples returned by a select query. Default to 30 if not set.                                                                                                                                                                                                                                                                              |
| `max_query_variable_count`     | **integer**. The maximum number of variables a query can select. Default to 30 if not set.                                                                                                                                                                                                                                                                                                                       |
| `max_triple_byte_size`         | **[Uint128](#uint128)**. The maximum number of bytes the store can contain for a single triple. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. The limit is used to prevent storing very large triples, especially literals. Default to [Uint128::MAX] if not set, which can be considered as no limit. |
| `max_triple_count`             | **[Uint128](#uint128)**. The maximum number of triples the store can contain. Default to [Uint128::MAX] if not set, which can be considered as no limit.                                                                                                                                                                                                                                                         |

### StoreStat

Contains usage information about the triple store.

| property          | description                                                                                     |
| ----------------- | ----------------------------------------------------------------------------------------------- |
| `byte_size`       | _(Required.) _ **[Uint128](#uint128)**. The total triple size in the store, in bytes.           |
| `namespace_count` | _(Required.) _ **[Uint128](#uint128)**. The total number of IRI namespace present in the store. |
| `triple_count`    | _(Required.) _ **[Uint128](#uint128)**. The total number of triple present in the store.        |

### TriplePattern

Represents a triple pattern in a [SimpleWhereCondition].

| property    | description                                                                                     |
| ----------- | ----------------------------------------------------------------------------------------------- |
| `object`    | _(Required.) _ **[VarOrNodeOrLiteral](#varornodeorliteral)**. The object of the triple pattern. |
| `predicate` | _(Required.) _ **[VarOrNode](#varornode)**. The predicate of the triple pattern.                |
| `subject`   | _(Required.) _ **[VarOrNode](#varornode)**. The subject of the triple pattern.                  |

### Turtle

Output in [Turtle](https://www.w3.org/TR/turtle/) format.

| literal    |
| ---------- |
| `"turtle"` |

### TypedValue

A value with a datatype.

| property               | description                                                                                                 |
| ---------------------- | ----------------------------------------------------------------------------------------------------------- |
| `typed_value`          | _(Required.) _ **object**.                                                                                  |
| `typed_value.datatype` | _(Required.) _ **[IRI](#iri)**. The [datatype IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-datatype-iri). |
| `typed_value.value`    | _(Required.) _ **string**. The [lexical form](https://www.w3.org/TR/rdf11-concepts/#dfn-lexical-form).      |

### URI

Represents an IRI.

| property | description                                           |
| -------- | ----------------------------------------------------- |
| `type`   | _(Required.) _ **string**.                            |
| `value`  | _(Required.) _ **[IRI](#iri)**. The value of the IRI. |

### Uint128

A string containing a 128-bit integer in decimal representation.

| type        |
| ----------- |
| **string**. |

### Value

| variant                 | description                                                                        |
| ----------------------- | ---------------------------------------------------------------------------------- |
| [URI](#uri)             | **object**. Represents an IRI.                                                     |
| [Literal](#literal)     | **object**. Represents a literal S with optional language tag L or datatype IRI D. |
| [BlankNode](#blanknode) | **object**. Represents a blank node.                                               |

### VarOrNamedNode

Represents either a variable or a named node (IRI).

| variant                 | description                                                              |
| ----------------------- | ------------------------------------------------------------------------ |
| [Variable](#variable)   | **object**. A variable.                                                  |
| [NamedNode](#namednode) | **object**. An RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri). |

### VarOrNode

Represents either a variable or a node.

| variant               | description                                      |
| --------------------- | ------------------------------------------------ |
| [Variable](#variable) | **object**. A variable.                          |
| [Node](#node)         | **object**. A node, i.e. an IRI or a blank node. |

### VarOrNodeOrLiteral

Represents either a variable, a node or a literal.

| variant               | description                                                                                                                                        |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Variable](#variable) | **object**. A variable.                                                                                                                            |
| [Node](#node)         | **object**. A node, i.e. an IRI or a blank node.                                                                                                   |
| [Literal](#literal)   | **object**. An RDF [literal](https://www.w3.org/TR/rdf11-concepts/#dfn-literal), i.e. a simple literal, a language-tagged string or a typed value. |

### Variable

A variable.

| property   | description                |
| ---------- | -------------------------- |
| `variable` | _(Required.) _ **string**. |

### WhereCondition

Represents a condition in a [WhereClause].

| variant           | description                                |
| ----------------- | ------------------------------------------ |
| [Simple](#simple) | **object**. Represents a simple condition. |

---

_Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `okp4-cognitarium.json` (`0b746186a6e8df78`)_
