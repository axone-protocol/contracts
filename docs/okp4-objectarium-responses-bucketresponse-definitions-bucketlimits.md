# Untitled object in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketLimits
```

BucketLimits is the type of the limits of a bucket.

The limits are optional and if not set, there is no limit.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## BucketLimits Type

`object` ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits.md))

# BucketLimits Properties

| Property                                                              | Type    | Required | Nullable       | Defined by                                                                                                                                                                                                                             |
| :-------------------------------------------------------------------- | :------ | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [accepted\_compression\_algorithms](#accepted_compression_algorithms) | `array` | Optional | can be null    | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-accepted_compression_algorithms.md "undefined#/responses/bucket/definitions/BucketLimits/properties/accepted_compression_algorithms") |
| [max\_object\_pins](#max_object_pins)                                 | Merged  | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_pins")                                 |
| [max\_object\_size](#max_object_size)                                 | Merged  | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_size")                                 |
| [max\_objects](#max_objects)                                          | Merged  | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_objects")                                         |
| [max\_total\_size](#max_total_size)                                   | Merged  | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_total_size")                                   |

## accepted\_compression\_algorithms

The acceptable compression algorithms for the objects in the bucket. If this parameter is not set (none or empty array), then all compression algorithms are accepted. If this parameter is set, then only the compression algorithms in the array are accepted.

When an object is stored in the bucket without a specified compression algorithm, the first algorithm in the array is used. Therefore, the order of the algorithms in the array is significant. Typically, the most efficient compression algorithm, such as the NoCompression algorithm, should be placed first in the array.

Any attempt to store an object using a different compression algorithm than the ones specified here will fail.

`accepted_compression_algorithms`

*   is optional

*   Type: unknown\[]

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-accepted_compression_algorithms.md "undefined#/responses/bucket/definitions/BucketLimits/properties/accepted_compression_algorithms")

### accepted\_compression\_algorithms Type

unknown\[]

## max\_object\_pins

The maximum number of pins in the bucket for an object.

`max_object_pins`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_pins")

### max\_object\_pins Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins-anyof-1.md "check type definition")

## max\_object\_size

The maximum size of the objects in the bucket.

`max_object_size`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_size")

### max\_object\_size Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size-anyof-1.md "check type definition")

## max\_objects

The maximum number of objects in the bucket.

`max_objects`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_objects")

### max\_objects Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects-anyof-1.md "check type definition")

## max\_total\_size

The maximum total size of the objects in the bucket.

`max_total_size`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_total_size")

### max\_total\_size Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size-anyof-1.md "check type definition")
