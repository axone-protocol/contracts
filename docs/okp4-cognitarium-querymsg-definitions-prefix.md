# Prefix Schema

```txt
undefined#/query/definitions/Prefix
```

Represents a prefix in a \[SelectQuery]. A prefix is a shortcut for a namespace used in the query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## Prefix Type

`object` ([Prefix](okp4-cognitarium-querymsg-definitions-prefix.md))

# Prefix Properties

| Property                | Type     | Required | Nullable       | Defined by                                                                                                                                          |
| :---------------------- | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| [namespace](#namespace) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-namespace.md "undefined#/query/definitions/Prefix/properties/namespace") |
| [prefix](#prefix)       | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-prefix.md "undefined#/query/definitions/Prefix/properties/prefix")       |

## namespace

The namespace associated with the prefix.

`namespace`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-namespace.md "undefined#/query/definitions/Prefix/properties/namespace")

### namespace Type

`string`

## prefix

The prefix.

`prefix`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-prefix.md "undefined#/query/definitions/Prefix/properties/prefix")

### prefix Type

`string`
