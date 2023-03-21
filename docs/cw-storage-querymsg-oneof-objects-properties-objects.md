# Untitled object in cw-storage Schema

```txt
undefined#/query/oneOf/2/properties/objects
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [cw-storage.json\*](schema/cw-storage.json "open original schema") |

## objects Type

`object` ([Details](cw-storage-querymsg-oneof-objects-properties-objects.md))

# objects Properties

| Property            | Type      | Required | Nullable    | Defined by                                                                                                                                                |
| :------------------ | :-------- | :------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [address](#address) | `string`  | Optional | can be null | [cw-storage](cw-storage-querymsg-oneof-objects-properties-objects-properties-address.md "undefined#/query/oneOf/2/properties/objects/properties/address") |
| [after](#after)     | `string`  | Optional | can be null | [cw-storage](cw-storage-querymsg-oneof-objects-properties-objects-properties-after.md "undefined#/query/oneOf/2/properties/objects/properties/after")     |
| [first](#first)     | `integer` | Optional | can be null | [cw-storage](cw-storage-querymsg-oneof-objects-properties-objects-properties-first.md "undefined#/query/oneOf/2/properties/objects/properties/first")     |

## address

The owner of the objects to get.

`address`

*   is optional

*   Type: `string`

*   can be null

*   defined in: [cw-storage](cw-storage-querymsg-oneof-objects-properties-objects-properties-address.md "undefined#/query/oneOf/2/properties/objects/properties/address")

### address Type

`string`

## after

The point in the sequence to start returning objects.

`after`

*   is optional

*   Type: `string`

*   can be null

*   defined in: [cw-storage](cw-storage-querymsg-oneof-objects-properties-objects-properties-after.md "undefined#/query/oneOf/2/properties/objects/properties/after")

### after Type

`string`

## first

The number of objects to return.

`first`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [cw-storage](cw-storage-querymsg-oneof-objects-properties-objects-properties-first.md "undefined#/query/oneOf/2/properties/objects/properties/first")

### first Type

`integer`

### first Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
