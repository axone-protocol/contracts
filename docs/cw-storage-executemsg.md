# ExecuteMsg Schema

```txt
undefined#/execute
```

Execute messages

| Abstract               | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                         |
| :--------------------- | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------- |
| Cannot be instantiated | Yes        | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [cw-storage.json\*](schema/cw-storage.json "open original schema") |

## execute Type

merged type ([ExecuteMsg](cw-storage-executemsg.md))

one (and only one) of

*   [StoreObject](cw-storage-executemsg-oneof-storeobject.md "check type definition")

*   [ForgetObject](cw-storage-executemsg-oneof-forgetobject.md "check type definition")

*   [PinObject](cw-storage-executemsg-oneof-pinobject.md "check type definition")

*   [UnpinObject](cw-storage-executemsg-oneof-unpinobject.md "check type definition")

# ExecuteMsg Definitions

## Definitions group Binary

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/Binary"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
