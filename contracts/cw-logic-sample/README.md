# CW Logic Sample

Sample contract to query the OKP4 logic module.

## Samples

### Instantiate

The instantiation can take a program in input:

```bash
okp4d tx wasm instantiate $CODE \
  '{"program":"bank_balances_has_coin(A, D, V, S) :- bank_balances(A, R), member(D-V, R), compare(>, V, S)."}' \
  --label "logic-binding-sample" \
  --from $MY_ADDR
```

### Query

The contract offer only an `ask` query taking a query as input:

```bash
okp4d query contract-state smart $CONTRACT_ADDR \
  '{"ask":{"query": "bank_balances_has_coin(A, 'uknow', V, 100000000)."}}' \
  --output json \
  | jq -r '.'
```
