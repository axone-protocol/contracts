# StoreLimitsInput Schema

```txt
undefined#/instantiate/definitions/StoreLimitsInput
```

Contains requested limitations regarding store usages.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## StoreLimitsInput Type

`object` ([StoreLimitsInput](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput.md))

# StoreLimitsInput Properties

| Property                                                          | Type      | Required | Nullable       | Defined by                                                                                                                                                                                                                |
| :---------------------------------------------------------------- | :-------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [max\_byte\_size](#max_byte_size)                                 | Merged    | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_byte_size.md "undefined#/instantiate/definitions/StoreLimitsInput/properties/max_byte_size")                               |
| [max\_insert\_data\_byte\_size](#max_insert_data_byte_size)       | Merged    | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_insert_data_byte_size.md "undefined#/instantiate/definitions/StoreLimitsInput/properties/max_insert_data_byte_size")       |
| [max\_insert\_data\_triple\_count](#max_insert_data_triple_count) | Merged    | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_insert_data_triple_count.md "undefined#/instantiate/definitions/StoreLimitsInput/properties/max_insert_data_triple_count") |
| [max\_query\_limit](#max_query_limit)                             | `integer` | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_query_limit.md "undefined#/instantiate/definitions/StoreLimitsInput/properties/max_query_limit")                           |
| [max\_query\_variable\_count](#max_query_variable_count)          | `integer` | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_query_variable_count.md "undefined#/instantiate/definitions/StoreLimitsInput/properties/max_query_variable_count")         |
| [max\_triple\_byte\_size](#max_triple_byte_size)                  | Merged    | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_triple_byte_size.md "undefined#/instantiate/definitions/StoreLimitsInput/properties/max_triple_byte_size")                 |
| [max\_triple\_count](#max_triple_count)                           | Merged    | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_triple_count.md "undefined#/instantiate/definitions/StoreLimitsInput/properties/max_triple_count")                         |

## max\_byte\_size

The maximum number of bytes the store can contains. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. Default to \[Uint128::MAX] if not set, which can be considered as no limit.

`max_byte_size`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_byte_size.md "undefined#/instantiate/definitions/StoreLimitsInput/properties/max_byte_size")

### max\_byte\_size Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_byte_size.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_byte_size-allof-0.md "check type definition")

### max\_byte\_size Default Value

The default value is:

```json
"340282366920938463463374607431768211455"
```

## max\_insert\_data\_byte\_size

The maximum number of bytes an insert data query can contains. Default to \[Uint128::MAX] if not set, which can be considered as no limit.

`max_insert_data_byte_size`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_insert_data_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_insert_data_byte_size.md "undefined#/instantiate/definitions/StoreLimitsInput/properties/max_insert_data_byte_size")

### max\_insert\_data\_byte\_size Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_insert_data_byte_size.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_insert_data_byte_size-allof-0.md "check type definition")

### max\_insert\_data\_byte\_size Default Value

The default value is:

```json
"340282366920938463463374607431768211455"
```

## max\_insert\_data\_triple\_count

The maximum number of triples an insert data query can contains (after parsing). Default to \[Uint128::MAX] if not set, which can be considered as no limit.

`max_insert_data_triple_count`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_insert_data_triple_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_insert_data_triple_count.md "undefined#/instantiate/definitions/StoreLimitsInput/properties/max_insert_data_triple_count")

### max\_insert\_data\_triple\_count Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_insert_data_triple_count.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_insert_data_triple_count-allof-0.md "check type definition")

### max\_insert\_data\_triple\_count Default Value

The default value is:

```json
"340282366920938463463374607431768211455"
```

## max\_query\_limit

The maximum limit of a query, i.e. the maximum number of triples returned by a select query. Default to 30 if not set.

`max_query_limit`

*   is optional

*   Type: `integer`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_query_limit.md "undefined#/instantiate/definitions/StoreLimitsInput/properties/max_query_limit")

### max\_query\_limit Type

`integer`

### max\_query\_limit Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

### max\_query\_limit Default Value

The default value is:

```json
30
```

## max\_query\_variable\_count

The maximum number of variables a query can select. Default to 30 if not set.

`max_query_variable_count`

*   is optional

*   Type: `integer`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_query_variable_count.md "undefined#/instantiate/definitions/StoreLimitsInput/properties/max_query_variable_count")

### max\_query\_variable\_count Type

`integer`

### max\_query\_variable\_count Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

### max\_query\_variable\_count Default Value

The default value is:

```json
30
```

## max\_triple\_byte\_size

The maximum number of bytes the store can contains for a single triple. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. The limit is used to prevent storing very large triples, especially literals. Default to \[Uint128::MAX] if not set, which can be considered as no limit.

`max_triple_byte_size`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_triple_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_triple_byte_size.md "undefined#/instantiate/definitions/StoreLimitsInput/properties/max_triple_byte_size")

### max\_triple\_byte\_size Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_triple_byte_size.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_triple_byte_size-allof-0.md "check type definition")

### max\_triple\_byte\_size Default Value

The default value is:

```json
"340282366920938463463374607431768211455"
```

## max\_triple\_count

The maximum number of triples the store can contains. Default to \[Uint128::MAX] if not set, which can be considered as no limit.

`max_triple_count`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_triple_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_triple_count.md "undefined#/instantiate/definitions/StoreLimitsInput/properties/max_triple_count")

### max\_triple\_count Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_triple_count.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimitsinput-properties-max_triple_count-allof-0.md "check type definition")

### max\_triple\_count Default Value

The default value is:

```json
"340282366920938463463374607431768211455"
```
