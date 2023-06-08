# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketConfig/properties/hash_algorithm
```

The algorithm used to hash the content of the objects to generate the id of the objects. The algorithm is optional and if not set, the default algorithm is used.

The default algorithm is Sha256 if not set.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## hash\_algorithm Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm-allof-0.md "check type definition")

## hash\_algorithm Default Value

The default value is:

```json
"sha256"
```
