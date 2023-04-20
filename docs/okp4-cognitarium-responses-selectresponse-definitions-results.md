# Results Schema

```txt
undefined#/responses/select/definitions/Results
```

Represents the results of a \[SelectResponse].

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## Results Type

`object` ([Results](okp4-cognitarium-responses-selectresponse-definitions-results.md))

# Results Properties

| Property              | Type    | Required | Nullable       | Defined by                                                                                                                                                                     |
| :-------------------- | :------ | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [bindings](#bindings) | `array` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings.md "undefined#/responses/select/definitions/Results/properties/bindings") |

## bindings

The bindings of the results.

`bindings`

*   is required

*   Type: `object[]` ([Details](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings-items.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings.md "undefined#/responses/select/definitions/Results/properties/bindings")

### bindings Type

`object[]` ([Details](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings-items.md))
