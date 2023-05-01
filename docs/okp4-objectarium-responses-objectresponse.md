# ObjectResponse Schema

```txt
undefined#/responses/object
```

ObjectResponse is the response of the Object query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## object Type

`object` ([ObjectResponse](okp4-objectarium-responses-objectresponse.md))

# object Properties

| Property                                         | Type      | Required | Nullable       | Defined by                                                                                                                                                       |
| :----------------------------------------------- | :-------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [compressed\_size](#compressed_size)             | Merged    | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-compressed_size.md "undefined#/responses/object/properties/compressed_size")             |
| [compression\_algorithm](#compression_algorithm) | Merged    | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-compression_algorithm.md "undefined#/responses/object/properties/compression_algorithm") |
| [id](#id)                                        | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-id.md "undefined#/responses/object/properties/id")                                       |
| [is\_pinned](#is_pinned)                         | `boolean` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-is_pinned.md "undefined#/responses/object/properties/is_pinned")                         |
| [owner](#owner)                                  | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-owner.md "undefined#/responses/object/properties/owner")                                 |
| [size](#size)                                    | Merged    | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-size.md "undefined#/responses/object/properties/size")                                   |

## compressed\_size

The size of the object when compressed. If the object is not compressed, the value is the same as `size`.

`compressed_size`

*   is required

*   Type: merged type ([Details](okp4-objectarium-responses-objectresponse-properties-compressed_size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-compressed_size.md "undefined#/responses/object/properties/compressed_size")

### compressed\_size Type

merged type ([Details](okp4-objectarium-responses-objectresponse-properties-compressed_size.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-compressed_size-allof-0.md "check type definition")

## compression\_algorithm

The compression algorithm used to compress the content of the object.

`compression_algorithm`

*   is required

*   Type: merged type ([Details](okp4-objectarium-responses-objectresponse-properties-compression_algorithm.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-compression_algorithm.md "undefined#/responses/object/properties/compression_algorithm")

### compression\_algorithm Type

merged type ([Details](okp4-objectarium-responses-objectresponse-properties-compression_algorithm.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-compression_algorithm-allof-0.md "check type definition")

## id

The id of the object.

`id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-id.md "undefined#/responses/object/properties/id")

### id Type

`string`

## is\_pinned

Tells if the object is pinned by at least one address.

`is_pinned`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-is_pinned.md "undefined#/responses/object/properties/is_pinned")

### is\_pinned Type

`boolean`

## owner

The owner of the object.

`owner`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-owner.md "undefined#/responses/object/properties/owner")

### owner Type

`string`

## size

The size of the object.

`size`

*   is required

*   Type: merged type ([Details](okp4-objectarium-responses-objectresponse-properties-size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-size.md "undefined#/responses/object/properties/size")

### size Type

merged type ([Details](okp4-objectarium-responses-objectresponse-properties-size.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-size-allof-0.md "check type definition")

# ObjectResponse Definitions

## Definitions group CompressionAlgorithm

Reference this group by using

```json
{"$ref":"undefined#/responses/object/definitions/CompressionAlgorithm"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group Uint128

Reference this group by using

```json
{"$ref":"undefined#/responses/object/definitions/Uint128"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
