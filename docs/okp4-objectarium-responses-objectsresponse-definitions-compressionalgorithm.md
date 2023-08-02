# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/objects/definitions/CompressionAlgorithm
```

CompressionAlgorithm is an enumeration that defines the different compression algorithms supported for compressing the content of objects. The compression algorithm specified here are relevant algorithms for compressing data on-chain, which means that they are fast to compress and decompress, and have a low computational cost.

The order of the compression algorithms is based on their estimated computational cost (quite opinionated) during both compression and decompression, ranging from the lowest to the highest. This particular order is utilized to establish the default compression algorithm for storing an object.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## CompressionAlgorithm Type

merged type ([Details](okp4-objectarium-responses-objectsresponse-definitions-compressionalgorithm.md))

one (and only one) of

*   [Passthrough](okp4-objectarium-responses-objectsresponse-definitions-compressionalgorithm-oneof-passthrough.md "check type definition")

*   [Snappy](okp4-objectarium-responses-objectsresponse-definitions-compressionalgorithm-oneof-snappy.md "check type definition")

*   [Lzma](okp4-objectarium-responses-objectsresponse-definitions-compressionalgorithm-oneof-lzma.md "check type definition")
