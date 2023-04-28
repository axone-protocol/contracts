# VarOrNodeOrLiteral Schema

```txt
undefined#/query/definitions/VarOrNodeOrLiteral
```

Represents either a variable, a node or a literal.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## VarOrNodeOrLiteral Type

merged type ([VarOrNodeOrLiteral](okp4-cognitarium-querymsg-definitions-varornodeorliteral.md))

one (and only one) of

*   [Variable](okp4-cognitarium-querymsg-definitions-varornodeorliteral-oneof-variable.md "check type definition")

*   [Node](okp4-cognitarium-querymsg-definitions-varornodeorliteral-oneof-node.md "check type definition")

*   [Literal](okp4-cognitarium-querymsg-definitions-varornodeorliteral-oneof-literal.md "check type definition")
