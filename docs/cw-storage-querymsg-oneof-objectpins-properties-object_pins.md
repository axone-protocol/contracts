# Untitled object in cw-storage Schema

```txt
undefined#/query/oneOf/4/properties/object_pins
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [cw-storage.json\*](schema/cw-storage.json "open original schema") |

## object\_pins Type

`object` ([Details](cw-storage-querymsg-oneof-objectpins-properties-object_pins.md))

# object\_pins Properties

| Property        | Type     | Required | Nullable       | Defined by                                                                                                                                                       |
| :-------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [after](#after) | `string` | Optional | can be null    | [cw-storage](cw-storage-querymsg-oneof-objectpins-properties-object_pins-properties-after.md "undefined#/query/oneOf/4/properties/object_pins/properties/after") |
| [first](#first) | Merged   | Optional | cannot be null | [cw-storage](cw-storage-querymsg-oneof-objectpins-properties-object_pins-properties-first.md "undefined#/query/oneOf/4/properties/object_pins/properties/first") |
| [id](#id)       | `string` | Required | cannot be null | [cw-storage](cw-storage-querymsg-oneof-objectpins-properties-object_pins-properties-id.md "undefined#/query/oneOf/4/properties/object_pins/properties/id")       |

## after

The point in the sequence to start returning pins.

`after`

*   is optional

*   Type: `string`

*   can be null

*   defined in: [cw-storage](cw-storage-querymsg-oneof-objectpins-properties-object_pins-properties-after.md "undefined#/query/oneOf/4/properties/object_pins/properties/after")

### after Type

`string`

## first

The number of pins to return.

`first`

*   is optional

*   Type: merged type ([Details](cw-storage-querymsg-oneof-objectpins-properties-object_pins-properties-first.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-querymsg-oneof-objectpins-properties-object_pins-properties-first.md "undefined#/query/oneOf/4/properties/object_pins/properties/first")

### first Type

merged type ([Details](cw-storage-querymsg-oneof-objectpins-properties-object_pins-properties-first.md))

any of

*   [Untitled undefined type in cw-storage](cw-storage-querymsg-oneof-objectpins-properties-object_pins-properties-first-anyof-0.md "check type definition")

*   [Untitled null in cw-storage](cw-storage-querymsg-oneof-objectpins-properties-object_pins-properties-first-anyof-1.md "check type definition")

## id

The id of the object to get the pins for.

`id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [cw-storage](cw-storage-querymsg-oneof-objectpins-properties-object_pins-properties-id.md "undefined#/query/oneOf/4/properties/object_pins/properties/id")

### id Type

`string`
