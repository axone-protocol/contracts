# Untitled object in okp4-triplestore Schema

```txt
undefined#/query/oneOf/0/properties/resources
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-triplestore.json\*](schema/okp4-triplestore.json "open original schema") |

## resources Type

`object` ([Details](okp4-triplestore-querymsg-oneof-resources-properties-resources.md))

# resources Properties

| Property            | Type    | Required | Nullable       | Defined by                                                                                                                                                                  |
| :------------------ | :------ | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [format](#format)   | Merged  | Required | cannot be null | [okp4-triplestore](okp4-triplestore-querymsg-oneof-resources-properties-resources-properties-format.md "undefined#/query/oneOf/0/properties/resources/properties/format")   |
| [queries](#queries) | `array` | Required | cannot be null | [okp4-triplestore](okp4-triplestore-querymsg-oneof-resources-properties-resources-properties-queries.md "undefined#/query/oneOf/0/properties/resources/properties/queries") |

## format

The expected output format. Its value shape the way the response shall be interpreted.

`format`

*   is required

*   Type: merged type ([Details](okp4-triplestore-querymsg-oneof-resources-properties-resources-properties-format.md))

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-querymsg-oneof-resources-properties-resources-properties-format.md "undefined#/query/oneOf/0/properties/resources/properties/format")

### format Type

merged type ([Details](okp4-triplestore-querymsg-oneof-resources-properties-resources-properties-format.md))

all of

*   [Untitled undefined type in okp4-triplestore](okp4-triplestore-querymsg-oneof-resources-properties-resources-properties-format-allof-0.md "check type definition")

## queries

The queries act as the logical disjunction of each single query, a resource shall match at least one query.

`queries`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-querymsg-oneof-resources-properties-resources-properties-queries.md "undefined#/query/oneOf/0/properties/resources/properties/queries")

### queries Type

unknown\[]
