# QueryMsg Schema

```txt
undefined#/query
```

Query messages

| Abstract               | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :--------------------- | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Cannot be instantiated | Yes        | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-triplestore.json\*](schema/okp4-triplestore.json "open original schema") |

## query Type

merged type ([QueryMsg](okp4-triplestore-querymsg.md))

one (and only one) of

*   [Resources](okp4-triplestore-querymsg-oneof-resources.md "check type definition")

# QueryMsg Definitions

## Definitions group Literal

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/Literal"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group ObjectValue

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/ObjectValue"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group ResourceCriteria

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/ResourceCriteria"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group ResourceQuery

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/ResourceQuery"}
```

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                      |
| :-------------------- | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [criteria](#criteria) | `array`  | Required | cannot be null | [okp4-triplestore](okp4-triplestore-querymsg-definitions-resourcequery-properties-criteria.md "undefined#/query/definitions/ResourceQuery/properties/criteria") |
| [name](#name)         | `string` | Required | cannot be null | [okp4-triplestore](okp4-triplestore-querymsg-definitions-resourcequery-properties-name.md "undefined#/query/definitions/ResourceQuery/properties/name")         |

### criteria

The set of criteria a resource must meet to validate the query, it act as the logical conjunction of all the criteria.

`criteria`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-querymsg-definitions-resourcequery-properties-criteria.md "undefined#/query/definitions/ResourceQuery/properties/criteria")

#### criteria Type

unknown\[]

### name

The query name, can be used to reference another query to allow join. Must be unique.

`name`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-querymsg-definitions-resourcequery-properties-name.md "undefined#/query/definitions/ResourceQuery/properties/name")

#### name Type

`string`

## Definitions group ResourcesOutputFormat

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/ResourcesOutputFormat"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group ValueOrJoin\_for\_ObjectValue

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/ValueOrJoin_for_ObjectValue"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group ValueOrJoin\_for\_String

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/ValueOrJoin_for_String"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
