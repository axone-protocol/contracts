# Untitled object in okp4-triplestore Schema

```txt
undefined#/execute/definitions/ResourceCriteria/oneOf/2/properties/referenced
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-triplestore.json\*](schema/okp4-triplestore.json "open original schema") |

## referenced Type

`object` ([Details](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-2-properties-referenced.md))

# referenced Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                                                                                            |
| :-------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [property](#property) | `string` | Required | cannot be null | [okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-2-properties-referenced-properties-property.md "undefined#/execute/definitions/ResourceCriteria/oneOf/2/properties/referenced/properties/property") |
| [referer](#referer)   | Merged   | Required | cannot be null | [okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-2-properties-referenced-properties-referer.md "undefined#/execute/definitions/ResourceCriteria/oneOf/2/properties/referenced/properties/referer")   |

## property

The predicate through which the referencing resource shall express the reference.

`property`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-2-properties-referenced-properties-property.md "undefined#/execute/definitions/ResourceCriteria/oneOf/2/properties/referenced/properties/property")

### property Type

`string`

## referer

The `subject` the referencing resource shall have, which may be joined on another query.

`referer`

*   is required

*   Type: merged type ([Details](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-2-properties-referenced-properties-referer.md))

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-2-properties-referenced-properties-referer.md "undefined#/execute/definitions/ResourceCriteria/oneOf/2/properties/referenced/properties/referer")

### referer Type

merged type ([Details](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-2-properties-referenced-properties-referer.md))

all of

*   [Untitled undefined type in okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-2-properties-referenced-properties-referer-allof-0.md "check type definition")
