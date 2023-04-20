# InsertData Schema

```txt
undefined#/execute/oneOf/0
```

Insert the data as RDF triples in the store. For already existing triples it acts as no-op.

Only the smart contract owner (i.e. the address who instantiated it) is authorized to perform this action.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`object` ([InsertData](okp4-cognitarium-executemsg-oneof-insertdata.md))

# 0 Properties

| Property                     | Type     | Required | Nullable       | Defined by                                                                                                                                     |
| :--------------------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| [insert\_data](#insert_data) | `object` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data.md "undefined#/execute/oneOf/0/properties/insert_data") |

## insert\_data



`insert_data`

*   is required

*   Type: `object` ([Details](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data.md "undefined#/execute/oneOf/0/properties/insert_data")

### insert\_data Type

`object` ([Details](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data.md))
