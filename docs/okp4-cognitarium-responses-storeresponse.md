# StoreResponse Schema

```txt
undefined#/responses/store
```

Contains information related to triple store.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## store Type

`object` ([StoreResponse](okp4-cognitarium-responses-storeresponse.md))

# store Properties

| Property          | Type     | Required | Nullable       | Defined by                                                                                                                       |
| :---------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| [limits](#limits) | Merged   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-properties-limits.md "undefined#/responses/store/properties/limits") |
| [owner](#owner)   | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-properties-owner.md "undefined#/responses/store/properties/owner")   |
| [stat](#stat)     | Merged   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-properties-stat.md "undefined#/responses/store/properties/stat")     |

## limits

The store limits.

`limits`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-properties-limits.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-properties-limits.md "undefined#/responses/store/properties/limits")

### limits Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-properties-limits.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-properties-limits-allof-0.md "check type definition")

## owner

The store owner.

`owner`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-properties-owner.md "undefined#/responses/store/properties/owner")

### owner Type

`string`

## stat

The store current usage.

`stat`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-properties-stat.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-properties-stat.md "undefined#/responses/store/properties/stat")

### stat Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-properties-stat.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-properties-stat-allof-0.md "check type definition")

# StoreResponse Definitions

## Definitions group StoreLimits

Reference this group by using

```json
{"$ref":"undefined#/responses/store/definitions/StoreLimits"}
```

| Property                                                          | Type      | Required | Nullable       | Defined by                                                                                                                                                                                                                   |
| :---------------------------------------------------------------- | :-------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [max\_byte\_size](#max_byte_size)                                 | Merged    | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_byte_size")                               |
| [max\_insert\_data\_byte\_size](#max_insert_data_byte_size)       | Merged    | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_byte_size")       |
| [max\_insert\_data\_triple\_count](#max_insert_data_triple_count) | Merged    | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_triple_count") |
| [max\_query\_limit](#max_query_limit)                             | `integer` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit.md "undefined#/responses/store/definitions/StoreLimits/properties/max_query_limit")                           |
| [max\_query\_variable\_count](#max_query_variable_count)          | `integer` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_query_variable_count")         |
| [max\_triple\_byte\_size](#max_triple_byte_size)                  | Merged    | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_triple_byte_size")                 |
| [max\_triple\_count](#max_triple_count)                           | Merged    | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_triple_count")                         |

### max\_byte\_size

The maximum number of bytes the store can contains. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any.

`max_byte_size`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_byte_size")

#### max\_byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size-allof-0.md "check type definition")

### max\_insert\_data\_byte\_size

The maximum number of bytes an insert data query can contains.

`max_insert_data_byte_size`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_byte_size")

#### max\_insert\_data\_byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size-allof-0.md "check type definition")

### max\_insert\_data\_triple\_count

The maximum number of triples an insert data query can contains (after parsing).

`max_insert_data_triple_count`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_triple_count")

#### max\_insert\_data\_triple\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count-allof-0.md "check type definition")

### max\_query\_limit

The maximum limit of a query, i.e. the maximum number of triples returned by a select query.

`max_query_limit`

*   is required

*   Type: `integer`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit.md "undefined#/responses/store/definitions/StoreLimits/properties/max_query_limit")

#### max\_query\_limit Type

`integer`

#### max\_query\_limit Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

### max\_query\_variable\_count

The maximum number of variables a query can select.

`max_query_variable_count`

*   is required

*   Type: `integer`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_query_variable_count")

#### max\_query\_variable\_count Type

`integer`

#### max\_query\_variable\_count Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

### max\_triple\_byte\_size

The maximum number of bytes the store can contains for a single triple. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. The limit is used to prevent storing very large triples, especially literals.

`max_triple_byte_size`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_triple_byte_size")

#### max\_triple\_byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size-allof-0.md "check type definition")

### max\_triple\_count

The maximum number of triples the store can contains.

`max_triple_count`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_triple_count")

#### max\_triple\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count-allof-0.md "check type definition")

## Definitions group StoreStat

Reference this group by using

```json
{"$ref":"undefined#/responses/store/definitions/StoreStat"}
```

| Property                             | Type   | Required | Nullable       | Defined by                                                                                                                                                                                     |
| :----------------------------------- | :----- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [byte\_size](#byte_size)             | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size.md "undefined#/responses/store/definitions/StoreStat/properties/byte_size")             |
| [namespace\_count](#namespace_count) | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count.md "undefined#/responses/store/definitions/StoreStat/properties/namespace_count") |
| [triple\_count](#triple_count)       | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count.md "undefined#/responses/store/definitions/StoreStat/properties/triple_count")       |

### byte\_size

The total triple size in the store, in bytes.

`byte_size`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size.md "undefined#/responses/store/definitions/StoreStat/properties/byte_size")

#### byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size-allof-0.md "check type definition")

### namespace\_count

The total number of IRI namespace present in the store.

`namespace_count`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count.md "undefined#/responses/store/definitions/StoreStat/properties/namespace_count")

#### namespace\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count-allof-0.md "check type definition")

### triple\_count

The total number of triple present in the store.

`triple_count`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count.md "undefined#/responses/store/definitions/StoreStat/properties/triple_count")

#### triple\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count-allof-0.md "check type definition")

## Definitions group Uint128

Reference this group by using

```json
{"$ref":"undefined#/responses/store/definitions/Uint128"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
