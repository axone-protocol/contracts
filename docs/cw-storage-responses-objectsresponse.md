# ObjectsResponse Schema

```txt
undefined#/responses/objects
```

ObjectsResponse is the response of the Objects query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [cw-storage.json\*](schema/cw-storage.json "open original schema") |

## objects Type

`object` ([ObjectsResponse](cw-storage-responses-objectsresponse.md))

# objects Properties

| Property                 | Type    | Required | Nullable       | Defined by                                                                                                                     |
| :----------------------- | :------ | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| [data](#data)            | `array` | Required | cannot be null | [cw-storage](cw-storage-responses-objectsresponse-properties-data.md "undefined#/responses/objects/properties/data")           |
| [page\_info](#page_info) | Merged  | Required | cannot be null | [cw-storage](cw-storage-responses-objectsresponse-properties-page_info.md "undefined#/responses/objects/properties/page_info") |

## data

The list of objects in the bucket.

`data`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectsresponse-properties-data.md "undefined#/responses/objects/properties/data")

### data Type

unknown\[]

## page\_info

The page information.

`page_info`

*   is required

*   Type: merged type ([Details](cw-storage-responses-objectsresponse-properties-page_info.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectsresponse-properties-page_info.md "undefined#/responses/objects/properties/page_info")

### page\_info Type

merged type ([Details](cw-storage-responses-objectsresponse-properties-page_info.md))

all of

*   [Untitled undefined type in cw-storage](cw-storage-responses-objectsresponse-properties-page_info-allof-0.md "check type definition")

# ObjectsResponse Definitions

## Definitions group ObjectResponse

Reference this group by using

```json
{"$ref":"undefined#/responses/objects/definitions/ObjectResponse"}
```

| Property                 | Type      | Required | Nullable       | Defined by                                                                                                                                                                           |
| :----------------------- | :-------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [id](#id)                | `string`  | Required | cannot be null | [cw-storage](cw-storage-responses-objectsresponse-definitions-objectresponse-properties-id.md "undefined#/responses/objects/definitions/ObjectResponse/properties/id")               |
| [is\_pinned](#is_pinned) | `boolean` | Required | cannot be null | [cw-storage](cw-storage-responses-objectsresponse-definitions-objectresponse-properties-is_pinned.md "undefined#/responses/objects/definitions/ObjectResponse/properties/is_pinned") |
| [owner](#owner)          | `string`  | Required | cannot be null | [cw-storage](cw-storage-responses-objectsresponse-definitions-objectresponse-properties-owner.md "undefined#/responses/objects/definitions/ObjectResponse/properties/owner")         |
| [size](#size)            | Merged    | Required | cannot be null | [cw-storage](cw-storage-responses-objectsresponse-definitions-objectresponse-properties-size.md "undefined#/responses/objects/definitions/ObjectResponse/properties/size")           |

### id

The id of the object.

`id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectsresponse-definitions-objectresponse-properties-id.md "undefined#/responses/objects/definitions/ObjectResponse/properties/id")

#### id Type

`string`

### is\_pinned

Tells if the object is pinned by at least one address.

`is_pinned`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectsresponse-definitions-objectresponse-properties-is_pinned.md "undefined#/responses/objects/definitions/ObjectResponse/properties/is_pinned")

#### is\_pinned Type

`boolean`

### owner

The owner of the object.

`owner`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectsresponse-definitions-objectresponse-properties-owner.md "undefined#/responses/objects/definitions/ObjectResponse/properties/owner")

#### owner Type

`string`

### size

The size of the object.

`size`

*   is required

*   Type: merged type ([Details](cw-storage-responses-objectsresponse-definitions-objectresponse-properties-size.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectsresponse-definitions-objectresponse-properties-size.md "undefined#/responses/objects/definitions/ObjectResponse/properties/size")

#### size Type

merged type ([Details](cw-storage-responses-objectsresponse-definitions-objectresponse-properties-size.md))

all of

*   [Untitled undefined type in cw-storage](cw-storage-responses-objectsresponse-definitions-objectresponse-properties-size-allof-0.md "check type definition")

## Definitions group PageInfo

Reference this group by using

```json
{"$ref":"undefined#/responses/objects/definitions/PageInfo"}
```

| Property                                  | Type      | Required | Nullable       | Defined by                                                                                                                                                                               |
| :---------------------------------------- | :-------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [end\_cursor](#end_cursor)                | `string`  | Required | cannot be null | [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-end_cursor.md "undefined#/responses/objects/definitions/PageInfo/properties/end_cursor")               |
| [has\_next\_page](#has_next_page)         | `boolean` | Required | cannot be null | [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/objects/definitions/PageInfo/properties/has_next_page")         |
| [has\_previous\_page](#has_previous_page) | `boolean` | Required | cannot be null | [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-has_previous_page.md "undefined#/responses/objects/definitions/PageInfo/properties/has_previous_page") |
| [start\_cursor](#start_cursor)            | `string`  | Required | cannot be null | [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-start_cursor.md "undefined#/responses/objects/definitions/PageInfo/properties/start_cursor")           |

### end\_cursor

The cursor to the previous page.

`end_cursor`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-end_cursor.md "undefined#/responses/objects/definitions/PageInfo/properties/end_cursor")

#### end\_cursor Type

`string`

### has\_next\_page

Tells if there is a next page.

`has_next_page`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/objects/definitions/PageInfo/properties/has_next_page")

#### has\_next\_page Type

`boolean`

### has\_previous\_page

Tells if there is a previous page.

`has_previous_page`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-has_previous_page.md "undefined#/responses/objects/definitions/PageInfo/properties/has_previous_page")

#### has\_previous\_page Type

`boolean`

### start\_cursor

The cursor to the next page.

`start_cursor`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-start_cursor.md "undefined#/responses/objects/definitions/PageInfo/properties/start_cursor")

#### start\_cursor Type

`string`

## Definitions group Uint128

Reference this group by using

```json
{"$ref":"undefined#/responses/objects/definitions/Uint128"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
