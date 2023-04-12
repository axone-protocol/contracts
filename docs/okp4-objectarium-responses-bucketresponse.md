# BucketResponse Schema

```txt
undefined#/responses/bucket
```

BucketResponse is the response of the Bucket query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## bucket Type

`object` ([BucketResponse](okp4-objectarium-responses-bucketresponse.md))

# bucket Properties

| Property                  | Type     | Required | Nullable       | Defined by                                                                                                                                 |
| :------------------------ | :------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| [config](#config)         | Merged   | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-config.md "undefined#/responses/bucket/properties/config")         |
| [limits](#limits)         | Merged   | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-limits.md "undefined#/responses/bucket/properties/limits")         |
| [name](#name)             | `string` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-name.md "undefined#/responses/bucket/properties/name")             |
| [pagination](#pagination) | Merged   | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-pagination.md "undefined#/responses/bucket/properties/pagination") |

## config

The configuration of the bucket.

`config`

*   is required

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-properties-config.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-config.md "undefined#/responses/bucket/properties/config")

### config Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-properties-config.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-config-allof-0.md "check type definition")

## limits

The limits of the bucket.

`limits`

*   is required

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-properties-limits.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-limits.md "undefined#/responses/bucket/properties/limits")

### limits Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-properties-limits.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-limits-allof-0.md "check type definition")

## name

The name of the bucket.

`name`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-name.md "undefined#/responses/bucket/properties/name")

### name Type

`string`

## pagination

The configuration for paginated query.

`pagination`

*   is required

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-properties-pagination.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-pagination.md "undefined#/responses/bucket/properties/pagination")

### pagination Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-properties-pagination.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-pagination-allof-0.md "check type definition")

# BucketResponse Definitions

## Definitions group BucketConfig

Reference this group by using

```json
{"$ref":"undefined#/responses/bucket/definitions/BucketConfig"}
```

| Property                           | Type   | Required | Nullable       | Defined by                                                                                                                                                                                           |
| :--------------------------------- | :----- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [hash\_algorithm](#hash_algorithm) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md "undefined#/responses/bucket/definitions/BucketConfig/properties/hash_algorithm") |

### hash\_algorithm

The algorithm used to hash the content of the objects to generate the id of the objects. The algorithm is optional and if not set, the default algorithm is used.

The default algorithm is Sha256 .

`hash_algorithm`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md "undefined#/responses/bucket/definitions/BucketConfig/properties/hash_algorithm")

#### hash\_algorithm Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm-anyof-1.md "check type definition")

## Definitions group BucketLimits

Reference this group by using

```json
{"$ref":"undefined#/responses/bucket/definitions/BucketLimits"}
```

| Property                              | Type   | Required | Nullable       | Defined by                                                                                                                                                                                             |
| :------------------------------------ | :----- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [max\_object\_pins](#max_object_pins) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_pins") |
| [max\_object\_size](#max_object_size) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_size") |
| [max\_objects](#max_objects)          | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_objects")         |
| [max\_total\_size](#max_total_size)   | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_total_size")   |

### max\_object\_pins

The maximum number of pins in the bucket for an object.

`max_object_pins`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_pins")

#### max\_object\_pins Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins-anyof-1.md "check type definition")

### max\_object\_size

The maximum size of the objects in the bucket.

`max_object_size`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_size")

#### max\_object\_size Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size-anyof-1.md "check type definition")

### max\_objects

The maximum number of objects in the bucket.

`max_objects`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_objects")

#### max\_objects Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects-anyof-1.md "check type definition")

### max\_total\_size

The maximum total size of the objects in the bucket.

`max_total_size`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_total_size")

#### max\_total\_size Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size-anyof-1.md "check type definition")

## Definitions group HashAlgorithm

Reference this group by using

```json
{"$ref":"undefined#/responses/bucket/definitions/HashAlgorithm"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group PaginationConfig

Reference this group by using

```json
{"$ref":"undefined#/responses/bucket/definitions/PaginationConfig"}
```

| Property                                  | Type      | Required | Nullable    | Defined by                                                                                                                                                                                                         |
| :---------------------------------------- | :-------- | :------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [default\_page\_size](#default_page_size) | `integer` | Optional | can be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-paginationconfig-properties-default_page_size.md "undefined#/responses/bucket/definitions/PaginationConfig/properties/default_page_size") |
| [max\_page\_size](#max_page_size)         | `integer` | Optional | can be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-paginationconfig-properties-max_page_size.md "undefined#/responses/bucket/definitions/PaginationConfig/properties/max_page_size")         |

### default\_page\_size

The default number of elements in a page.

Shall be less or equal than `max_page_size`. Default to '10' if not set.

`default_page_size`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-paginationconfig-properties-default_page_size.md "undefined#/responses/bucket/definitions/PaginationConfig/properties/default_page_size")

#### default\_page\_size Type

`integer`

#### default\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

### max\_page\_size

The maximum elements a page can contains.

Shall be less than `u32::MAX - 1`. Default to '30' if not set.

`max_page_size`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-paginationconfig-properties-max_page_size.md "undefined#/responses/bucket/definitions/PaginationConfig/properties/max_page_size")

#### max\_page\_size Type

`integer`

#### max\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

## Definitions group Uint128

Reference this group by using

```json
{"$ref":"undefined#/responses/bucket/definitions/Uint128"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
