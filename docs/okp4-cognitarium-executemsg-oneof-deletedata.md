# DeleteData Schema

```txt
undefined#/execute/oneOf/1
```

Delete the data (RDF triples) from the store matching the patterns defined by the provided query. For non-existing triples it acts as no-op.

Example: `json { "prefixes": [ { "prefix": "foaf", "namespace": "http://xmlns.com/foaf/0.1/" } ], "delete": [ { "subject": { "variable": "s" }, "predicate": { "variable": "p" }, "object": { "variable": "o" } } ], "where": [ { "simple": { "triplePattern": { "subject": { "variable": "s" }, "predicate": { "node": { "namedNode": {"prefixed": "foaf:givenName"} } }, "object": { "literal": { "simple": "Myrddin" } } } } }, { "simple": { "triplePattern": { "subject": { "variable": "s" }, "predicate": { "variable": "p" }, "object": { "variable": "o" } } } } ] `

Only the smart contract owner (i.e. the address who instantiated it) is authorized to perform this action.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`object` ([DeleteData](okp4-cognitarium-executemsg-oneof-deletedata.md))

# 1 Properties

| Property                     | Type     | Required | Nullable       | Defined by                                                                                                                                     |
| :--------------------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| [delete\_data](#delete_data) | `object` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-deletedata-properties-delete_data.md "undefined#/execute/oneOf/1/properties/delete_data") |

## delete\_data



`delete_data`

*   is required

*   Type: `object` ([Details](okp4-cognitarium-executemsg-oneof-deletedata-properties-delete_data.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-deletedata-properties-delete_data.md "undefined#/execute/oneOf/1/properties/delete_data")

### delete\_data Type

`object` ([Details](okp4-cognitarium-executemsg-oneof-deletedata-properties-delete_data.md))
