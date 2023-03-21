# Untitled undefined type in cw-storage Schema

```txt
undefined#/instantiate/definitions/PaginationConfig/properties/max_page_size
```

The maximum elements a page can contains.

Shall be less than `u32::MAX - 1`. Default to '30' if not set.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                         |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [cw-storage.json\*](schema/cw-storage.json "open original schema") |

## max\_page\_size Type

`integer`

## max\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
