# UnpinObject Schema

```txt
undefined#/execute/oneOf/3
```

UnpinObject unpins the object in the bucket for the considered sender. If the object is not pinned for the sender, this is a no-op. The object can be removed from the storage if it is not pinned anymore.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 3 Type

`object` ([UnpinObject](okp4-objectarium-executemsg-oneof-unpinobject.md))

# 3 Properties

| Property                       | Type     | Required | Nullable       | Defined by                                                                                                                                        |
| :----------------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| [unpin\_object](#unpin_object) | `object` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-executemsg-oneof-unpinobject-properties-unpin_object.md "undefined#/execute/oneOf/3/properties/unpin_object") |

## unpin\_object



`unpin_object`

*   is required

*   Type: `object` ([Details](okp4-objectarium-executemsg-oneof-unpinobject-properties-unpin_object.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-executemsg-oneof-unpinobject-properties-unpin_object.md "undefined#/execute/oneOf/3/properties/unpin_object")

### unpin\_object Type

`object` ([Details](okp4-objectarium-executemsg-oneof-unpinobject-properties-unpin_object.md))
