# Untitled object in okp4-triplestore Schema

```txt
undefined#/execute/definitions/ResourceCriteria/oneOf/1/properties/property
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-triplestore.json\*](schema/okp4-triplestore.json "open original schema") |

## property Type

`object` ([Details](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-1-properties-property.md))

# property Properties

| Property                | Type     | Required | Nullable       | Defined by                                                                                                                                                                                                                          |
| :---------------------- | :------- | :------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [object](#object)       | Merged   | Required | cannot be null | [okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-1-properties-property-properties-object.md "undefined#/execute/definitions/ResourceCriteria/oneOf/1/properties/property/properties/object")       |
| [predicate](#predicate) | `string` | Required | cannot be null | [okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-1-properties-property-properties-predicate.md "undefined#/execute/definitions/ResourceCriteria/oneOf/1/properties/property/properties/predicate") |

## object

The object to match, which may be joined on another query.

`object`

*   is required

*   Type: merged type ([Details](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-1-properties-property-properties-object.md))

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-1-properties-property-properties-object.md "undefined#/execute/definitions/ResourceCriteria/oneOf/1/properties/property/properties/object")

### object Type

merged type ([Details](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-1-properties-property-properties-object.md))

all of

*   [Untitled undefined type in okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-1-properties-property-properties-object-allof-0.md "check type definition")

## predicate

The predicate to match.

`predicate`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-triplestore](okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-1-properties-property-properties-predicate.md "undefined#/execute/definitions/ResourceCriteria/oneOf/1/properties/property/properties/predicate")

### predicate Type

`string`
