# Untitled object in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketConfig
```

BucketConfig is the type of the configuration of a bucket.

The configuration is set at the instantiation of the bucket, and is immutable and cannot be changed. The configuration is optional and if not set, the default configuration is used.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## BucketConfig Type

`object` ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig.md))

# BucketConfig Properties

| Property                                                              | Type    | Required | Nullable       | Defined by                                                                                                                                                                                                                             |
| :-------------------------------------------------------------------- | :------ | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [accepted\_compression\_algorithms](#accepted_compression_algorithms) | `array` | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-accepted_compression_algorithms.md "undefined#/responses/bucket/definitions/BucketConfig/properties/accepted_compression_algorithms") |
| [hash\_algorithm](#hash_algorithm)                                    | Merged  | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md "undefined#/responses/bucket/definitions/BucketConfig/properties/hash_algorithm")                                   |

## accepted\_compression\_algorithms

The acceptable compression algorithms for the objects in the bucket. If this parameter is not set (none or empty array), then all compression algorithms are accepted. If this parameter is set, then only the compression algorithms in the array are accepted.

When an object is stored in the bucket without a specified compression algorithm, the first algorithm in the array is used. Therefore, the order of the algorithms in the array is significant. Typically, the most efficient compression algorithm, such as the NoCompression algorithm, should be placed first in the array.

Any attempt to store an object using a different compression algorithm than the ones specified here will fail.

`accepted_compression_algorithms`

*   is optional

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-accepted_compression_algorithms.md "undefined#/responses/bucket/definitions/BucketConfig/properties/accepted_compression_algorithms")

### accepted\_compression\_algorithms Type

unknown\[]

### accepted\_compression\_algorithms Default Value

The default value is:

```json
[
  "passthrough",
  "snappy"
]
```

## hash\_algorithm

The algorithm used to hash the content of the objects to generate the id of the objects. The algorithm is optional and if not set, the default algorithm is used.

The default algorithm is Sha256 if not set.

`hash_algorithm`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md "undefined#/responses/bucket/definitions/BucketConfig/properties/hash_algorithm")

### hash\_algorithm Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm-allof-0.md "check type definition")

### hash\_algorithm Default Value

The default value is:

```json
"sha256"
```
