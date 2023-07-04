# Basic example

In this example we'll see how to instantiate the `cognitarium` smart contract, insert some sample data and make some query using the [okp4d](https://github.com/okp4/okp4d).

## Instantiate

The instantiation is pretty straightforward and can be achieved as below:

```bash
okp4d tx wasm instantiate $CODE_ID \
    --label "dataverse" \
    --from $ADDR \
    --admin $ADMIN_ADDR \
    --gas 1000000 \
    "{}"
```

To be noted that we can provide some limitation parameters to restrict usage for both execute and query messages.

## Insert

The insertion can be performed through the `InsertData` execute message which takes as input the triples data base64 encoded and its format, it supports the formats below:
- `RDFXml`
- `Turtle`
- `NTriples`
- `NQuads`

Let's try to insert the [sample data](../sample-data.rdf.xml):

```bash
okp4d tx wasm execute $CONTRACT_ADDR \
    --from $ADDR \
    --gas 1000000 \
    '{\"insert_data\":{\"format\":\"rdf_xml\",\"data\":\"$(cat ./sample-data.rdf.xml | base64)\"}}'
```

With the transaction hash we can query the number of triples inserted:

```bash
okp4d query tx $TX_HASH -ojson | 
    jq -r '.events[] | select(.type == "wasm") | .attributes[] | select(.key == "triple_count") | .value'
```

## Select

Now that we have some triples in the `cognitarium`, let's see how we can query it. For that we can use the `Select` query message which, if you have some experience in [SPARQL](https://www.w3.org/TR/rdf-sparql-query/) you should be comfortable with it.

This query will basically take a `WHERE` condition composed of triple patterns to express constraints with variables, and `SELECT` some variables and result, let's see some examples.

The following example will query all the triples `subject`, `predicate` and `object`:

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

Let's execute this query with:

```bash
okp4d query wasm contract-state smart $CONTRACT_ADDR \
    '{"select":{"query":{"prefixes":[],"select":[{"variable":"subject"},{"variable":"predicate"},{"variable":"object"}],"where":[{"simple":{"triple_pattern":{"subject":{"variable":"subject"},"predicate":{"variable":"predicate"},"object":{"variable":"object"}}}}],"limit":null}}}'
```

Now we'll see a little more complex query:

```json
{
    "select": {
        "query": {
            "prefixes": [],
            "select": [
                {
                    "variable": "tag"
                }
            ],
            "where": [
                {
                    "simple": {
                        "triple_pattern": {
                            "subject": {
                                "variable": "dataset"
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
                                        "full": "https://ontology.okp4.space/core/Dataset"
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
                                "variable": "metadata"
                            },
                            "predicate": {
                                "node": {
                                    "named_node": {
                                        "full": "https://ontology.okp4.space/core/describes"
                                    }
                                }
                            },
                            "object": {
                                "variable": "dataset"
                            }
                        }
                    }
                },
                {
                    "simple": {
                        "triple_pattern": {
                            "subject": {
                                "variable": "metadata"
                            },
                            "predicate": {
                                "node": {
                                    "named_node": {
                                        "full": "https://ontology.okp4.space/core/hasTag"
                                    }
                                }
                            },
                            "object": {
                                "variable": "tag"
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

This will resolve the `dataset` variable to any subject being of type dataset, the `metadata` variable will then identify all the subjects who describe the datasets, and the `tag` variable will contains the metadata tags. The query will only return the values for the `tag` variable.

Let's execute this query with:

```bash
okp4d query wasm contract-state smart $CONTRACT_ADDR \
    '{"select":{"query":{"prefixes":[],"select":[{"variable":"tag"}],"where":[{"simple":{"triple_pattern":{"subject":{"variable":"dataset"},"predicate":{"node":{"named_node":{"full":"http://www.w3.org/1999/02/22-rdf-syntax-ns#type"}}},"object":{"node":{"named_node":{"full":"https://ontology.okp4.space/core/Dataset"}}}}}},{"simple":{"triple_pattern":{"subject":{"variable":"metadata"},"predicate":{"node":{"named_node":{"full":"https://ontology.okp4.space/core/describes"}}},"object":{"variable":"dataset"}}}},{"simple":{"triple_pattern":{"subject":{"variable":"metadata"},"predicate":{"node":{"named_node":{"full":"https://ontology.okp4.space/core/hasTag"}}},"object":{"variable":"tag"}}}}],"limit":null}}}'
```

We can modify this query to integrate a `limit` field which will restrict the result count, and the `prefixes` field to simplify the writing of named nodes in patterns:

```json
{
    "select": {
        "query": {
            "prefixes": [
                {
                    "prefix": "rdf",
                    "namespace": "http://www.w3.org/1999/02/22-rdf-syntax-ns#"
                },
                {
                    "prefix": "okp4",
                    "namespace": "https://ontology.okp4.space/core/"
                }
            ],
            "select": [
                {
                    "variable": "tag"
                }
            ],
            "where": [
                {
                    "simple": {
                        "triple_pattern": {
                            "subject": {
                                "variable": "dataset"
                            },
                            "predicate": {
                                "node": {
                                    "named_node": {
                                        "prefixed": "rdf:type"
                                    }
                                }
                            },
                            "object": {
                                "node": {
                                    "named_node": {
                                        "prefixed": "okp4:Dataset"
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
                                "variable": "metadata"
                            },
                            "predicate": {
                                "node": {
                                    "named_node": {
                                        "prefixed": "okp4:describes"
                                    }
                                }
                            },
                            "object": {
                                "variable": "dataset"
                            }
                        }
                    }
                },
                {
                    "simple": {
                        "triple_pattern": {
                            "subject": {
                                "variable": "metadata"
                            },
                            "predicate": {
                                "node": {
                                    "named_node": {
                                        "prefixed": "okp4:hasTag"
                                    }
                                }
                            },
                            "object": {
                                "variable": "tag"
                            }
                        }
                    }
                }
            ],
            "limit": 2
        }
    }
}
```

And the corresponding command:

```bash
okp4d query wasm contract-state smart $CONTRACT_ADDR \
    '{"select":{"query":{"prefixes":[{"prefix":"rdf","namespace":"http://www.w3.org/1999/02/22-rdf-syntax-ns#"},{"prefix":"okp4","namespace":"https://ontology.okp4.space/core/"}],"select":[{"variable":"tag"}],"where":[{"simple":{"triple_pattern":{"subject":{"variable":"dataset"},"predicate":{"node":{"named_node":{"prefixed":"rdf:type"}}},"object":{"node":{"named_node":{"prefixed":"okp4:Dataset"}}}}}},{"simple":{"triple_pattern":{"subject":{"variable":"metadata"},"predicate":{"node":{"named_node":{"prefixed":"okp4:describes"}}},"object":{"variable":"dataset"}}}},{"simple":{"triple_pattern":{"subject":{"variable":"metadata"},"predicate":{"node":{"named_node":{"prefixed":"okp4:hasTag"}}},"object":{"variable":"tag"}}}}],"limit":2}}}'
```
