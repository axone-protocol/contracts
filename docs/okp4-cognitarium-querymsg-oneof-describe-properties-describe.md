# Untitled object in okp4-cognitarium Schema

```txt
undefined#/query/oneOf/2/properties/describe
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## describe Type

`object` ([Details](okp4-cognitarium-querymsg-oneof-describe-properties-describe.md))

# describe Properties

| Property          | Type   | Required | Nullable       | Defined by                                                                                                                                                             |
| :---------------- | :----- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [format](#format) | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-oneof-describe-properties-describe-properties-format.md "undefined#/query/oneOf/2/properties/describe/properties/format") |
| [query](#query)   | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-oneof-describe-properties-describe-properties-query.md "undefined#/query/oneOf/2/properties/describe/properties/query")   |

## format

The format in which the triples are serialized. If not provided, the default format is [Turtle](https://www.w3.org/TR/turtle/) format.

`format`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-querymsg-oneof-describe-properties-describe-properties-format.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-oneof-describe-properties-describe-properties-format.md "undefined#/query/oneOf/2/properties/describe/properties/format")

### format Type

merged type ([Details](okp4-cognitarium-querymsg-oneof-describe-properties-describe-properties-format.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-oneof-describe-properties-describe-properties-format-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-querymsg-oneof-describe-properties-describe-properties-format-anyof-1.md "check type definition")

## query

The query to execute.

`query`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-querymsg-oneof-describe-properties-describe-properties-query.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-oneof-describe-properties-describe-properties-query.md "undefined#/query/oneOf/2/properties/describe/properties/query")

### query Type

merged type ([Details](okp4-cognitarium-querymsg-oneof-describe-properties-describe-properties-query.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-oneof-describe-properties-describe-properties-query-allof-0.md "check type definition")
