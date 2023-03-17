# InstantiateMsg Schema

```txt
undefined#/instantiate
```

Instantiate message

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                             |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [cw-law-stone.json\*](schema/cw-law-stone.json "open original schema") |

## instantiate Type

`object` ([InstantiateMsg](cw-law-stone-instantiatemsg.md))

# instantiate Properties

| Property                             | Type     | Required | Nullable       | Defined by                                                                                                                    |
| :----------------------------------- | :------- | :------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------- |
| [program](#program)                  | Merged   | Required | cannot be null | [cw-law-stone](cw-law-stone-instantiatemsg-properties-program.md "undefined#/instantiate/properties/program")                 |
| [storage\_address](#storage_address) | `string` | Required | cannot be null | [cw-law-stone](cw-law-stone-instantiatemsg-properties-storage_address.md "undefined#/instantiate/properties/storage_address") |

## program

The Prolog program carrying law rules and facts.

`program`

*   is required

*   Type: merged type ([Details](cw-law-stone-instantiatemsg-properties-program.md))

*   cannot be null

*   defined in: [cw-law-stone](cw-law-stone-instantiatemsg-properties-program.md "undefined#/instantiate/properties/program")

### program Type

merged type ([Details](cw-law-stone-instantiatemsg-properties-program.md))

all of

*   [Untitled undefined type in cw-law-stone](cw-law-stone-instantiatemsg-properties-program-allof-0.md "check type definition")

## storage\_address

The `cw-storage` contract address on which to store the law program.

`storage_address`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [cw-law-stone](cw-law-stone-instantiatemsg-properties-storage_address.md "undefined#/instantiate/properties/storage_address")

### storage\_address Type

`string`

# InstantiateMsg Definitions

## Definitions group Binary

Reference this group by using

```json
{"$ref":"undefined#/instantiate/definitions/Binary"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
