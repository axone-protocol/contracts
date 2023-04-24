# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimitsInput/properties/max_query_limit
```

The maximum limit of a query, i.e. the maximum number of triples returned by a select query. If `None`, the default value of 30 is used.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_query\_limit Type

`integer`

## max\_query\_limit Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
