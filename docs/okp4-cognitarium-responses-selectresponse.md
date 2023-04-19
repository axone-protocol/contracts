# SelectResponse Schema

```txt
undefined#/responses/select
```

Represents the response of a \[QueryMsg::Select] query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## select Type

`object` ([SelectResponse](okp4-cognitarium-responses-selectresponse.md))

# select Properties

| Property            | Type   | Required | Nullable       | Defined by                                                                                                                           |
| :------------------ | :----- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------- |
| [head](#head)       | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-properties-head.md "undefined#/responses/select/properties/head")       |
| [results](#results) | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-properties-results.md "undefined#/responses/select/properties/results") |

## head

The head of the response, i.e. the set of variables mentioned in the results.

`head`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-selectresponse-properties-head.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-properties-head.md "undefined#/responses/select/properties/head")

### head Type

merged type ([Details](okp4-cognitarium-responses-selectresponse-properties-head.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-selectresponse-properties-head-allof-0.md "check type definition")

## results

The results of the select query.

`results`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-selectresponse-properties-results.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-properties-results.md "undefined#/responses/select/properties/results")

### results Type

merged type ([Details](okp4-cognitarium-responses-selectresponse-properties-results.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-selectresponse-properties-results-allof-0.md "check type definition")

# SelectResponse Definitions

## Definitions group Head

Reference this group by using

```json
{"$ref":"undefined#/responses/select/definitions/Head"}
```

| Property      | Type    | Required | Nullable       | Defined by                                                                                                                                                       |
| :------------ | :------ | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [vars](#vars) | `array` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-head-properties-vars.md "undefined#/responses/select/definitions/Head/properties/vars") |

### vars

The variables selected in the query.

`vars`

*   is required

*   Type: `string[]`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-head-properties-vars.md "undefined#/responses/select/definitions/Head/properties/vars")

#### vars Type

`string[]`

## Definitions group IRI

Reference this group by using

```json
{"$ref":"undefined#/responses/select/definitions/IRI"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group Results

Reference this group by using

```json
{"$ref":"undefined#/responses/select/definitions/Results"}
```

| Property              | Type    | Required | Nullable       | Defined by                                                                                                                                                                     |
| :-------------------- | :------ | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [bindings](#bindings) | `array` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings.md "undefined#/responses/select/definitions/Results/properties/bindings") |

### bindings

The bindings of the results.

`bindings`

*   is required

*   Type: `object[]` ([Details](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings-items.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings.md "undefined#/responses/select/definitions/Results/properties/bindings")

#### bindings Type

`object[]` ([Details](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings-items.md))

## Definitions group Value

Reference this group by using

```json
{"$ref":"undefined#/responses/select/definitions/Value"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
