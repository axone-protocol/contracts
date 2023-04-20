# SelectQuery Schema

```txt
undefined#/query/definitions/SelectQuery
```

Represents a SELECT query over the triple store, allowing to select variables to return and to filter the results.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## SelectQuery Type

`object` ([SelectQuery](okp4-cognitarium-querymsg-definitions-selectquery.md))

# SelectQuery Properties

| Property              | Type      | Required | Nullable       | Defined by                                                                                                                                                  |
| :-------------------- | :-------- | :------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [limit](#limit)       | `integer` | Optional | can be null    | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-limit.md "undefined#/query/definitions/SelectQuery/properties/limit")       |
| [prefixes](#prefixes) | `array`   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-prefixes.md "undefined#/query/definitions/SelectQuery/properties/prefixes") |
| [select](#select)     | `array`   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-select.md "undefined#/query/definitions/SelectQuery/properties/select")     |
| [where](#where)       | `array`   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-where.md "undefined#/query/definitions/SelectQuery/properties/where")       |

## limit

The maximum number of results to return. If `None`, there is no limit. Note: the value of the limit cannot exceed the maximum query limit defined in the store limitations.

`limit`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-limit.md "undefined#/query/definitions/SelectQuery/properties/limit")

### limit Type

`integer`

### limit Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint64`

## prefixes

The prefixes used in the query.

`prefixes`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-prefixes.md "undefined#/query/definitions/SelectQuery/properties/prefixes")

### prefixes Type

unknown\[]

## select

The items to select. Note: the number of items to select cannot exceed the maximum query variable count defined in the store limitations.

`select`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-select.md "undefined#/query/definitions/SelectQuery/properties/select")

### select Type

unknown\[]

## where

The WHERE clause. If `None`, there is no WHERE clause, i.e. all triples are returned without filtering.

`where`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-where.md "undefined#/query/definitions/SelectQuery/properties/where")

### where Type

unknown\[]
