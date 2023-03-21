# Untitled object in cw-storage Schema

```txt
undefined#/instantiate/definitions/PaginationConfig
```

PaginationConfig is the type carrying configuration for paginated queries.

The fields are optional and if not set, there is a default configuration.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [cw-storage.json\*](schema/cw-storage.json "open original schema") |

## PaginationConfig Type

`object` ([Details](cw-storage-instantiatemsg-definitions-paginationconfig.md))

# PaginationConfig Properties

| Property                                  | Type      | Required | Nullable    | Defined by                                                                                                                                                                              |
| :---------------------------------------- | :-------- | :------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [default\_page\_size](#default_page_size) | `integer` | Optional | can be null | [cw-storage](cw-storage-instantiatemsg-definitions-paginationconfig-properties-default_page_size.md "undefined#/instantiate/definitions/PaginationConfig/properties/default_page_size") |
| [max\_page\_size](#max_page_size)         | `integer` | Optional | can be null | [cw-storage](cw-storage-instantiatemsg-definitions-paginationconfig-properties-max_page_size.md "undefined#/instantiate/definitions/PaginationConfig/properties/max_page_size")         |

## default\_page\_size

The default number of elements in a page.

Shall be less or equal than `max_page_size`. Default to '10' if not set.

`default_page_size`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [cw-storage](cw-storage-instantiatemsg-definitions-paginationconfig-properties-default_page_size.md "undefined#/instantiate/definitions/PaginationConfig/properties/default_page_size")

### default\_page\_size Type

`integer`

### default\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

## max\_page\_size

The maximum elements a page can contains.

Shall be less than `u32::MAX - 1`. Default to '30' if not set.

`max_page_size`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [cw-storage](cw-storage-instantiatemsg-definitions-paginationconfig-properties-max_page_size.md "undefined#/instantiate/definitions/PaginationConfig/properties/max_page_size")

### max\_page\_size Type

`integer`

### max\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
