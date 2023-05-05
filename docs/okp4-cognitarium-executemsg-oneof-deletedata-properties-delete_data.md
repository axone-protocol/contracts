# Untitled object in okp4-cognitarium Schema

```txt
undefined#/execute/oneOf/1/properties/delete_data
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## delete\_data Type

`object` ([Details](okp4-cognitarium-executemsg-oneof-deletedata-properties-delete_data.md))

# delete\_data Properties

| Property              | Type    | Required | Nullable       | Defined by                                                                                                                                                                             |
| :-------------------- | :------ | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [delete](#delete)     | `array` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-deletedata-properties-delete_data-properties-delete.md "undefined#/execute/oneOf/1/properties/delete_data/properties/delete")     |
| [prefixes](#prefixes) | `array` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-deletedata-properties-delete_data-properties-prefixes.md "undefined#/execute/oneOf/1/properties/delete_data/properties/prefixes") |
| [where](#where)       | `array` | Optional | can be null    | [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-deletedata-properties-delete_data-properties-where.md "undefined#/execute/oneOf/1/properties/delete_data/properties/where")       |

## delete

The items to delete.

`delete`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-deletedata-properties-delete_data-properties-delete.md "undefined#/execute/oneOf/1/properties/delete_data/properties/delete")

### delete Type

unknown\[]

## prefixes

The prefixes used in the operation.

`prefixes`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-deletedata-properties-delete_data-properties-prefixes.md "undefined#/execute/oneOf/1/properties/delete_data/properties/prefixes")

### prefixes Type

unknown\[]

## where

The WHERE clause to apply. If not provided, all the RDF triples are considered.

`where`

*   is optional

*   Type: unknown\[]

*   can be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-deletedata-properties-delete_data-properties-where.md "undefined#/execute/oneOf/1/properties/delete_data/properties/where")

### where Type

unknown\[]
