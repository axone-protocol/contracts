# ObjectResponse Schema

```txt
undefined#/responses/objects/definitions/ObjectResponse
```

ObjectResponse is the response of the Object query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## ObjectResponse Type

`object` ([ObjectResponse](okp4-objectarium-responses-objectsresponse-definitions-objectresponse.md))

# ObjectResponse Properties

| Property                 | Type      | Required | Nullable       | Defined by                                                                                                                                                                                       |
| :----------------------- | :-------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [id](#id)                | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-id.md "undefined#/responses/objects/definitions/ObjectResponse/properties/id")               |
| [is\_pinned](#is_pinned) | `boolean` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-is_pinned.md "undefined#/responses/objects/definitions/ObjectResponse/properties/is_pinned") |
| [owner](#owner)          | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-owner.md "undefined#/responses/objects/definitions/ObjectResponse/properties/owner")         |
| [size](#size)            | Merged    | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size.md "undefined#/responses/objects/definitions/ObjectResponse/properties/size")           |

## id

The id of the object.

`id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-id.md "undefined#/responses/objects/definitions/ObjectResponse/properties/id")

### id Type

`string`

## is\_pinned

Tells if the object is pinned by at least one address.

`is_pinned`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-is_pinned.md "undefined#/responses/objects/definitions/ObjectResponse/properties/is_pinned")

### is\_pinned Type

`boolean`

## owner

The owner of the object.

`owner`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-owner.md "undefined#/responses/objects/definitions/ObjectResponse/properties/owner")

### owner Type

`string`

## size

The size of the object.

`size`

*   is required

*   Type: merged type ([Details](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size.md "undefined#/responses/objects/definitions/ObjectResponse/properties/size")

### size Type

merged type ([Details](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size-allof-0.md "check type definition")
