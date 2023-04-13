# InstantiateMsg Schema

```txt
undefined#/instantiate
```

Instantiate message

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-triplestore.json\*](schema/okp4-triplestore.json "open original schema") |

## instantiate Type

`object` ([InstantiateMsg](okp4-triplestore-instantiatemsg.md))

# instantiate Properties

| Property          | Type   | Required | Nullable       | Defined by                                                                                                          |
| :---------------- | :----- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------ |
| [limits](#limits) | Merged | Required | cannot be null | [okp4-triplestore](okp4-triplestore-instantiatemsg-properties-limits.md "undefined#/instantiate/properties/limits") |

## limits

Limitations regarding store usage.

`limits`

*   is required

*   Type: merged type ([Details](okp4-triplestore-instantiatemsg-properties-limits.md))

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-instantiatemsg-properties-limits.md "undefined#/instantiate/properties/limits")

### limits Type

merged type ([Details](okp4-triplestore-instantiatemsg-properties-limits.md))

all of

*   [Untitled undefined type in okp4-triplestore](okp4-triplestore-instantiatemsg-properties-limits-allof-0.md "check type definition")

# InstantiateMsg Definitions

## Definitions group StoreLimits

Reference this group by using

```json
{"$ref":"undefined#/instantiate/definitions/StoreLimits"}
```

| Property                                | Type   | Required | Nullable       | Defined by                                                                                                                                                                              |
| :-------------------------------------- | :----- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [max\_triple\_count](#max_triple_count) | Merged | Optional | cannot be null | [okp4-triplestore](okp4-triplestore-instantiatemsg-definitions-storelimits-properties-max_triple_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_triple_count") |

### max\_triple\_count

The maximum number of triples the store can contains. If `None`, there is no limit on the number of triples.

`max_triple_count`

*   is optional

*   Type: merged type ([Details](okp4-triplestore-instantiatemsg-definitions-storelimits-properties-max_triple_count.md))

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-instantiatemsg-definitions-storelimits-properties-max_triple_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_triple_count")

#### max\_triple\_count Type

merged type ([Details](okp4-triplestore-instantiatemsg-definitions-storelimits-properties-max_triple_count.md))

any of

*   [Untitled undefined type in okp4-triplestore](okp4-triplestore-instantiatemsg-definitions-storelimits-properties-max_triple_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-triplestore](okp4-triplestore-instantiatemsg-definitions-storelimits-properties-max_triple_count-anyof-1.md "check type definition")

## Definitions group Uint128

Reference this group by using

```json
{"$ref":"undefined#/instantiate/definitions/Uint128"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
