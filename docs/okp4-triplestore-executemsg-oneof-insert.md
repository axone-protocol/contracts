# Insert Schema

```txt
undefined#/execute/oneOf/0
```

Insert the Tuples extracted from the provided RDF graph. For already existing triples it act as no-op.

Only the smart contract owner (i.e. the address who instantiated it) is authorized to perform this action.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-triplestore.json\*](schema/okp4-triplestore.json "open original schema") |

## 0 Type

`object` ([Insert](okp4-triplestore-executemsg-oneof-insert.md))

# 0 Properties

| Property          | Type     | Required | Nullable       | Defined by                                                                                                                       |
| :---------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| [insert](#insert) | `object` | Required | cannot be null | [okp4-triplestore](okp4-triplestore-executemsg-oneof-insert-properties-insert.md "undefined#/execute/oneOf/0/properties/insert") |

## insert



`insert`

*   is required

*   Type: `object` ([Details](okp4-triplestore-executemsg-oneof-insert-properties-insert.md))

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-executemsg-oneof-insert-properties-insert.md "undefined#/execute/oneOf/0/properties/insert")

### insert Type

`object` ([Details](okp4-triplestore-executemsg-oneof-insert-properties-insert.md))
