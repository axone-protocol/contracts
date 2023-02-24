# PinObject Schema

```txt
undefined#/execute/oneOf/2
```

PinObject pins the object in the bucket for the considered sender. If the object is already pinned for the sender, this is a no-op. While an object is pinned, it cannot be removed from the storage.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [cw-storage.json\*](schema/cw-storage.json "open original schema") |

## 2 Type

`object` ([PinObject](cw-storage-executemsg-oneof-pinobject.md))

# 2 Properties

| Property                   | Type     | Required | Nullable       | Defined by                                                                                                                      |
| :------------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| [pin\_object](#pin_object) | `object` | Required | cannot be null | [cw-storage](cw-storage-executemsg-oneof-pinobject-properties-pin_object.md "undefined#/execute/oneOf/2/properties/pin_object") |

## pin\_object



`pin_object`

*   is required

*   Type: `object` ([Details](cw-storage-executemsg-oneof-pinobject-properties-pin_object.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-executemsg-oneof-pinobject-properties-pin_object.md "undefined#/execute/oneOf/2/properties/pin_object")

### pin\_object Type

`object` ([Details](cw-storage-executemsg-oneof-pinobject-properties-pin_object.md))
