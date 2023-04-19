# Head Schema

```txt
undefined#/responses/select/definitions/Head
```

Represents the head of a \[SelectResponse].

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## Head Type

`object` ([Head](okp4-cognitarium-responses-selectresponse-definitions-head.md))

# Head Properties

| Property      | Type    | Required | Nullable       | Defined by                                                                                                                                                       |
| :------------ | :------ | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [vars](#vars) | `array` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-head-properties-vars.md "undefined#/responses/select/definitions/Head/properties/vars") |

## vars

The variables selected in the query.

`vars`

*   is required

*   Type: `string[]`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-head-properties-vars.md "undefined#/responses/select/definitions/Head/properties/vars")

### vars Type

`string[]`
