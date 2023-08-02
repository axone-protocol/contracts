# Lzma Schema

```txt
undefined#/responses/object/definitions/CompressionAlgorithm/oneOf/2
```

Represents the LZMA algorithm. LZMA is a lossless data compression/decompression algorithm that features a high compression ratio and a variable compression-dictionary size up to 4 GB.

See [the LZMA wiki page](https://en.wikipedia.org/wiki/Lempel%E2%80%93Ziv%E2%80%93Markov_chain_algorithm) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 2 Type

`string` ([Lzma](okp4-objectarium-responses-objectresponse-definitions-compressionalgorithm-oneof-lzma.md))

## 2 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value    | Explanation |
| :------- | :---------- |
| `"lzma"` |             |
