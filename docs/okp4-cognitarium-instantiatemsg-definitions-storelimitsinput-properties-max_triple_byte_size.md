# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimitsInput/properties/max_triple_byte_size
```

The maximum number of bytes the store can contains for a single triple. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. The limit is used to prevent storing very large triples, especially literals. Default to \[Uint128::MAX] if not set, which can be considered as no limit.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_triple\_byte\_size Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_triple_byte_size.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_triple_byte_size-allof-0.md "check type definition")

## max\_triple\_byte\_size Default Value

The default value is:

```json
"340282366920938463463374607431768211455"
```
