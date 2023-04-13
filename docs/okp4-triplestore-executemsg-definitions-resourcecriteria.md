# ResourceCriteria Schema

```txt
undefined#/execute/definitions/ResourceCriteria
```

Represents a single query criteria on a resource.

It can rely on another query referencing it by its name to express conditions on links between resources (e.g. the `subject` of a resource shall be referenced in a resource of another query). It behaves as a right join, the resources of the referenced query aren't filtered.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-triplestore.json\*](schema/okp4-triplestore.json "open original schema") |

## ResourceCriteria Type

merged type ([ResourceCriteria](okp4-triplestore-executemsg-definitions-resourcecriteria.md))

one (and only one) of

*   [Untitled object in okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-0.md "check type definition")

*   [Untitled object in okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-1.md "check type definition")

*   [Untitled object in okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-2.md "check type definition")
