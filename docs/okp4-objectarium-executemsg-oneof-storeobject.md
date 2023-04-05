# StoreObject Schema

```txt
undefined#/execute/oneOf/0
```

StoreObject store an object to the bucket and make the sender the owner of the object. The object is referenced by the hash of its content and this value is returned. If the object is already stored, an error is returned. If pin is true, the object is pinned for the sender.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

`object` ([StoreObject](okp4-objectarium-executemsg-oneof-storeobject.md))

# 0 Properties

| Property                       | Type     | Required | Nullable       | Defined by                                                                                                                                        |
| :----------------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| [store\_object](#store_object) | `object` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object.md "undefined#/execute/oneOf/0/properties/store_object") |

## store\_object



`store_object`

*   is required

*   Type: `object` ([Details](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object.md "undefined#/execute/oneOf/0/properties/store_object")

### store\_object Type

`object` ([Details](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object.md))
