# Prolog query

In this example we'll see how to query the `axone-cognitarium` from Prolog programs executed by the AXONE logic module.

We'll use for that purpose the [query.pl](query.pl) sample program, multiple predicates are defined in it, we'll explore them step by step.

The query we'll make will be performed against a `axone-cognitarium` instance filled with the provided [sample data](../sample-data.rdf.xml), see the [basic example](../basic) to insert them.

## Forge the CosmWasm query

As seen in a [axone-law-stone example](../../../axone-law-stone/examples/multiple-sources), interaction with smart contracts from Prolog is based on the
interpreter virtual filesystem that'll handle dedicated cosmwasm URIs.

It's worth to mention that to query cosmwasm smart contracts and getting the raw response we'll need to set in the related URI the param `base64Decode` to `false`.

The `cosmwasm_query` predicate will help to create the cosmwasm URI, for example:

```bash
axoned query logic ask \
    --program-file query.pl \
    "cosmwasm_query(cognitarium, '${CONTRACT_ADDR}', json([key-value]), false, URI)."
```

## Call the smart contract

By calling the `cosmwasm_call` predicate with a cosmwasm URI we'll be able to get the JSON response, let's try it with a simple `axone-cognitarium` `Store` query which returns usage information about the triple store:

```bash
axoned query logic ask \
    --program-file query.pl \
    "cosmwasm_query(cognitarium, '${CONTRACT_ADDR}', 'store', false, URI), cosmwasm_call(URI, Response)."
```

## Select query

Through the `cognitarium_dataset_tags`, we can query the tags present in metadata describing a specific dataset, for example:

```bash
axoned query logic ask \
    --program-file query.pl \
    "cognitarium_dataset_tags('${CONTRACT_ADDR}', 'https://ontology.axone.space/dataverse/dataset/0ea1fc7a-dd97-4adc-a10e-169c6597bcde', Tags)."
```

## Exploiting the response

Using the `cognitarium_dataset_has_tag` predicate we show how to define rules based on the contract response, here on the present of a certain tag:

```bash
axoned query logic ask \
    --program-file query.pl \
    "cognitarium_dataset_has_tag('${CONTRACT_ADDR}', 'https://ontology.axone.space/dataverse/dataset/0ea1fc7a-dd97-4adc-a10e-169c6597bcde', 'AwesomeData')."
```
