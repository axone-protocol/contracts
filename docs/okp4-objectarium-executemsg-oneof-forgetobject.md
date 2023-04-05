# ForgetObject Schema

```txt
undefined#/execute/oneOf/1
```

ForgetObject first unpin the object from the bucket for the considered sender, then remove it from the storage if it is not pinned anymore. If the object is pinned for other senders, it is not removed from the storage and an error is returned. If the object is not pinned for the sender, this is a no-op.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`object` ([ForgetObject](okp4-objectarium-executemsg-oneof-forgetobject.md))

# 1 Properties

| Property                         | Type     | Required | Nullable       | Defined by                                                                                                                                           |
| :------------------------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| [forget\_object](#forget_object) | `object` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-executemsg-oneof-forgetobject-properties-forget_object.md "undefined#/execute/oneOf/1/properties/forget_object") |

## forget\_object



`forget_object`

*   is required

*   Type: `object` ([Details](okp4-objectarium-executemsg-oneof-forgetobject-properties-forget_object.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-executemsg-oneof-forgetobject-properties-forget_object.md "undefined#/execute/oneOf/1/properties/forget_object")

### forget\_object Type

`object` ([Details](okp4-objectarium-executemsg-oneof-forgetobject-properties-forget_object.md))
