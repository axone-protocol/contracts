# BucketResponse Schema

```txt
undefined#/responses/bucket
```

BucketResponse is the response of the Bucket query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [cw-storage.json\*](schema/cw-storage.json "open original schema") |

## bucket Type

`object` ([BucketResponse](cw-storage-responses-bucketresponse.md))

# bucket Properties

| Property          | Type     | Required | Nullable       | Defined by                                                                                                             |
| :---------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------- |
| [limits](#limits) | Merged   | Required | cannot be null | [cw-storage](cw-storage-responses-bucketresponse-properties-limits.md "undefined#/responses/bucket/properties/limits") |
| [name](#name)     | `string` | Required | cannot be null | [cw-storage](cw-storage-responses-bucketresponse-properties-name.md "undefined#/responses/bucket/properties/name")     |

## limits

The limits of the bucket.

`limits`

*   is required

*   Type: merged type ([Details](cw-storage-responses-bucketresponse-properties-limits.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-bucketresponse-properties-limits.md "undefined#/responses/bucket/properties/limits")

### limits Type

merged type ([Details](cw-storage-responses-bucketresponse-properties-limits.md))

all of

*   [Untitled undefined type in cw-storage](cw-storage-responses-bucketresponse-properties-limits-allof-0.md "check type definition")

## name

The name of the bucket.

`name`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-bucketresponse-properties-name.md "undefined#/responses/bucket/properties/name")

### name Type

`string`

# BucketResponse Definitions

## Definitions group BucketLimits

Reference this group by using

```json
{"$ref":"undefined#/responses/bucket/definitions/BucketLimits"}
```

| Property                              | Type   | Required | Nullable       | Defined by                                                                                                                                                                                 |
| :------------------------------------ | :----- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [max\_object\_pins](#max_object_pins) | Merged | Optional | cannot be null | [cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_pins") |
| [max\_object\_size](#max_object_size) | Merged | Optional | cannot be null | [cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_size") |
| [max\_objects](#max_objects)          | Merged | Optional | cannot be null | [cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_objects")         |
| [max\_total\_size](#max_total_size)   | Merged | Optional | cannot be null | [cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_total_size")   |

### max\_object\_pins

The maximum number of pins in the bucket for an object.

`max_object_pins`

*   is optional

*   Type: merged type ([Details](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_pins")

#### max\_object\_pins Type

merged type ([Details](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md))

any of

*   [Untitled undefined type in cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins-anyof-0.md "check type definition")

*   [Untitled null in cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins-anyof-1.md "check type definition")

### max\_object\_size

The maximum size of the objects in the bucket.

`max_object_size`

*   is optional

*   Type: merged type ([Details](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_size")

#### max\_object\_size Type

merged type ([Details](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md))

any of

*   [Untitled undefined type in cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size-anyof-0.md "check type definition")

*   [Untitled null in cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size-anyof-1.md "check type definition")

### max\_objects

The maximum number of objects in the bucket.

`max_objects`

*   is optional

*   Type: merged type ([Details](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_objects")

#### max\_objects Type

merged type ([Details](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md))

any of

*   [Untitled undefined type in cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_objects-anyof-0.md "check type definition")

*   [Untitled null in cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_objects-anyof-1.md "check type definition")

### max\_total\_size

The maximum total size of the objects in the bucket.

`max_total_size`

*   is optional

*   Type: merged type ([Details](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_total_size")

#### max\_total\_size Type

merged type ([Details](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md))

any of

*   [Untitled undefined type in cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size-anyof-0.md "check type definition")

*   [Untitled null in cw-storage](cw-storage-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size-anyof-1.md "check type definition")

## Definitions group Uint128

Reference this group by using

```json
{"$ref":"undefined#/responses/bucket/definitions/Uint128"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
