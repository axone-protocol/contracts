# Untitled object in okp4-objectarium Schema

```txt
undefined#/query/oneOf/4/properties/object_pins
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## object\_pins Type

`object` ([Details](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins.md))

# object\_pins Properties

| Property        | Type      | Required | Nullable       | Defined by                                                                                                                                                                   |
| :-------------- | :-------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [after](#after) | `string`  | Optional | can be null    | [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins-properties-after.md "undefined#/query/oneOf/4/properties/object_pins/properties/after") |
| [first](#first) | `integer` | Optional | can be null    | [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins-properties-first.md "undefined#/query/oneOf/4/properties/object_pins/properties/first") |
| [id](#id)       | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins-properties-id.md "undefined#/query/oneOf/4/properties/object_pins/properties/id")       |

## after

The point in the sequence to start returning pins.

`after`

*   is optional

*   Type: `string`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins-properties-after.md "undefined#/query/oneOf/4/properties/object_pins/properties/after")

### after Type

`string`

## first

The number of pins to return.

`first`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins-properties-first.md "undefined#/query/oneOf/4/properties/object_pins/properties/first")

### first Type

`integer`

### first Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

## id

The id of the object to get the pins for.

`id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins-properties-id.md "undefined#/query/oneOf/4/properties/object_pins/properties/id")

### id Type

`string`
