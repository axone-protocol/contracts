# ObjectResponse Schema

```txt
undefined#/responses/object
```

ObjectResponse is the response of the Object query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [cw-storage.json\*](schema/cw-storage.json "open original schema") |

## object Type

`object` ([ObjectResponse](cw-storage-responses-objectresponse.md))

# object Properties

| Property                 | Type      | Required | Nullable       | Defined by                                                                                                                   |
| :----------------------- | :-------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| [id](#id)                | `string`  | Required | cannot be null | [cw-storage](cw-storage-responses-objectresponse-properties-id.md "undefined#/responses/object/properties/id")               |
| [is\_pinned](#is_pinned) | `boolean` | Required | cannot be null | [cw-storage](cw-storage-responses-objectresponse-properties-is_pinned.md "undefined#/responses/object/properties/is_pinned") |
| [owner](#owner)          | `string`  | Required | cannot be null | [cw-storage](cw-storage-responses-objectresponse-properties-owner.md "undefined#/responses/object/properties/owner")         |
| [size](#size)            | Merged    | Required | cannot be null | [cw-storage](cw-storage-responses-objectresponse-properties-size.md "undefined#/responses/object/properties/size")           |

## id

The id of the object.

`id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectresponse-properties-id.md "undefined#/responses/object/properties/id")

### id Type

`string`

## is\_pinned

Tells if the object is pinned by at least one address.

`is_pinned`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectresponse-properties-is_pinned.md "undefined#/responses/object/properties/is_pinned")

### is\_pinned Type

`boolean`

## owner

The owner of the object.

`owner`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectresponse-properties-owner.md "undefined#/responses/object/properties/owner")

### owner Type

`string`

## size

The size of the object.

`size`

*   is required

*   Type: merged type ([Details](cw-storage-responses-objectresponse-properties-size.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectresponse-properties-size.md "undefined#/responses/object/properties/size")

### size Type

merged type ([Details](cw-storage-responses-objectresponse-properties-size.md))

all of

*   [Untitled undefined type in cw-storage](cw-storage-responses-objectresponse-properties-size-allof-0.md "check type definition")

# ObjectResponse Definitions

## Definitions group Uint128

Reference this group by using

```json
{"$ref":"undefined#/responses/object/definitions/Uint128"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
