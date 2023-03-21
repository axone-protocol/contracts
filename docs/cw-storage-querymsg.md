# QueryMsg Schema

```txt
undefined#/query
```

Query messages

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                         |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [cw-storage.json\*](schema/cw-storage.json "open original schema") |

## query Type

merged type ([QueryMsg](cw-storage-querymsg.md))

one (and only one) of

*   [Bucket](cw-storage-querymsg-oneof-bucket.md "check type definition")

*   [Object](cw-storage-querymsg-oneof-object.md "check type definition")

*   [Objects](cw-storage-querymsg-oneof-objects.md "check type definition")

*   [ObjectData](cw-storage-querymsg-oneof-objectdata.md "check type definition")

*   [ObjectPins](cw-storage-querymsg-oneof-objectpins.md "check type definition")
