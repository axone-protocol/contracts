# Single source

This example aims to illustrate the most simple case of the `cw-law-stone`: The law is composed of only one Prolog source program.

## The Program

The spirit here is to provide a `cw-law-stone` smart contract instance providing rules similar in form to Dataspace governance rules.

You'll find in the [gov.pl](gov.pl) Prolog program some predicates defining the rules allowing to perform some typical Dataspaces actions.

The `can(Action, DID)` predicate will allow or not an action for a `did` (i.e. Decentralized Identifier), a `did` being expected to have the form: `did:key:${OKP4_ADDRESS}`. We can describe the action rules as follows:

- `change_governance`: Only the did admin can do it: `did:key:okp41p8u47en82gmzfm259y6z93r9qe63l25dfwwng6`;
- `exec_workflow`: Only a valid DID having a minimum spendable of `1000000uknow`;
- `create_dataset` Only a valid DID having a minimum spendable of `10000uknow`;
- `create_service` Only a valid DID having a minimum spendable of `100000uknow`;

## Instantiate

The instantiate will take as parameters the base64 encoded program and the address of a `cw-storage` contract, on which the program will be stored and pinned to prevent its removal and thus ensure its availability:

```bash
okp4d tx wasm instantiate $CODE_ID \
    --label "single-source" \
    --from $ADDR \
    --admin $ADMIN_ADDR \
    --gas 1000000 \
    --broadcast-mode block \
    "{\"program\":\"$(cat gov.pl | base64)\", \"storage_address\": \"$STORAGE_ADDR\"}"
```

You can retrieve the new `cw-law-stone` smart contract address in the `_contract_address` instantiate attribute of the transaction.

## Query

By using the `Ask` query we can provide Prolog predicates to be evaluated againsts the underlying program:

```bash
okp4d query wasm contract smart $CONTRACT_ADDR \
    "{\"ask\": {\"query\": \"can('change_governance', 'did:key:okp41p8u47en82gmzfm259y6z93r9qe63l25dfwwng6').\"}}"
```

## Break

Only the smart contract admin can break the stone, if any.

The program stored in the `cw-storage` smart contract will be removed, or at least un-pinned.

By breaking the stone, you will not be able to query it anymore.

```bash
okp4d tx wasm execute $CONTRACT_ADDR \
    --from $ADDR \
    --gas 1000000 \
    --broadcast-mode block \
    '"break_stone"'
```
