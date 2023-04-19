# Literal Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/1
```

Represents a literal S with optional language tag L or datatype IRI D.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`object` ([Literal](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal.md))

# 1 Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                                                       |
| :-------------------- | :------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [datatype](#datatype) | Merged   | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-datatype.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/datatype") |
| [type](#type)         | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-type.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/type")         |
| [value](#value)       | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-value.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/value")       |
| [xml:lang](#xmllang)  | `string` | Optional | can be null    | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-xmllang.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/xml:lang")  |

## datatype

The datatype of the literal.

`datatype`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-datatype.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-datatype.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/datatype")

### datatype Type

merged type ([Details](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-datatype.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-datatype-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-datatype-anyof-1.md "check type definition")

## type



`type`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-type.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/type")

### type Type

`string`

### type Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value       | Explanation |
| :---------- | :---------- |
| `"literal"` |             |

## value

The value of the literal.

`value`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-value.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/value")

### value Type

`string`

## xml:lang

The language tag of the literal.

`xml:lang`

*   is optional

*   Type: `string`

*   can be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-xmllang.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/xml:lang")

### xml:lang Type

`string`
