# Untitled object in cw-storage Schema

```txt
undefined#/instantiate/definitions/BucketLimits
```

BucketLimits is the type of the limits of a bucket.

The limits are optional and if not set, there is no limit.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [cw-storage.json\*](schema/cw-storage.json "open original schema") |

## BucketLimits Type

`object` ([Details](cw-storage-instantiatemsg-definitions-bucketlimits.md))

# BucketLimits Properties

| Property                              | Type   | Required | Nullable       | Defined by                                                                                                                                                                  |
| :------------------------------------ | :----- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [max\_object\_pins](#max_object_pins) | Merged | Optional | cannot be null | [cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_object_pins.md "undefined#/instantiate/definitions/BucketLimits/properties/max_object_pins") |
| [max\_object\_size](#max_object_size) | Merged | Optional | cannot be null | [cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_object_size.md "undefined#/instantiate/definitions/BucketLimits/properties/max_object_size") |
| [max\_objects](#max_objects)          | Merged | Optional | cannot be null | [cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_objects.md "undefined#/instantiate/definitions/BucketLimits/properties/max_objects")         |
| [max\_total\_size](#max_total_size)   | Merged | Optional | cannot be null | [cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_total_size.md "undefined#/instantiate/definitions/BucketLimits/properties/max_total_size")   |

## max\_object\_pins

The maximum number of pins in the bucket for an object.

`max_object_pins`

*   is optional

*   Type: merged type ([Details](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_object_pins.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_object_pins.md "undefined#/instantiate/definitions/BucketLimits/properties/max_object_pins")

### max\_object\_pins Type

merged type ([Details](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_object_pins.md))

any of

*   [Untitled undefined type in cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_object_pins-anyof-0.md "check type definition")

*   [Untitled null in cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_object_pins-anyof-1.md "check type definition")

## max\_object\_size

The maximum size of the objects in the bucket.

`max_object_size`

*   is optional

*   Type: merged type ([Details](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_object_size.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_object_size.md "undefined#/instantiate/definitions/BucketLimits/properties/max_object_size")

### max\_object\_size Type

merged type ([Details](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_object_size.md))

any of

*   [Untitled undefined type in cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_object_size-anyof-0.md "check type definition")

*   [Untitled null in cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_object_size-anyof-1.md "check type definition")

## max\_objects

The maximum number of objects in the bucket.

`max_objects`

*   is optional

*   Type: merged type ([Details](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_objects.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_objects.md "undefined#/instantiate/definitions/BucketLimits/properties/max_objects")

### max\_objects Type

merged type ([Details](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_objects.md))

any of

*   [Untitled undefined type in cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_objects-anyof-0.md "check type definition")

*   [Untitled null in cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_objects-anyof-1.md "check type definition")

## max\_total\_size

The maximum total size of the objects in the bucket.

`max_total_size`

*   is optional

*   Type: merged type ([Details](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_total_size.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_total_size.md "undefined#/instantiate/definitions/BucketLimits/properties/max_total_size")

### max\_total\_size Type

merged type ([Details](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_total_size.md))

any of

*   [Untitled undefined type in cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_total_size-anyof-0.md "check type definition")

*   [Untitled null in cw-storage](cw-storage-instantiatemsg-definitions-bucketlimits-properties-max_total_size-anyof-1.md "check type definition")
