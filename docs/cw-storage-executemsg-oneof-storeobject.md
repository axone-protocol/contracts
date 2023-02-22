# StoreObject Schema

```txt
undefined#/execute/oneOf/0
```

StoreObject store an object to the bucket and make the sender the owner of the object. The object is referenced by the hash of its content and this value is returned. If the object is already stored, this is a no-op. If pin is true, the object is pinned for the sender.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [cw-storage.json\*](schema/cw-storage.json "open original schema") |

## 0 Type

`object` ([StoreObject](cw-storage-executemsg-oneof-storeobject.md))

# 0 Properties

| Property                       | Type     | Required | Nullable       | Defined by                                                                                                                            |
| :----------------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| [store\_object](#store_object) | `object` | Required | cannot be null | [cw-storage](cw-storage-executemsg-oneof-storeobject-properties-store_object.md "undefined#/execute/oneOf/0/properties/store_object") |

## store\_object



`store_object`

*   is required

*   Type: `object` ([Details](cw-storage-executemsg-oneof-storeobject-properties-store_object.md))

*   cannot be null

*   defined in: [cw-storage](cw-storage-executemsg-oneof-storeobject-properties-store_object.md "undefined#/execute/oneOf/0/properties/store_object")

### store\_object Type

`object` ([Details](cw-storage-executemsg-oneof-storeobject-properties-store_object.md))
