# Node Schema

```txt
undefined#/execute/definitions/Node
```

Represents either an IRI (named node) or a blank node.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## Node Type

merged type ([Node](okp4-cognitarium-executemsg-definitions-node.md))

one (and only one) of

*   [NamedNode](okp4-cognitarium-executemsg-definitions-node-oneof-namednode.md "check type definition")

*   [BlankNode](okp4-cognitarium-executemsg-definitions-node-oneof-blanknode.md "check type definition")
