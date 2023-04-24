# Untitled object in okp4-cognitarium Schema

```txt
undefined#/execute/oneOf/0/properties/insert_data
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## insert\_data Type

`object` ([Details](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data.md))

# insert\_data Properties

| Property          | Type   | Required | Nullable       | Defined by                                                                                                                                                                         |
| :---------------- | :----- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [data](#data)     | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data-properties-data.md "undefined#/execute/oneOf/0/properties/insert_data/properties/data")     |
| [format](#format) | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data-properties-format.md "undefined#/execute/oneOf/0/properties/insert_data/properties/format") |

## data

The data to insert. The data must be serialized in the format specified by the `format` field. And the data are subject to the limitations defined by the `limits` specified at contract instantiation.

`data`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data-properties-data.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data-properties-data.md "undefined#/execute/oneOf/0/properties/insert_data/properties/data")

### data Type

merged type ([Details](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data-properties-data.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data-properties-data-allof-0.md "check type definition")

## format

The data format in which the triples are serialized. If not provided, the default format is [Turtle](https://www.w3.org/TR/turtle/) format.

`format`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data-properties-format.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data-properties-format.md "undefined#/execute/oneOf/0/properties/insert_data/properties/format")

### format Type

merged type ([Details](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data-properties-format.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data-properties-format-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data-properties-format-anyof-1.md "check type definition")
