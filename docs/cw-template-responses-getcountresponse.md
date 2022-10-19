# GetCountResponse Schema

```txt
undefined#/responses/get_count
```

We define a custom struct for each query response

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                           |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [cw-template.json\*](schema/cw-template.json "open original schema") |

## get\_count Type

`object` ([GetCountResponse](cw-template-responses-getcountresponse.md))

# get\_count Properties

| Property        | Type      | Required | Nullable       | Defined by                                                                                                                  |
| :-------------- | :-------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------- |
| [count](#count) | `integer` | Required | cannot be null | [cw-template](cw-template-responses-getcountresponse-properties-count.md "undefined#/responses/get_count/properties/count") |

## count



`count`

*   is required

*   Type: `integer`

*   cannot be null

*   defined in: [cw-template](cw-template-responses-getcountresponse-properties-count.md "undefined#/responses/get_count/properties/count")

### count Type

`integer`

### count Constraints

**unknown format**: the value of this string must follow the format: `int32`
