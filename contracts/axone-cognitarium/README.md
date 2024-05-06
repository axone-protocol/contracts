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
