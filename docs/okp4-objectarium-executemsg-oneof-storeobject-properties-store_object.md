# Untitled object in okp4-objectarium Schema

```txt
undefined#/execute/oneOf/0/properties/store_object
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## store\_object Type

`object` ([Details](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object.md))

# store\_object Properties

| Property                                         | Type      | Required | Nullable       | Defined by                                                                                                                                                                                                          |
| :----------------------------------------------- | :-------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [compression\_algorithm](#compression_algorithm) | Merged    | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-compression_algorithm.md "undefined#/execute/oneOf/0/properties/store_object/properties/compression_algorithm") |
| [data](#data)                                    | Merged    | Required | cannot be null | [okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-data.md "undefined#/execute/oneOf/0/properties/store_object/properties/data")                                   |
| [pin](#pin)                                      | `boolean` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-pin.md "undefined#/execute/oneOf/0/properties/store_object/properties/pin")                                     |

## compression\_algorithm

Specifies the compression algorithm to use when storing the object. If None, the first algorithm specified in the list of accepted compression algorithms of the bucket is used (see \[BucketLimits::accepted\_compression\_algorithms]).

`compression_algorithm`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-compression_algorithm.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-compression_algorithm.md "undefined#/execute/oneOf/0/properties/store_object/properties/compression_algorithm")

### compression\_algorithm Type

merged type ([Details](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-compression_algorithm.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-compression_algorithm-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-compression_algorithm-anyof-1.md "check type definition")

## data

The content of the object to store.

`data`

*   is required

*   Type: merged type ([Details](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-data.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-data.md "undefined#/execute/oneOf/0/properties/store_object/properties/data")

### data Type

merged type ([Details](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-data.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-data-allof-0.md "check type definition")

## pin

Specifies if the object should be pinned for the sender.

`pin`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-pin.md "undefined#/execute/oneOf/0/properties/store_object/properties/pin")

### pin Type

`boolean`
