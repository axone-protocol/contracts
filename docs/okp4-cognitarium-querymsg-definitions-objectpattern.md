# ObjectPattern Schema

```txt
undefined#/query/definitions/ObjectPattern
```

Represents an object pattern in a \[TriplePattern] that can be either a variable, a node or a literal.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## ObjectPattern Type

merged type ([ObjectPattern](okp4-cognitarium-querymsg-definitions-objectpattern.md))

one (and only one) of

*   [Variable](okp4-cognitarium-querymsg-definitions-objectpattern-oneof-variable.md "check type definition")

*   [Node](okp4-cognitarium-querymsg-definitions-objectpattern-oneof-node.md "check type definition")

*   [Literal](okp4-cognitarium-querymsg-definitions-objectpattern-oneof-literal.md "check type definition")
