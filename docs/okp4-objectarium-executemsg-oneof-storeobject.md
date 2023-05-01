# StoreObject Schema

```txt
undefined#/execute/oneOf/0
```

StoreObject store an object to the bucket and make the sender the owner of the object. The object is referenced by the hash of its content and this value is returned. If the object is already stored, an error is returned.

The "pin" parameter specifies if the object should be pinned for the sender. In such case, the object cannot be removed (forget) from the storage.

The "compression\_algorithm" parameter specifies the algorithm for compressing the object before storing it in the storage, which is optional. If no algorithm is specified, the algorithm used is the first algorithm of the bucket configuration limits. Note that the chosen algorithm can save storage space, but it will increase CPU usage. Depending on the chosen compression algorithm and the achieved compression ratio, the gas cost of the operation will vary, either increasing or decreasing.

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
