# Untitled undefined type in cw-storage Schema

```txt
undefined#/instantiate/definitions/PaginationConfig/properties/default_page_size
```

The default number of elements in a page.

Shall be less or equal than `max_page_size`. Default to '10' if not set.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                         |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [cw-storage.json\*](schema/cw-storage.json "open original schema") |

## default\_page\_size Type

`integer`

## default\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
