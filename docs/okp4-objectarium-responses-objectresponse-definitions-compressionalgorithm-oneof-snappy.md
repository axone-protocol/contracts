# Snappy Schema

```txt
undefined#/responses/object/definitions/CompressionAlgorithm/oneOf/1
```

Represents the Snappy algorithm. Snappy is a compression/decompression algorithm that does not aim for maximum compression. Instead, it aims for very high speeds and reasonable compression.

See [the snappy web page](https://google.github.io/snappy/) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`string` ([Snappy](okp4-objectarium-responses-objectresponse-definitions-compressionalgorithm-oneof-snappy.md))

## 1 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value      | Explanation |
| :--------- | :---------- |
| `"snappy"` |             |
