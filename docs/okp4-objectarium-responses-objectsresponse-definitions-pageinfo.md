# PageInfo Schema

```txt
undefined#/responses/objects/definitions/PageInfo
```

PageInfo is the page information returned for paginated queries.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## PageInfo Type

`object` ([PageInfo](okp4-objectarium-responses-objectsresponse-definitions-pageinfo.md))

# PageInfo Properties

| Property                          | Type      | Required | Nullable       | Defined by                                                                                                                                                                                   |
| :-------------------------------- | :-------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [cursor](#cursor)                 | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-pageinfo-properties-cursor.md "undefined#/responses/objects/definitions/PageInfo/properties/cursor")               |
| [has\_next\_page](#has_next_page) | `boolean` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/objects/definitions/PageInfo/properties/has_next_page") |

## cursor

The cursor to the next page.

`cursor`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-pageinfo-properties-cursor.md "undefined#/responses/objects/definitions/PageInfo/properties/cursor")

### cursor Type

`string`

## has\_next\_page

Tells if there is a next page.

`has_next_page`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/objects/definitions/PageInfo/properties/has_next_page")

### has\_next\_page Type

`boolean`
