# ExecuteMsg Schema

```txt
undefined#/execute
```

Execute messages

| Abstract               | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :--------------------- | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Cannot be instantiated | Yes        | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## execute Type

merged type ([ExecuteMsg](okp4-objectarium-executemsg.md))

one (and only one) of

*   [StoreObject](okp4-objectarium-executemsg-oneof-storeobject.md "check type definition")

*   [ForgetObject](okp4-objectarium-executemsg-oneof-forgetobject.md "check type definition")

*   [PinObject](okp4-objectarium-executemsg-oneof-pinobject.md "check type definition")

*   [UnpinObject](okp4-objectarium-executemsg-oneof-unpinobject.md "check type definition")

# ExecuteMsg Definitions

## Definitions group Binary

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/Binary"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group CompressionAlgorithm

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/CompressionAlgorithm"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
