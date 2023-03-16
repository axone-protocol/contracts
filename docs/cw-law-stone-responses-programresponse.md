# ProgramResponse Schema

```txt
undefined#/responses/program
```

ProgramResponse carry elements to locate the program in a `cw-storage` contract.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                             |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [cw-law-stone.json\*](schema/cw-law-stone.json "open original schema") |

## program Type

`object` ([ProgramResponse](cw-law-stone-responses-programresponse.md))

# program Properties

| Property                             | Type     | Required | Nullable       | Defined by                                                                                                                                     |
| :----------------------------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| [object\_id](#object_id)             | `string` | Required | cannot be null | [cw-law-stone](cw-law-stone-responses-programresponse-properties-object_id.md "undefined#/responses/program/properties/object_id")             |
| [storage\_address](#storage_address) | `string` | Required | cannot be null | [cw-law-stone](cw-law-stone-responses-programresponse-properties-storage_address.md "undefined#/responses/program/properties/storage_address") |

## object\_id

The program object id in the `cw-storage` contract.

`object_id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [cw-law-stone](cw-law-stone-responses-programresponse-properties-object_id.md "undefined#/responses/program/properties/object_id")

### object\_id Type

`string`

## storage\_address

The `cw-storage` contract address on which the law program is stored.

`storage_address`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [cw-law-stone](cw-law-stone-responses-programresponse-properties-storage_address.md "undefined#/responses/program/properties/storage_address")

### storage\_address Type

`string`
