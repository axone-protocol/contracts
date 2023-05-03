# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketLimits/properties/accepted_compression_algorithms
```

The acceptable compression algorithms for the objects in the bucket. If this parameter is not set (none or empty array), then all compression algorithms are accepted. If this parameter is set, then only the compression algorithms in the array are accepted.

When an object is stored in the bucket without a specified compression algorithm, the first algorithm in the array is used. Therefore, the order of the algorithms in the array is significant. Typically, the most efficient compression algorithm, such as the NoCompression algorithm, should be placed first in the array.

Any attempt to store an object using a different compression algorithm than the ones specified here will fail.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## accepted\_compression\_algorithms Type

unknown\[]
