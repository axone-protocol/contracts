# StoreLimits Schema

```txt
undefined#/instantiate/definitions/StoreLimits
```

Contains limitations regarding store usages.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-triplestore.json\*](schema/okp4-triplestore.json "open original schema") |

## StoreLimits Type

`object` ([StoreLimits](okp4-triplestore-instantiatemsg-definitions-storelimits.md))

# StoreLimits Properties

| Property                                | Type   | Required | Nullable       | Defined by                                                                                                                                                                              |
| :-------------------------------------- | :----- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [max\_triple\_count](#max_triple_count) | Merged | Optional | cannot be null | [okp4-triplestore](okp4-triplestore-instantiatemsg-definitions-storelimits-properties-max_triple_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_triple_count") |

## max\_triple\_count

The maximum number of triples the store can contains. If `None`, there is no limit on the number of triples.

`max_triple_count`

*   is optional

*   Type: merged type ([Details](okp4-triplestore-instantiatemsg-definitions-storelimits-properties-max_triple_count.md))

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-instantiatemsg-definitions-storelimits-properties-max_triple_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_triple_count")

### max\_triple\_count Type

merged type ([Details](okp4-triplestore-instantiatemsg-definitions-storelimits-properties-max_triple_count.md))

any of

*   [Untitled undefined type in okp4-triplestore](okp4-triplestore-instantiatemsg-definitions-storelimits-properties-max_triple_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-triplestore](okp4-triplestore-instantiatemsg-definitions-storelimits-properties-max_triple_count-anyof-1.md "check type definition")
