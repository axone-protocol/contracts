# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/SelectQuery/properties/limit
```

The maximum number of results to return. If `None`, there is no limit. Note: the value of the limit cannot exceed the maximum query limit defined in the store limitations.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## limit Type

`integer`

## limit Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
