# AskResponse Schema

```txt
undefined#/responses/ask
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## ask Type

`object` ([AskResponse](okp4-law-stone-responses-askresponse.md))

# ask Properties

| Property               | Type      | Required | Nullable       | Defined by                                                                                                                   |
| :--------------------- | :-------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| [answer](#answer)      | Merged    | Optional | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-properties-answer.md "undefined#/responses/ask/properties/answer")     |
| [gas\_used](#gas_used) | `integer` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-properties-gas_used.md "undefined#/responses/ask/properties/gas_used") |
| [height](#height)      | `integer` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-properties-height.md "undefined#/responses/ask/properties/height")     |

## answer



`answer`

*   is optional

*   Type: merged type ([Details](okp4-law-stone-responses-askresponse-properties-answer.md))

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-properties-answer.md "undefined#/responses/ask/properties/answer")

### answer Type

merged type ([Details](okp4-law-stone-responses-askresponse-properties-answer.md))

any of

*   [Untitled undefined type in okp4-law-stone](okp4-law-stone-responses-askresponse-properties-answer-anyof-0.md "check type definition")

*   [Untitled null in okp4-law-stone](okp4-law-stone-responses-askresponse-properties-answer-anyof-1.md "check type definition")

## gas\_used



`gas_used`

*   is required

*   Type: `integer`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-properties-gas_used.md "undefined#/responses/ask/properties/gas_used")

### gas\_used Type

`integer`

### gas\_used Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint64`

## height



`height`

*   is required

*   Type: `integer`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-properties-height.md "undefined#/responses/ask/properties/height")

### height Type

`integer`

### height Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint64`

# AskResponse Definitions

## Definitions group Answer

Reference this group by using

```json
{"$ref":"undefined#/responses/ask/definitions/Answer"}
```

| Property                | Type      | Required | Nullable       | Defined by                                                                                                                                                           |
| :---------------------- | :-------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [has\_more](#has_more)  | `boolean` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-has_more.md "undefined#/responses/ask/definitions/Answer/properties/has_more")   |
| [results](#results)     | `array`   | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-results.md "undefined#/responses/ask/definitions/Answer/properties/results")     |
| [success](#success)     | `boolean` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-success.md "undefined#/responses/ask/definitions/Answer/properties/success")     |
| [variables](#variables) | `array`   | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-variables.md "undefined#/responses/ask/definitions/Answer/properties/variables") |

### has\_more



`has_more`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-has_more.md "undefined#/responses/ask/definitions/Answer/properties/has_more")

#### has\_more Type

`boolean`

### results



`results`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-results.md "undefined#/responses/ask/definitions/Answer/properties/results")

#### results Type

unknown\[]

### success



`success`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-success.md "undefined#/responses/ask/definitions/Answer/properties/success")

#### success Type

`boolean`

### variables



`variables`

*   is required

*   Type: `string[]`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-variables.md "undefined#/responses/ask/definitions/Answer/properties/variables")

#### variables Type

`string[]`

## Definitions group Result

Reference this group by using

```json
{"$ref":"undefined#/responses/ask/definitions/Result"}
```

| Property                        | Type    | Required | Nullable       | Defined by                                                                                                                                                                   |
| :------------------------------ | :------ | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [substitutions](#substitutions) | `array` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-result-properties-substitutions.md "undefined#/responses/ask/definitions/Result/properties/substitutions") |

### substitutions



`substitutions`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-result-properties-substitutions.md "undefined#/responses/ask/definitions/Result/properties/substitutions")

#### substitutions Type

unknown\[]

## Definitions group Substitution

Reference this group by using

```json
{"$ref":"undefined#/responses/ask/definitions/Substitution"}
```

| Property              | Type          | Required | Nullable       | Defined by                                                                                                                                                                     |
| :-------------------- | :------------ | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [term](#term)         | Not specified | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-substitution-properties-term.md "undefined#/responses/ask/definitions/Substitution/properties/term")         |
| [variable](#variable) | `string`      | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-substitution-properties-variable.md "undefined#/responses/ask/definitions/Substitution/properties/variable") |

### term



`term`

*   is required

*   Type: unknown

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-substitution-properties-term.md "undefined#/responses/ask/definitions/Substitution/properties/term")

#### term Type

unknown

### variable



`variable`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-substitution-properties-variable.md "undefined#/responses/ask/definitions/Substitution/properties/variable")

#### variable Type

`string`

## Definitions group Term

Reference this group by using

```json
{"$ref":"undefined#/responses/ask/definitions/Term"}
```

| Property                | Type     | Required | Nullable       | Defined by                                                                                                                                                       |
| :---------------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [arguments](#arguments) | `array`  | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-term-properties-arguments.md "undefined#/responses/ask/definitions/Term/properties/arguments") |
| [name](#name)           | `string` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-term-properties-name.md "undefined#/responses/ask/definitions/Term/properties/name")           |

### arguments



`arguments`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-term-properties-arguments.md "undefined#/responses/ask/definitions/Term/properties/arguments")

#### arguments Type

unknown\[]

### name



`name`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-term-properties-name.md "undefined#/responses/ask/definitions/Term/properties/name")

#### name Type

`string`
