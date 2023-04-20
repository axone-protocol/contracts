# DataInput Schema

```txt
undefined#/execute/definitions/DataInput
```

Represents the input data for the \[ExecuteMsg::InsertData] message as RDF triples in a specific format.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## DataInput Type

merged type ([DataInput](okp4-cognitarium-executemsg-definitions-datainput.md))

one (and only one) of

*   [RDF XML](okp4-cognitarium-executemsg-definitions-datainput-oneof-rdf-xml.md "check type definition")

*   [Turtle](okp4-cognitarium-executemsg-definitions-datainput-oneof-turtle.md "check type definition")

*   [N-Triples](okp4-cognitarium-executemsg-definitions-datainput-oneof-n-triples.md "check type definition")
