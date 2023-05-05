# ExecuteMsg Schema

```txt
undefined#/execute
```

Execute messages

| Abstract               | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :--------------------- | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Cannot be instantiated | Yes        | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## execute Type

merged type ([ExecuteMsg](okp4-cognitarium-executemsg.md))

one (and only one) of

*   [InsertData](okp4-cognitarium-executemsg-oneof-insertdata.md "check type definition")

*   [DeleteData](okp4-cognitarium-executemsg-oneof-deletedata.md "check type definition")

# ExecuteMsg Definitions

## Definitions group Binary

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/Binary"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group DataFormat

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/DataFormat"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group IRI

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/IRI"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group Literal

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/Literal"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group Node

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/Node"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group Prefix

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/Prefix"}
```

| Property                | Type     | Required | Nullable       | Defined by                                                                                                                                              |
| :---------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [namespace](#namespace) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-prefix-properties-namespace.md "undefined#/execute/definitions/Prefix/properties/namespace") |
| [prefix](#prefix)       | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-prefix-properties-prefix.md "undefined#/execute/definitions/Prefix/properties/prefix")       |

### namespace

The namespace associated with the prefix.

`namespace`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-prefix-properties-namespace.md "undefined#/execute/definitions/Prefix/properties/namespace")

#### namespace Type

`string`

### prefix

The prefix.

`prefix`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-prefix-properties-prefix.md "undefined#/execute/definitions/Prefix/properties/prefix")

#### prefix Type

`string`

## Definitions group SimpleWhereCondition

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/SimpleWhereCondition"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group TriplePattern

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/TriplePattern"}
```

| Property                | Type   | Required | Nullable       | Defined by                                                                                                                                                            |
| :---------------------- | :----- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [object](#object)       | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-triplepattern-properties-object.md "undefined#/execute/definitions/TriplePattern/properties/object")       |
| [predicate](#predicate) | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-triplepattern-properties-predicate.md "undefined#/execute/definitions/TriplePattern/properties/predicate") |
| [subject](#subject)     | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-triplepattern-properties-subject.md "undefined#/execute/definitions/TriplePattern/properties/subject")     |

### object

The object of the triple pattern.

`object`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-executemsg-definitions-triplepattern-properties-object.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-triplepattern-properties-object.md "undefined#/execute/definitions/TriplePattern/properties/object")

#### object Type

merged type ([Details](okp4-cognitarium-executemsg-definitions-triplepattern-properties-object.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-executemsg-definitions-triplepattern-properties-object-allof-0.md "check type definition")

### predicate

The predicate of the triple pattern.

`predicate`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-executemsg-definitions-triplepattern-properties-predicate.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-triplepattern-properties-predicate.md "undefined#/execute/definitions/TriplePattern/properties/predicate")

#### predicate Type

merged type ([Details](okp4-cognitarium-executemsg-definitions-triplepattern-properties-predicate.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-executemsg-definitions-triplepattern-properties-predicate-allof-0.md "check type definition")

### subject

The subject of the triple pattern.

`subject`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-executemsg-definitions-triplepattern-properties-subject.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-triplepattern-properties-subject.md "undefined#/execute/definitions/TriplePattern/properties/subject")

#### subject Type

merged type ([Details](okp4-cognitarium-executemsg-definitions-triplepattern-properties-subject.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-executemsg-definitions-triplepattern-properties-subject-allof-0.md "check type definition")

## Definitions group VarOrNode

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/VarOrNode"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group VarOrNodeOrLiteral

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/VarOrNodeOrLiteral"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group WhereCondition

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/WhereCondition"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
