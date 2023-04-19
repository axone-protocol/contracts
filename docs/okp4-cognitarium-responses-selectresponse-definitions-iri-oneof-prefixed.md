# Prefixed Schema

```txt
undefined#/responses/select/definitions/IRI/oneOf/0
```

An IRI prefixed with a prefix. The prefixed IRI is expanded to a full IRI using the prefix definition specified in the query. For example, the prefixed IRI `rdf:type` is expanded to `http://www.w3.org/1999/02/22-rdf-syntax-ns#type`.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`object` ([Prefixed](okp4-cognitarium-responses-selectresponse-definitions-iri-oneof-prefixed.md))

# 0 Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                                                    |
| :-------------------- | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [prefixed](#prefixed) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-iri-oneof-prefixed-properties-prefixed.md "undefined#/responses/select/definitions/IRI/oneOf/0/properties/prefixed") |

## prefixed



`prefixed`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-iri-oneof-prefixed-properties-prefixed.md "undefined#/responses/select/definitions/IRI/oneOf/0/properties/prefixed")

### prefixed Type

`string`
