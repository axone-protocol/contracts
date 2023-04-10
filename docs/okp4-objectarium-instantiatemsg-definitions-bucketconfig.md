# Untitled object in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketConfig
```

BucketConfig is the type of the configuration of a bucket.

The configuration is set at the instantiation of the bucket, and is immutable and cannot be changed. The configuration is optional and if not set, the default configuration is used.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## BucketConfig Type

`object` ([Details](okp4-objectarium-instantiatemsg-definitions-bucketconfig.md))

# BucketConfig Properties

| Property                           | Type   | Required | Nullable       | Defined by                                                                                                                                                                            |
| :--------------------------------- | :----- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [hash\_algorithm](#hash_algorithm) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm.md "undefined#/instantiate/definitions/BucketConfig/properties/hash_algorithm") |

## hash\_algorithm

The algorithm used to hash the content of the objects to generate the id of the objects. The algorithm is optional and if not set, the default algorithm is used.

The default algorithm is Sha256 .

`hash_algorithm`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm.md "undefined#/instantiate/definitions/BucketConfig/properties/hash_algorithm")

### hash\_algorithm Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm-anyof-1.md "check type definition")
