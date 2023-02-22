# PageInfo Schema

```txt
undefined#/responses/objects/definitions/PageInfo
```

PageInfo is the page information returned for paginated queries.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [cw-storage.json\*](schema/cw-storage.json "open original schema") |

## PageInfo Type

`object` ([PageInfo](cw-storage-responses-objectsresponse-definitions-pageinfo.md))

# PageInfo Properties

| Property                                  | Type      | Required | Nullable       | Defined by                                                                                                                                                                               |
| :---------------------------------------- | :-------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [end\_cursor](#end_cursor)                | `string`  | Required | cannot be null | [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-end_cursor.md "undefined#/responses/objects/definitions/PageInfo/properties/end_cursor")               |
| [has\_next\_page](#has_next_page)         | `boolean` | Required | cannot be null | [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/objects/definitions/PageInfo/properties/has_next_page")         |
| [has\_previous\_page](#has_previous_page) | `boolean` | Required | cannot be null | [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-has_previous_page.md "undefined#/responses/objects/definitions/PageInfo/properties/has_previous_page") |
| [start\_cursor](#start_cursor)            | `string`  | Required | cannot be null | [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-start_cursor.md "undefined#/responses/objects/definitions/PageInfo/properties/start_cursor")           |

## end\_cursor

The cursor to the previous page.

`end_cursor`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-end_cursor.md "undefined#/responses/objects/definitions/PageInfo/properties/end_cursor")

### end\_cursor Type

`string`

## has\_next\_page

Tells if there is a next page.

`has_next_page`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/objects/definitions/PageInfo/properties/has_next_page")

### has\_next\_page Type

`boolean`

## has\_previous\_page

Tells if there is a previous page.

`has_previous_page`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-has_previous_page.md "undefined#/responses/objects/definitions/PageInfo/properties/has_previous_page")

### has\_previous\_page Type

`boolean`

## start\_cursor

The cursor to the next page.

`start_cursor`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-objectsresponse-definitions-pageinfo-properties-start_cursor.md "undefined#/responses/objects/definitions/PageInfo/properties/start_cursor")

### start\_cursor Type

`string`
