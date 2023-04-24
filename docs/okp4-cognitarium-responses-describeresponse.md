# DescribeResponse Schema

```txt
undefined#/responses/describe
```

Represents the response of a \[QueryMsg::Describe] query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## describe Type

`object` ([DescribeResponse](okp4-cognitarium-responses-describeresponse.md))

# describe Properties

| Property          | Type   | Required | Nullable       | Defined by                                                                                                                             |
| :---------------- | :----- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| [data](#data)     | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-describeresponse-properties-data.md "undefined#/responses/describe/properties/data")     |
| [format](#format) | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-describeresponse-properties-format.md "undefined#/responses/describe/properties/format") |

## data

The data serialized in the specified format.

`data`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-describeresponse-properties-data.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-describeresponse-properties-data.md "undefined#/responses/describe/properties/data")

### data Type

merged type ([Details](okp4-cognitarium-responses-describeresponse-properties-data.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-describeresponse-properties-data-allof-0.md "check type definition")

## format

The format of the data.

`format`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-describeresponse-properties-format.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-describeresponse-properties-format.md "undefined#/responses/describe/properties/format")

### format Type

merged type ([Details](okp4-cognitarium-responses-describeresponse-properties-format.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-describeresponse-properties-format-allof-0.md "check type definition")

# DescribeResponse Definitions

## Definitions group Binary

Reference this group by using

```json
{"$ref":"undefined#/responses/describe/definitions/Binary"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group DataFormat

Reference this group by using

```json
{"$ref":"undefined#/responses/describe/definitions/DataFormat"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
