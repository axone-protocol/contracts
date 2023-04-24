# DataFormat Schema

```txt
undefined#/execute/definitions/DataFormat
```

Represents the format in which the data are serialized, for example when returned by a query or when inserted in the store.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## DataFormat Type

merged type ([DataFormat](okp4-cognitarium-executemsg-definitions-dataformat.md))

one (and only one) of

*   [RDF XML](okp4-cognitarium-executemsg-definitions-dataformat-oneof-rdf-xml.md "check type definition")

*   [Turtle](okp4-cognitarium-executemsg-definitions-dataformat-oneof-turtle.md "check type definition")

*   [N-Triples](okp4-cognitarium-executemsg-definitions-dataformat-oneof-n-triples.md "check type definition")

*   [N-Quads](okp4-cognitarium-executemsg-definitions-dataformat-oneof-n-quads.md "check type definition")
