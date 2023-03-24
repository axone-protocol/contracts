# Multiple source

When executed by the logic module, a Prolog program can load other programs through the `consult(File).` predicate. This example aims to illustrate this case, when the law is composed of multiple Prolog sources.

## The Program

We'll reuse the exact same story as the [single-source](../single-source/README.md) example, we'll just split the program in two:

- `template.pl`: Contains the governance rules predicates, which can be customized by defining configuration predicates;
- `gov.pl`: Load `template.pl` and define configuration predicates.

⚠️ A special attention must be brought to the template loading with the `consult(File).` predicate.

The Logic module expects the `File` variable to be an URI so it can resolve its content. Through the `cosmwasm` prefix it can loads data from any smart contract query, we'll configure the URI to perform a `cw-storage` `ObjectData` query in order to load the `template.pl`.

The URI has the following form:

```bash
cosmwasm:{contract_name}:{contract_address}?query={contract_query}
```

Where:

- `{contract_name}`: Only informative, represents the corresponding smart contract name or type (e.g. `cw-storage`);
- `{contract_address}`: The smart contract to query, concerning the `cw-law-stone` it must be a `cw-storage` contract;
- `{contract_query}`: The JSON query to perform on the targeted smart contract, URL encoded. In our case an `ObjectData` query, for example: `%7B%22object_data%22%3A%7B%22id%22%3A%20%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D`;

## Instantiate

First the `template.pl` program must be stored on a `cw-storage` and the `gov.pl` updated with the right URI in the `consult(File).` predicate, the URI should be in the form:

```bash
cosmwasm:cw-storage:${STORAGE_ADDRESS}?query=%7B%22object_data%22%3A%7B%22id%22%3A%221a88ca1632c7323c0aa594000cda26ed9f48b36351c29c3d1e35e0a0474e862e%22%7D%7D
```

The instantiate will take as parameters the base64 encoded program and the address of a `cw-storage` contract, on which the program will be stored and pinned, the `template.pl` object will also be pinned to ensure all the needed resources stays available:

```bash
okp4d tx wasm instantiate $CODE_ID \
    --label "multiple-source" \
    --from $ADDR \
    --admin $ADMIN_ADDR \
    --gas 1000000 \
    --broadcast-mode block \
    "{\"program\":\"$(cat gov.pl | base64)\", \"storage_address\": \"$STORAGE_ADDR\"}"
```

You can retrieve the new `cw-law-stone` smart contract address in the `_contract_address` instantiate attribute of the transaction.

## Query

By using the `Ask` query we can provide Prolog predicates to be evaluated againsts the underlying programs:

```bash
okp4d query wasm contract smart $CONTRACT_ADDR \
    "{\"ask\": {\"query\": \"can('change_governance', 'did:key:okp41p8u47en82gmzfm259y6z93r9qe63l25dfwwng6').\"}}"
```

## Break

Only the smart contract admin can break the stone, if any.

The program stored in the `cw-storage` smart contract will be removed, or at least un-pinned. And the `template.pl` object will be un pinned.

By breaking the stone, you will not be able to query it anymore.

```bash
okp4d tx wasm execute $CONTRACT_ADDR \
    --from $ADDR \
    --gas 1000000 \
    --broadcast-mode block \
    '"break_stone"'
```
