# ObjectPinsResponse Schema

```txt
undefined#/responses/object_pins
```

ObjectPinsResponse is the response of the GetObjectPins query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [cw-storage.json\*](schema/cw-storage.json "open original schema") |

## object\_pins Type

`object` ([ObjectPinsResponse](cw-storage-responses-objectpinsresponse.md))

# object\_pins Properties

| Property                 | Type    | Required | Nullable       | Defined by                                                                                                                            |
| :----------------------- | :------ | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| [data](#data)            | `array` | Required | cannot be null | [cw-storage](cw-storage-responses-objectpinsresponse-properties-data.md "undefined#/responses/object_pins/properties/data")           |
| [page\_info](#page_info) | Merged  | Required | cannot be null | [cw-storage](cw-storage-responses-objectpinsresponse-properties-page_info.md "undefined#/responses/object_pins/properties/page_info") |

## data

The list of addresses that pinned the object.

`data`

*   is required

*   Type: `string[]`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectpinsresponse-properties-data.md "undefined#/responses/object_pins/properties/data")

### data Type

`string[]`

## page\_info

The page information.

`page_info`

*   is required

*   Type: merged type ([Details](cw-storage-responses-objectpinsresponse-properties-page_info.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectpinsresponse-properties-page_info.md "undefined#/responses/object_pins/properties/page_info")

### page\_info Type

merged type ([Details](cw-storage-responses-objectpinsresponse-properties-page_info.md))

all of

*   [Untitled undefined type in cw-storage](cw-storage-responses-objectpinsresponse-properties-page_info-allof-0.md "check type definition")

# ObjectPinsResponse Definitions

## Definitions group PageInfo

Reference this group by using

```json
{"$ref":"undefined#/responses/object_pins/definitions/PageInfo"}
```

| Property                          | Type      | Required | Nullable       | Defined by                                                                                                                                                                              |
| :-------------------------------- | :-------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [cursor](#cursor)                 | `string`  | Required | cannot be null | [cw-storage](cw-storage-responses-objectpinsresponse-definitions-pageinfo-properties-cursor.md "undefined#/responses/object_pins/definitions/PageInfo/properties/cursor")               |
| [has\_next\_page](#has_next_page) | `boolean` | Required | cannot be null | [cw-storage](cw-storage-responses-objectpinsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/object_pins/definitions/PageInfo/properties/has_next_page") |

### cursor

The cursor to the next page.

`cursor`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectpinsresponse-definitions-pageinfo-properties-cursor.md "undefined#/responses/object_pins/definitions/PageInfo/properties/cursor")

#### cursor Type

`string`

### has\_next\_page

Tells if there is a next page.

`has_next_page`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectpinsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/object_pins/definitions/PageInfo/properties/has_next_page")

#### has\_next\_page Type

`boolean`
