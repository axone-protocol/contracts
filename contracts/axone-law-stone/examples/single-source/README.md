# Single source

This example aims to illustrate the most simple case of the `axone-law-stone`: The law is composed of only one Prolog source program.

## The Program

The spirit here is to provide a `axone-law-stone` smart contract instance providing rules similar in form to Dataspace governance rules.

You'll find in the [gov.pl](gov.pl) Prolog program some predicates defining the rules allowing to perform some typical Dataspaces actions.

The `can(Action, DID)` predicate will allow or not an action for a `did` (i.e. Decentralized Identifier), a `did` being expected to have the form: `did:example:${AXONE_ADDRESS}`. We can describe the action rules as follows:

- `change_governance`: Only the did admin can do it: `did:example:axone1p8u47en82gmzfm259y6z93r9qe63l25d858vqu`;
- `exec_workflow`: Only a valid DID having a minimum spendable of `1000000uaxone`;
- `create_dataset` Only a valid DID having a minimum spendable of `10000uaxone`;
- `create_service` Only a valid DID having a minimum spendable of `100000uaxone`;

## Instantiate

The instantiate will take as parameters the base64 encoded program and the address of a `axone-objectarium` contract, on which the program will be stored and pinned to prevent its removal and thus ensure its availability:

```bash
axoned tx wasm instantiate $CODE_ID \
    --label "single-source" \
    --from $ADDR \
    --admin $ADMIN_ADDR \
    --gas 1000000 \
    "{\"program\":\"$(cat gov.pl | base64)\", \"storage_address\": \"$STORAGE_ADDR\"}"
```

You can retrieve the new `axone-law-stone` smart contract address in the `_contract_address` instantiate attribute of the transaction.

## Query

By using the `Ask` query we can provide Prolog predicates to be evaluated againsts the underlying program:

```bash
axoned query wasm contract-state smart $CONTRACT_ADDR \
    "{\"ask\": {\"query\": \"can('change_governance', 'did:example:axone1p8u47en82gmzfm259y6z93r9qe63l25d858vqu').\"}}"
```

## Break

Only the smart contract admin can break the stone, if any.

The program stored in the `axone-objectarium` smart contract will be removed, or at least un-pinned.

By breaking the stone, you will not be able to query it anymore.

```bash
axoned tx wasm execute $CONTRACT_ADDR \
    --from $ADDR \
    --gas 1000000 \
    '"break_stone"'
```
