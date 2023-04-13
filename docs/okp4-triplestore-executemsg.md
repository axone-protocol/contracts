# ExecuteMsg Schema

```txt
undefined#/execute
```

Execute messages

| Abstract               | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :--------------------- | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Cannot be instantiated | Yes        | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-triplestore.json\*](schema/okp4-triplestore.json "open original schema") |

## execute Type

merged type ([ExecuteMsg](okp4-triplestore-executemsg.md))

one (and only one) of

*   [Insert](okp4-triplestore-executemsg-oneof-insert.md "check type definition")

*   [Remove](okp4-triplestore-executemsg-oneof-remove.md "check type definition")

# ExecuteMsg Definitions

## Definitions group Binary

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/Binary"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group GraphInput

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/GraphInput"}
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

## Definitions group ObjectValue

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/ObjectValue"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group ResourceCriteria

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/ResourceCriteria"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group ResourceQuery

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/ResourceQuery"}
```

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                          |
| :-------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [criteria](#criteria) | `array`  | Required | cannot be null | [okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcequery-properties-criteria.md "undefined#/execute/definitions/ResourceQuery/properties/criteria") |
| [name](#name)         | `string` | Required | cannot be null | [okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcequery-properties-name.md "undefined#/execute/definitions/ResourceQuery/properties/name")         |

### criteria

The set of criteria a resource must meet to validate the query, it act as the logical conjunction of all the criteria.

`criteria`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcequery-properties-criteria.md "undefined#/execute/definitions/ResourceQuery/properties/criteria")

#### criteria Type

unknown\[]

### name

The query name, can be used to reference another query to allow join. Must be unique.

`name`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcequery-properties-name.md "undefined#/execute/definitions/ResourceQuery/properties/name")

#### name Type

`string`

## Definitions group ValueOrJoin\_for\_ObjectValue

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/ValueOrJoin_for_ObjectValue"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group ValueOrJoin\_for\_String

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/ValueOrJoin_for_String"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
