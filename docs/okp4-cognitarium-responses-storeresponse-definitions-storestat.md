# StoreStat Schema

```txt
undefined#/responses/store/definitions/StoreStat
```

Contains usage information about the triple store.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## StoreStat Type

`object` ([StoreStat](okp4-cognitarium-responses-storeresponse-definitions-storestat.md))

# StoreStat Properties

| Property                             | Type   | Required | Nullable       | Defined by                                                                                                                                                                                     |
| :----------------------------------- | :----- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [byte\_size](#byte_size)             | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size.md "undefined#/responses/store/definitions/StoreStat/properties/byte_size")             |
| [namespace\_count](#namespace_count) | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count.md "undefined#/responses/store/definitions/StoreStat/properties/namespace_count") |
| [triple\_count](#triple_count)       | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count.md "undefined#/responses/store/definitions/StoreStat/properties/triple_count")       |

## byte\_size

The total triple size in the store, in bytes.

`byte_size`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size.md "undefined#/responses/store/definitions/StoreStat/properties/byte_size")

### byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size-allof-0.md "check type definition")

## namespace\_count

The total number of IRI namespace present in the store.

`namespace_count`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count.md "undefined#/responses/store/definitions/StoreStat/properties/namespace_count")

### namespace\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count-allof-0.md "check type definition")

## triple\_count

The total number of triple present in the store.

`triple_count`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count.md "undefined#/responses/store/definitions/StoreStat/properties/triple_count")

### triple\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count-allof-0.md "check type definition")
