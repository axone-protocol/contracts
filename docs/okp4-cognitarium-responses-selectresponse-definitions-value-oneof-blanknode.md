# BlankNode Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/2
```

Represents a blank node.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 2 Type

`object` ([BlankNode](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-blanknode.md))

# 2 Properties

| Property        | Type     | Required | Nullable       | Defined by                                                                                                                                                                                   |
| :-------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [type](#type)   | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-blanknode-properties-type.md "undefined#/responses/select/definitions/Value/oneOf/2/properties/type")   |
| [value](#value) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-blanknode-properties-value.md "undefined#/responses/select/definitions/Value/oneOf/2/properties/value") |

## type



`type`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-blanknode-properties-type.md "undefined#/responses/select/definitions/Value/oneOf/2/properties/type")

### type Type

`string`

### type Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value          | Explanation |
| :------------- | :---------- |
| `"blank_node"` |             |

## value

The identifier of the blank node.

`value`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-blanknode-properties-value.md "undefined#/responses/select/definitions/Value/oneOf/2/properties/value")

### value Type

`string`
