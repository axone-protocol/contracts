# ResourceQuery Schema

```txt
undefined#/query/definitions/ResourceQuery
```

A named query targeting resources.

As the contained \[ResourceCriteria] can rely on other \[ResourceQuery] it is possible to build circular queries, which is forbidden and will result in an error.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-triplestore.json\*](schema/okp4-triplestore.json "open original schema") |

## ResourceQuery Type

`object` ([ResourceQuery](okp4-triplestore-querymsg-definitions-resourcequery.md))

# ResourceQuery Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                      |
| :-------------------- | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [criteria](#criteria) | `array`  | Required | cannot be null | [okp4-triplestore](okp4-triplestore-querymsg-definitions-resourcequery-properties-criteria.md "undefined#/query/definitions/ResourceQuery/properties/criteria") |
| [name](#name)         | `string` | Required | cannot be null | [okp4-triplestore](okp4-triplestore-querymsg-definitions-resourcequery-properties-name.md "undefined#/query/definitions/ResourceQuery/properties/name")         |

## criteria

The set of criteria a resource must meet to validate the query, it act as the logical conjunction of all the criteria.

`criteria`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-querymsg-definitions-resourcequery-properties-criteria.md "undefined#/query/definitions/ResourceQuery/properties/criteria")

### criteria Type

unknown\[]

## name

The query name, can be used to reference another query to allow join. Must be unique.

`name`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-querymsg-definitions-resourcequery-properties-name.md "undefined#/query/definitions/ResourceQuery/properties/name")

### name Type

`string`
