# DescribeQuery Schema

```txt
undefined#/query/definitions/DescribeQuery
```

Represents a DESCRIBE query over the triple store, allowing to retrieve a description of a resource as a set of triples serialized in a specific format.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## DescribeQuery Type

`object` ([DescribeQuery](okp4-cognitarium-querymsg-definitions-describequery.md))

# DescribeQuery Properties

| Property              | Type    | Required | Nullable       | Defined by                                                                                                                                                      |
| :-------------------- | :------ | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [prefixes](#prefixes) | `array` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-describequery-properties-prefixes.md "undefined#/query/definitions/DescribeQuery/properties/prefixes") |
| [resource](#resource) | Merged  | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-describequery-properties-resource.md "undefined#/query/definitions/DescribeQuery/properties/resource") |
| [where](#where)       | `array` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-describequery-properties-where.md "undefined#/query/definitions/DescribeQuery/properties/where")       |

## prefixes

The prefixes used in the query.

`prefixes`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-describequery-properties-prefixes.md "undefined#/query/definitions/DescribeQuery/properties/prefixes")

### prefixes Type

unknown\[]

## resource

The resource to describe given as a variable or a node.

`resource`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-querymsg-definitions-describequery-properties-resource.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-describequery-properties-resource.md "undefined#/query/definitions/DescribeQuery/properties/resource")

### resource Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-describequery-properties-resource.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-describequery-properties-resource-allof-0.md "check type definition")

## where

The WHERE clause. This clause is used to specify the resource identifier to describe using variable bindings.

`where`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-describequery-properties-where.md "undefined#/query/definitions/DescribeQuery/properties/where")

### where Type

unknown\[]
