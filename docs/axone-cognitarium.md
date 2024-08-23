# Cognitarium

A [CosmWasm](https://cosmwasm.com/) Smart Contract which enables the storage and querying of Semantic data using [RDF (Resource Description Framework)](https://en.wikipedia.org/wiki/Resource_Description_Framework), which represents information as semantic triples.

## Purpose

The Smart Contract operates as a [semantic database](https://en.wikipedia.org/wiki/Triplestore), adept at storing and fetching [RDF triples](https://en.wikipedia.org/wiki/Semantic_triple) via semantic queries. It can be deployed on any blockchain within the [Cosmos blockchains](https://cosmos.network/) network, utilizing the [CosmWasm](https://cosmwasm.com/) framework.

The key features of the contract include:

**Insertion of RDF Triples:**
This functionality enables the insertion of new data in the form of [RDF triples](https://en.wikipedia.org/wiki/Semantic_triple) onto the blockchain, ensuring secure and tamper-proof storage. The Smart Contract supports inserting these triples in various serialization formats including [RDF/XML](https://en.wikipedia.org/wiki/RDF/XML), [Turtle](https://www.w3.org/TR/turtle/), [N-Triples](https://www.w3.org/TR/n-triples/) and [N-Quads](https://www.w3.org/TR/n-quads/).

**Removal of RDF Triples:**
This functionality enables the selective deletion of RDF triples from the on-chain store. Users can specify patterns or criteria that identify the triples to be removed, ensuring precise and targeted removal of data.

**Querying of RDF Triples:**
The Smart Contract provides powerful on-chain querying capabilities, allowing users to retrieve specific RDF triples stored on the blockchain. This is done using a variation of [SPARQL](https://www.w3.org/TR/sparql11-query/), a specialized query language designed for retrieving and manipulating data stored in RDF format. Users can specify their search criteria in the query, and the Smart Contract will return the matching RDF triples, directly accessing the on-chain data. This feature supports various serialization formats for the output, such as Turtle, N-Triples, N-Quads, and RDF/XML, offering flexibility in how the retrieved data is presented and used.

**Policies of the Store:**
The Smart Contract includes a straightforward yet effective policies functionality to manage the capacity and usage of the on-chain storage effectively. These policies ensure efficient operation and prevent misuse or overuse of the Smart Contract. For instance:

- Maximum Triples: Caps the total number of RDF triples the store can hold, preventing database overload.
- Storage Size Limit: Sets an upper bound on the store's data size in bytes, managing blockchain resource use.
- Query Size Limit: Restricts the size or complexity of queries to maintain fast and reliable data retrieval.
- Insert Data Limit: Limits the size of data (in bytes) that can be added in a single transaction, ensuring smooth and efficient data insertion.

## Rationale

The data preserved in the blockchain holds significant value due to its derivation from a distributed consensus, rendering it a reliable source for decision-making, applicable to both on-chain and off-chain scenarios.

To effectively utilize this data, it's essential to adopt representation models that cater to diverse requirements. The Smart Contract Cognitarium provides such a model, facilitating the depiction of intricate and evolving semantic connections within a highly interconnected dataset. This approach transforms the data into a Knowledge Graph, enabling an accurate portrayal of existing facts and fostering the generation of new insights.

## Play

### Model your data with RDF

[RDF](https://www.w3.org/RDF/) encodes information in triple structures. The basic structure of an RDF triple is `subject-predicate-object`, much like a simple sentence in the English language.

1. **Subject**: The subject is the entity or resource the statement is about. It's typically a URI ([Uniform Resource Identifier](https://en.wikipedia.org/wiki/Uniform_Resource_Identifier)) which uniquely identifies a resource.
2. **Predicate**: The predicate (also called a property) is a specific aspect, characteristic, attribute, or relation that describes the subject. It's also typically a URI.
3. **Object**: The object is the value of the attribute defined by the predicate for the subject. It can be a URI or a literal (such as a string or a number) and may also include additional information such as a language tag or a datatype.

In RDF, **prefixes** are used as a shorthand notation for long URIs to make the data more readable and less verbose. They're similar to namespaces in programming languages. For instance, instead of writing `http://www.w3.org/2001/XMLSchema#integer`, you could declare a prefix `xsd` to represent the `http://www.w3.org/2001/XMLSchema#` URI and then use `xsd:integer`.

[Turtle (Terse RDF Triple Language)](https://www.w3.org/TR/turtle/) is a syntax that allows RDF to be completely written in a compact and natural text form, with abbreviations for common usage patterns and datatypes.

Here's an RDF triple written in Turtle format (`.ttl` file):

```turtle
@prefix ex: <http://example.com/stuff/1.0/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:Alice ex:hasAge "30"^^xsd:integer .
```

In this example:

- **`ex:Alice`** is the subject (using `ex` as a prefix for the `http://example.com/stuff/1.0/` URI).
- **`ex:hasAge`** is the predicate.
- **`"30"^^xsd:integer`** is the object, a literal of datatype integer (using **`xsd`** as a prefix for the XML Schema Datatype namespace).

In the Turtle syntax, the semicolon (**`;`**) is used as a shorthand to reduce verbosity when multiple predicates and objects have the same subject. It allows you to write multiple predicates and objects for the same subject without having to repeat the subject.
The comma (**`,`**) is used as a shorthand for reducing verbosity when the same subject and predicate have multiple objects.

Suppose we want to express that Alice is 30 years old person, and her email is `alice@example.com`:

```turtle
@prefix ex: <http://example.com/stuff/1.0/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:Alice a <http://www.w3.org/2002/07/owl#Person> ;
         ex:hasAge "30"^^xsd:integer ;
         ex:hasEmail "alice@example.com" .
```

:::tip
The lowercase "a" is a special abbreviation for the RDF type property, which states that a resource is an instance of a particular class. This is essentially equivalent to **`<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>`**, and it's used to indicate the type of a resource.
:::

The same RDF triple can be expressed in RDF/XML format (`.rdf.xml` file):

```xml
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:ex="http://example.com/stuff/1.0/"
         xmlns:xsd="http://www.w3.org/2001/XMLSchema#">
  <rdf:Description rdf:about="http://example.com/stuff/1.0/Alice">
    <rdf:type rdf:resource="http://www.w3.org/2002/07/owl#Person"/>
    <ex:hasAge rdf:datatype="http://www.w3.org/2001/XMLSchema#integer">30</ex:hasAge>
    <ex:hasEmail>alice@example.com</ex:hasEmail>
  </rdf:Description>
</rdf:RDF>
```

### Instantiate the Smart Contract

Let's initiate a new instance of Smart Contract and input some RDF triples into it. The `axone-cognitarium` can be set up in the following manner. Please consult the schema for additional details regarding configuration settings.

```bash
axoned tx wasm instantiate $CODE_ID \
    --from $ADDR \
    --label "my-rdf-storage" \
    --admin $ADMIN_ADDR \
    --gas 1000000 \
    '{}'
```

:::tip
You can provide some limitation parameters to restrict usage for both execute and query messages. For instance, you can set a maximum number of triples that can be stored in the smart contract, or a maximum size of data that can be inserted in a single transaction.

The default values are:

```json
{
  "limits": {
    "max_byte_size": "340282366920938463463374607431768211455",
    "max_insert_data_byte_size": "340282366920938463463374607431768211455",
    "max_insert_data_triple_count": "340282366920938463463374607431768211455",
    "max_query_limit": 30,
    "max_query_variable_count": 30,
    "max_triple_byte_size": "340282366920938463463374607431768211455",
    "max_triple_count": "340282366920938463463374607431768211455"
  }
}
```

:::

### Insert RDF triples

To insert RDF triples, you need to send an `InsertData` message through the `cognitarium` smart contract you've already instantiated. For this operation, your inputs should include the data of the triples, encoded in [base64](https://en.wikipedia.org/wiki/Base64), as well as the format. The format options available are:

- `turtle` (default)
- `rdf_xml`
- `n_triples`
- `n_quads`

Let's consider the following example of data in Turtle format, contained within a file named `data.ttl`. It describes a small network of people and their relationships, such as name, title, and whom they know.

```turtle
@prefix : <http://example.org/> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix schema: <http://schema.org/> .

:alice a foaf:Person ;
  foaf:name "Alice" ;
  foaf:knows :bob ;
  schema:email "alice@example.org" .

:bob a foaf:Person ;
  foaf:name "Bob" ;
  foaf:knows :alice, :carol ;
  schema:jobTitle "Software Developer" .

:carol a foaf:Person ;
  foaf:name "Carol" ;
  schema:jobTitle "Data Scientist" ;
  foaf:knows :bob .
```

You can insert this data into the `cognitarium` smart contract with the following command:

```bash
axoned tx wasm execute $CONTRACT_ADDR \
    --from axone1cu9wzlcyyxpek20jaqfwzu3llzjgx34cqf94yj \
    --gas 10000000 \
    "{\"insert_data\":{\"format\": \"turtle\", \"data\": \"$(cat data.ttl | base64 | tr -d '\n\r')\"}}"
```

With the transaction hash we can query the number of triples inserted:

```bash
axoned query tx $TX_HASH -ojson |
    jq -r '.events[] | select(.type == "wasm") | .attributes[] | select(.key == "triple_count") | .value'
```

### Query RDF triples

Now that we've populated the axone-cognitarium with several triples, let's explore how to retrieve this data. We can utilize the Select query message for this purpose. If you're familiar with [SPARQL](https://www.w3.org/TR/rdf-sparql-query/), you'll find the process quite intuitive.

A `select` query on a `cognitarium` instance enables you to fetch and filter the data. The `select.query` JSON should contain the following:

- `prefixes` array: to declare a `prefix` and its related `namespace`
- `limit`: the number of elements to return
- `where`: filters and variable declarations
- `select` array: all `variable` names you declared in `where` you want to get

`where` should be an array of elements specifying triple filterings. You have to specify `subject`, `predicate` and `object` as a `variable`, or, alternatively, a `prefixed` or `full` `named_node`.

`object` can also be a `simple` `literal`.

The following query will select all the triples `subject`, `predicate` and `object` from the store:

```json
{
  "select": {
    "query": {
      "prefixes": [],
      "select": [
        {
          "variable": "subject"
        },
        {
          "variable": "predicate"
        },
        {
          "variable": "object"
        }
      ],
      "where": [
        {
          "simple": {
            "triple_pattern": {
              "subject": {
                "variable": "subject"
              },
              "predicate": {
                "variable": "predicate"
              },
              "object": {
                "variable": "object"
              }
            }
          }
        }
      ],
      "limit": null
    }
  }
}
```

It's semantically equivalent to the following SPARQL query:

```sparql
SELECT ?subject ?predicate ?object
WHERE {
    ?subject ?predicate ?object
}
```

This query can be executed on the cognitarium smart contract using the command below:

```bash
axoned query wasm contract-state smart $CONTRACT_ADDR \
    '{"select":{"query":{"prefixes":[],"select":[{"variable":"subject"},{"variable":"predicate"},{"variable":"object"}],"where":[{"simple":{"triple_pattern":{"subject":{"variable":"subject"},"predicate":{"variable":"predicate"},"object":{"variable":"object"}}}}],"limit":null}}}'
```

Now, let's try something more interresting. Let's retrieve the names of people and their job titles, but only for those who know at least one other person in the network. This query introduces filtering based on relationships.

Here's the query:

```json
{
  "select": {
    "query": {
      "prefixes": [
        { "foaf": "http://xmlns.com/foaf/0.1/" },
        { "schema": "http://schema.org/" }
      ],
      "select": [
        {
          "variable": "personName"
        },
        {
          "variable": "jobTitle"
        }
      ],
      "where": [
        {
          "simple": {
            "triple_pattern": {
              "subject": {
                "variable": "person"
              },
              "predicate": {
                "node": {
                  "named_node": {
                    "full": "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
                  }
                }
              },
              "object": {
                "node": {
                  "named_node": {
                    "prefixed": "foaf:Person"
                  }
                }
              }
            }
          }
        },
        {
          "simple": {
            "triple_pattern": {
              "subject": {
                "variable": "person"
              },
              "predicate": {
                "node": {
                  "named_node": {
                    "prefixed": "foaf:Name"
                  }
                }
              },
              "object": {
                "variable": "personName"
              }
            }
          }
        },
        {
          "simple": {
            "triple_pattern": {
              "subject": {
                "variable": "person"
              },
              "predicate": {
                "node": {
                  "named_node": {
                    "prefixed": "schema:jobTitle"
                  }
                }
              },
              "object": {
                "variable": "jobTitle"
              }
            }
          }
        },
        {
          "simple": {
            "triple_pattern": {
              "subject": {
                "variable": "person"
              },
              "predicate": {
                "node": {
                  "named_node": {
                    "prefixed": "foaf:knows"
                  }
                }
              },
              "object": {
                "variable": "knownPerson"
              }
            }
          }
        }
      ],
      "limit": null
    }
  }
}
```

It's semantically equivalent to the following SPARQL query:

```sparql
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
PREFIX schema: <http://schema.org/>

SELECT ?personName ?jobTitle
WHERE {
  ?person a foaf:Person .
  ?person foaf:name ?personName .
  ?person schema:jobTitle ?jobTitle .
  ?person foaf:knows ?knownPerson .
}
```

This query can be executed on the cognitarium smart contract using the command below:

```bash
axoned query wasm contract-state smart $CONTRACT_ADDR \
    '{"select":{"query":{"prefixes":[{"foaf":"http://xmlns.com/foaf/0.1/"},{"schema":"http://schema.org/"}],"select":[{"variable":"personName"},{"variable":"jobTitle"}],"where":[{"simple":{"triple_pattern":{"subject":{"variable":"person"},"predicate":{"node":{"named_node":{"full":"http://www.w3.org/1999/02/22-rdf-syntax-ns#type"}}},"object":{"node":{"named_node":{"prefixed":"foaf:Person"}}}}}},{"simple":{"triple_pattern":{"subject":{"variable":"person"},"predicate":{"node":{"named_node":{"prefixed":"foaf:Name"}}},"object":{"variable":"personName"}}}},{"simple":{"triple_pattern":{"subject":{"variable":"person"},"predicate":{"node":{"named_node":{"prefixed":"schema:jobTitle"}}},"object":{"variable":"jobTitle"}}}},{"simple":{"triple_pattern":{"subject":{"variable":"person"},"predicate":{"node":{"named_node":{"prefixed":"foaf:knows"}}},"object":{"variable":"knownPerson"}}}}],"limit":null}}}'
```

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

Example: `json { "prefixes": [ { "prefix": "foaf", "namespace": "http://xmlns.com/foaf/0.1/" } ], "delete": [ { "subject": { "variable": "s" }, "predicate": { "variable": "p" }, "object": { "variable": "o" } } ], "where": [ { "simple": { "triplePattern": { "subject": { "variable": "s" }, "predicate": { "namedNode": {"prefixed": "foaf:givenName"} }, "object": { "literal": { "simple": "Myrddin" } } } } }, { "simple": { "triplePattern": { "subject": { "variable": "s" }, "predicate": { "variable": "p" }, "object": { "variable": "o" } } } } ] `

Only the smart contract owner (i.e. the address who instantiated it) is authorized to perform this action.

| parameter              | description                                                                                                                                                                                                                          |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `delete_data`          | _(Required.) _ **object**.                                                                                                                                                                                                           |
| `delete_data.delete`   | _(Required.) _ **Array&lt;[TripleDeleteTemplate](#tripledeletetemplate)&gt;**. Specifies the specific triple templates to delete. If nothing is provided and the `where` clause is a single Bgp, the patterns are used for deletion. |
| `delete_data.prefixes` | _(Required.) _ **Array&lt;[Prefix](#prefix)&gt;**. The prefixes used in the operation.                                                                                                                                               |
| `delete_data.where`    | **[WhereClause](#whereclause)\|null**. Defines the patterns that data (RDF triples) should match in order for it to be considered for deletion, if any.                                                                              |

## QueryMsg

Query messages

### QueryMsg::Store

Returns information about the triple store.

| parameter | description                |
| --------- | -------------------------- |
| `store`   | _(Required.) _ **object**. |

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

### Bgp

Represents a basic graph pattern expressed as a set of triple patterns.

| property       | description                                                      |
| -------------- | ---------------------------------------------------------------- |
| `bgp`          | _(Required.) _ **object**.                                       |
| `bgp.patterns` | _(Required.) _ **Array&lt;[TriplePattern](#triplepattern)&gt;**. |

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

| property    | description                                                                                                                                                                                                           |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `construct` | _(Required.) _ **Array&lt;[TripleConstructTemplate](#tripleconstructtemplate)&gt;**. The triples to construct. If nothing is provided and the `where` clause is a single Bgp, the patterns are used for construction. |
| `prefixes`  | _(Required.) _ **Array&lt;[Prefix](#prefix)&gt;**. The prefixes used in the query.                                                                                                                                    |
| `where`     | _(Required.) _ **[WhereClause](#whereclause)**. The WHERE clause. This clause is used to specify the triples to construct using variable bindings.                                                                    |

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

| property   | description                                                                                                                                          |
| ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prefixes` | _(Required.) _ **Array&lt;[Prefix](#prefix)&gt;**. The prefixes used in the query.                                                                   |
| `resource` | _(Required.) _ **[VarOrNamedNode](#varornamednode)**. The resource to describe given as a variable or a node.                                        |
| `where`    | **[WhereClause](#whereclause)\|null**. The WHERE clause. This clause is used to specify the resource identifier to describe using variable bindings. |

### Expression

Represents a logical combination of operations whose evaluation results in a term.

| variant   | description                                                                                                                                                                        |
| --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| undefined | **object**. A named node constant.                                                                                                                                                 |
| undefined | **object**. A literal constant.                                                                                                                                                    |
| undefined | **object**. A variable that must be bound for evaluation.                                                                                                                          |
| undefined | **object**. Logical conjunction of expressions. All expressions must evaluate to true for the conjunction to be true. If the conjunction is empty, it is considered true.          |
| undefined | **object**. Logical disjunction of expressions. At least one expression must evaluate to true for the disjunction to be true. If the disjunction is empty, it is considered false. |
| undefined | **object**. Equality comparison.                                                                                                                                                   |
| undefined | **object**. Greater than comparison.                                                                                                                                               |
| undefined | **object**. Greater or equal comparison.                                                                                                                                           |
| undefined | **object**. Less than comparison.                                                                                                                                                  |
| undefined | **object**. Less or equal comparison.                                                                                                                                              |
| undefined | **object**. Negation of an expression.                                                                                                                                             |

### Filter

Filters the inner clause matching the expression. The solutions coming from the inner clause that do not match the expression are discarded. The variables provided in the inner clause are available in the filter expression.

| property       | description                                                                                                |
| -------------- | ---------------------------------------------------------------------------------------------------------- |
| `filter`       | _(Required.) _ **object**.                                                                                 |
| `filter.expr`  | _(Required.) _ **object\|object\|object\|object\|object\|object\|object\|object\|object\|object\|object**. |
| `filter.inner` | _(Required.) _ **[Bgp](#bgp)\|[LateralJoin](#lateraljoin)\|[Filter](#filter)**.                            |

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

### LateralJoin

Evaluates right for all result row of left

| property             | description                                                                     |
| -------------------- | ------------------------------------------------------------------------------- |
| `lateral_join`       | _(Required.) _ **object**.                                                      |
| `lateral_join.left`  | _(Required.) _ **[Bgp](#bgp)\|[LateralJoin](#lateraljoin)\|[Filter](#filter)**. |
| `lateral_join.right` | _(Required.) _ **[Bgp](#bgp)\|[LateralJoin](#lateraljoin)\|[Filter](#filter)**. |

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
| `where`    | _(Required.) _ **[WhereClause](#whereclause)**. The WHERE clause. If `None`, there is no WHERE clause, i.e. all triples are returned without filtering.                                              |

### Simple

A [simple literal](https://www.w3.org/TR/rdf11-concepts/#dfn-simple-literal) without datatype or language form.

| property | description                |
| -------- | -------------------------- |
| `simple` | _(Required.) _ **string**. |

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

### TripleConstructTemplate

Represents a triple template to be forged for a construct query.

| property    | description                                                                                     |
| ----------- | ----------------------------------------------------------------------------------------------- |
| `object`    | _(Required.) _ **[VarOrNodeOrLiteral](#varornodeorliteral)**. The object of the triple pattern. |
| `predicate` | _(Required.) _ **[VarOrNamedNode](#varornamednode)**. The predicate of the triple pattern.      |
| `subject`   | _(Required.) _ **[VarOrNode](#varornode)**. The subject of the triple pattern.                  |

### TripleDeleteTemplate

Represents a triple template to be deleted.

| property    | description                                                                                               |
| ----------- | --------------------------------------------------------------------------------------------------------- |
| `object`    | _(Required.) _ **[VarOrNamedNodeOrLiteral](#varornamednodeorliteral)**. The object of the triple pattern. |
| `predicate` | _(Required.) _ **[VarOrNamedNode](#varornamednode)**. The predicate of the triple pattern.                |
| `subject`   | _(Required.) _ **[VarOrNamedNode](#varornamednode)**. The subject of the triple pattern.                  |

### TriplePattern

Represents a triple pattern in a [SimpleWhereCondition].

| property    | description                                                                                     |
| ----------- | ----------------------------------------------------------------------------------------------- |
| `object`    | _(Required.) _ **[VarOrNodeOrLiteral](#varornodeorliteral)**. The object of the triple pattern. |
| `predicate` | _(Required.) _ **[VarOrNamedNode](#varornamednode)**. The predicate of the triple pattern.      |
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

### VarOrNamedNodeOrLiteral

Represents either a variable, a named node or a literal.

| variant                 | description                                                                                                                                        |
| ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Variable](#variable)   | **object**. A variable.                                                                                                                            |
| [NamedNode](#namednode) | **object**. An RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri).                                                                           |
| [Literal](#literal)     | **object**. An RDF [literal](https://www.w3.org/TR/rdf11-concepts/#dfn-literal), i.e. a simple literal, a language-tagged string or a typed value. |

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

### WhereClause

Represents a WHERE clause, i.e. a set of conditions to filter the results.

| variant                     | description                                                                                                                                                                                                                                 |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Bgp](#bgp)                 | **object**. Represents a basic graph pattern expressed as a set of triple patterns.                                                                                                                                                         |
| [LateralJoin](#lateraljoin) | **object**. Evaluates right for all result row of left                                                                                                                                                                                      |
| [Filter](#filter)           | **object**. Filters the inner clause matching the expression. The solutions coming from the inner clause that do not match the expression are discarded. The variables provided in the inner clause are available in the filter expression. |

### undefined

A named node constant.

| property     | description                                              |
| ------------ | -------------------------------------------------------- |
| `named_node` | _(Required.) _ **[Prefixed](#prefixed)\|[Full](#full)**. |

---

_Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `axone-cognitarium.json` (`a6344c92b24801fb`)_
