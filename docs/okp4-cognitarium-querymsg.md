# QueryMsg Schema

```txt
undefined#/query
```

Query messages

| Abstract               | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :--------------------- | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Cannot be instantiated | Yes        | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## query Type

merged type ([QueryMsg](okp4-cognitarium-querymsg.md))

one (and only one) of

*   [Store](okp4-cognitarium-querymsg-oneof-store.md "check type definition")

*   [Select](okp4-cognitarium-querymsg-oneof-select.md "check type definition")

*   [Describe](okp4-cognitarium-querymsg-oneof-describe.md "check type definition")

# QueryMsg Definitions

## Definitions group DataFormat

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/DataFormat"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group DescribeQuery

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/DescribeQuery"}
```

| Property              | Type    | Required | Nullable       | Defined by                                                                                                                                                      |
| :-------------------- | :------ | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [prefixes](#prefixes) | `array` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-describequery-properties-prefixes.md "undefined#/query/definitions/DescribeQuery/properties/prefixes") |
| [resource](#resource) | Merged  | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-describequery-properties-resource.md "undefined#/query/definitions/DescribeQuery/properties/resource") |
| [where](#where)       | `array` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-describequery-properties-where.md "undefined#/query/definitions/DescribeQuery/properties/where")       |

### prefixes

The prefixes used in the query.

`prefixes`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-describequery-properties-prefixes.md "undefined#/query/definitions/DescribeQuery/properties/prefixes")

#### prefixes Type

unknown\[]

### resource

The resource to describe given as a variable or a node.

`resource`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-querymsg-definitions-describequery-properties-resource.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-describequery-properties-resource.md "undefined#/query/definitions/DescribeQuery/properties/resource")

#### resource Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-describequery-properties-resource.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-describequery-properties-resource-allof-0.md "check type definition")

### where

The WHERE clause. This clause is used to specify the resource identifier to describe using variable bindings.

`where`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-describequery-properties-where.md "undefined#/query/definitions/DescribeQuery/properties/where")

#### where Type

unknown\[]

## Definitions group IRI

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/IRI"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group Literal

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/Literal"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group Node

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/Node"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group Prefix

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/Prefix"}
```

| Property                | Type     | Required | Nullable       | Defined by                                                                                                                                          |
| :---------------------- | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| [namespace](#namespace) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-namespace.md "undefined#/query/definitions/Prefix/properties/namespace") |
| [prefix](#prefix)       | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-prefix.md "undefined#/query/definitions/Prefix/properties/prefix")       |

### namespace

The namespace associated with the prefix.

`namespace`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-namespace.md "undefined#/query/definitions/Prefix/properties/namespace")

#### namespace Type

`string`

### prefix

The prefix.

`prefix`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-prefix.md "undefined#/query/definitions/Prefix/properties/prefix")

#### prefix Type

`string`

## Definitions group SelectItem

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/SelectItem"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group SelectQuery

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/SelectQuery"}
```

| Property                | Type      | Required | Nullable       | Defined by                                                                                                                                                  |
| :---------------------- | :-------- | :------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [limit](#limit)         | `integer` | Optional | can be null    | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-limit.md "undefined#/query/definitions/SelectQuery/properties/limit")       |
| [prefixes](#prefixes-1) | `array`   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-prefixes.md "undefined#/query/definitions/SelectQuery/properties/prefixes") |
| [select](#select)       | `array`   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-select.md "undefined#/query/definitions/SelectQuery/properties/select")     |
| [where](#where-1)       | `array`   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-where.md "undefined#/query/definitions/SelectQuery/properties/where")       |

### limit

The maximum number of results to return. If `None`, there is no limit. Note: the value of the limit cannot exceed the maximum query limit defined in the store limitations.

`limit`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-limit.md "undefined#/query/definitions/SelectQuery/properties/limit")

#### limit Type

`integer`

#### limit Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint64`

### prefixes

The prefixes used in the query.

`prefixes`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-prefixes.md "undefined#/query/definitions/SelectQuery/properties/prefixes")

#### prefixes Type

unknown\[]

### select

The items to select. Note: the number of items to select cannot exceed the maximum query variable count defined in the store limitations.

`select`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-select.md "undefined#/query/definitions/SelectQuery/properties/select")

#### select Type

unknown\[]

### where

The WHERE clause. If `None`, there is no WHERE clause, i.e. all triples are returned without filtering.

`where`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-where.md "undefined#/query/definitions/SelectQuery/properties/where")

#### where Type

unknown\[]

## Definitions group SimpleWhereCondition

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/SimpleWhereCondition"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group TriplePattern

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/TriplePattern"}
```

| Property                | Type   | Required | Nullable       | Defined by                                                                                                                                                        |
| :---------------------- | :----- | :------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [object](#object)       | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object.md "undefined#/query/definitions/TriplePattern/properties/object")       |
| [predicate](#predicate) | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate.md "undefined#/query/definitions/TriplePattern/properties/predicate") |
| [subject](#subject)     | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject.md "undefined#/query/definitions/TriplePattern/properties/subject")     |

### object

The object of the triple pattern.

`object`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object.md "undefined#/query/definitions/TriplePattern/properties/object")

#### object Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object-allof-0.md "check type definition")

### predicate

The predicate of the triple pattern.

`predicate`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate.md "undefined#/query/definitions/TriplePattern/properties/predicate")

#### predicate Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate-allof-0.md "check type definition")

### subject

The subject of the triple pattern.

`subject`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject.md "undefined#/query/definitions/TriplePattern/properties/subject")

#### subject Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject-allof-0.md "check type definition")

## Definitions group VarOrNode

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/VarOrNode"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group VarOrNodeOrLiteral

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/VarOrNodeOrLiteral"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group WhereCondition

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/WhereCondition"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
