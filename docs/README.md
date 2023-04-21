# README

## Top-level Schemas

*   [okp4-cognitarium](./okp4-cognitarium.md "TriplestoreOverviewThe okp4-cognitarium smart contract enables the storage of RDF graphs triples (i") – `-`

*   [okp4-law-stone](./okp4-law-stone.md "Law StoneOverviewThe okp4-law-stone smart contract aims to provide GaaS (i") – `-`

*   [okp4-objectarium](./okp4-objectarium.md "ObjectariumOverviewThe okp4-objectarium smart contract enables the storage of arbitrary objects in any Cosmos blockchains using the CosmWasm framework") – `-`

## Other Schemas

### Objects

*   [Ask](./okp4-law-stone-querymsg-oneof-ask.md "If not broken, ask the logic module the provided query with the law program loaded") – `undefined#/query/oneOf/0`

*   [AskResponse](./okp4-law-stone-responses-askresponse.md) – `undefined#/responses/ask`

*   [BlankNode](./okp4-cognitarium-querymsg-definitions-node-oneof-blanknode.md "An RDF blank node") – `undefined#/query/definitions/Node/oneOf/1`

*   [BlankNode](./okp4-cognitarium-responses-selectresponse-definitions-value-oneof-blanknode.md "Represents a blank node") – `undefined#/responses/select/definitions/Value/oneOf/2`

*   [Bucket](./okp4-objectarium-querymsg-oneof-bucket.md "Bucket returns the bucket information") – `undefined#/query/oneOf/0`

*   [BucketResponse](./okp4-objectarium-responses-bucketresponse.md "BucketResponse is the response of the Bucket query") – `undefined#/responses/bucket`

*   [ForgetObject](./okp4-objectarium-executemsg-oneof-forgetobject.md "ForgetObject first unpin the object from the bucket for the considered sender, then remove it from the storage if it is not pinned anymore") – `undefined#/execute/oneOf/1`

*   [Full](./okp4-cognitarium-querymsg-definitions-iri-oneof-full.md "A full IRI") – `undefined#/query/definitions/IRI/oneOf/1`

*   [Full](./okp4-cognitarium-responses-selectresponse-definitions-iri-oneof-full.md "A full IRI") – `undefined#/responses/select/definitions/IRI/oneOf/1`

*   [Head](./okp4-cognitarium-responses-selectresponse-definitions-head.md "Represents the head of a \[SelectResponse]") – `undefined#/responses/select/definitions/Head`

*   [InsertData](./okp4-cognitarium-executemsg-oneof-insertdata.md "Insert the data as RDF triples in the store") – `undefined#/execute/oneOf/0`

*   [InstantiateMsg](./okp4-cognitarium-instantiatemsg.md "Instantiate message") – `undefined#/instantiate`

*   [InstantiateMsg](./okp4-law-stone-instantiatemsg.md "Instantiate message") – `undefined#/instantiate`

*   [InstantiateMsg](./okp4-objectarium-instantiatemsg.md "Instantiate messages") – `undefined#/instantiate`

*   [LanguageTaggedString](./okp4-cognitarium-querymsg-definitions-literal-oneof-languagetaggedstring.md "A language-tagged string") – `undefined#/query/definitions/Literal/oneOf/1`

*   [Literal](./okp4-cognitarium-querymsg-definitions-objectpattern-oneof-literal.md "An RDF literal, i") – `undefined#/query/definitions/ObjectPattern/oneOf/2`

*   [Literal](./okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal.md "Represents a literal S with optional language tag L or datatype IRI D") – `undefined#/responses/select/definitions/Value/oneOf/1`

*   [N-Triples](./okp4-cognitarium-executemsg-definitions-datainput-oneof-n-triples.md "Input in N-Triples format") – `undefined#/execute/definitions/DataInput/oneOf/2`

*   [NamedNode](./okp4-cognitarium-querymsg-definitions-node-oneof-namednode.md "An RDF IRI") – `undefined#/query/definitions/Node/oneOf/0`

*   [Node](./okp4-cognitarium-querymsg-definitions-objectpattern-oneof-node.md "A node, i") – `undefined#/query/definitions/ObjectPattern/oneOf/1`

*   [Node](./okp4-cognitarium-querymsg-definitions-predicatepattern-oneof-node.md "A node, i") – `undefined#/query/definitions/PredicatePattern/oneOf/1`

*   [Node](./okp4-cognitarium-querymsg-definitions-subjectpattern-oneof-node.md "A node, i") – `undefined#/query/definitions/SubjectPattern/oneOf/1`

*   [Object](./okp4-objectarium-querymsg-oneof-object.md "Object returns the object information with the given id") – `undefined#/query/oneOf/1`

*   [ObjectData](./okp4-objectarium-querymsg-oneof-objectdata.md "ObjectData returns the content of the object with the given id") – `undefined#/query/oneOf/3`

*   [ObjectPins](./okp4-objectarium-querymsg-oneof-objectpins.md "ObjectPins returns the list of addresses that pinned the object with the given id with support for pagination") – `undefined#/query/oneOf/4`

*   [ObjectPinsResponse](./okp4-objectarium-responses-objectpinsresponse.md "ObjectPinsResponse is the response of the GetObjectPins query") – `undefined#/responses/object_pins`

*   [ObjectResponse](./okp4-objectarium-responses-objectresponse.md "ObjectResponse is the response of the Object query") – `undefined#/responses/object`

*   [ObjectResponse](./okp4-objectarium-responses-objectsresponse-definitions-objectresponse.md "ObjectResponse is the response of the Object query") – `undefined#/responses/objects/definitions/ObjectResponse`

*   [Objects](./okp4-objectarium-querymsg-oneof-objects.md "Objects returns the list of objects in the bucket with support for pagination") – `undefined#/query/oneOf/2`

*   [ObjectsResponse](./okp4-objectarium-responses-objectsresponse.md "ObjectsResponse is the response of the Objects query") – `undefined#/responses/objects`

*   [PageInfo](./okp4-objectarium-responses-objectpinsresponse-definitions-pageinfo.md "PageInfo is the page information returned for paginated queries") – `undefined#/responses/object_pins/definitions/PageInfo`

*   [PageInfo](./okp4-objectarium-responses-objectsresponse-definitions-pageinfo.md "PageInfo is the page information returned for paginated queries") – `undefined#/responses/objects/definitions/PageInfo`

*   [PinObject](./okp4-objectarium-executemsg-oneof-pinobject.md "PinObject pins the object in the bucket for the considered sender") – `undefined#/execute/oneOf/2`

*   [Prefix](./okp4-cognitarium-querymsg-definitions-prefix.md "Represents a prefix in a \[SelectQuery]") – `undefined#/query/definitions/Prefix`

*   [Prefixed](./okp4-cognitarium-querymsg-definitions-iri-oneof-prefixed.md "An IRI prefixed with a prefix") – `undefined#/query/definitions/IRI/oneOf/0`

*   [Prefixed](./okp4-cognitarium-responses-selectresponse-definitions-iri-oneof-prefixed.md "An IRI prefixed with a prefix") – `undefined#/responses/select/definitions/IRI/oneOf/0`

*   [ProgramResponse](./okp4-law-stone-responses-programresponse.md "ProgramResponse carry elements to locate the program in a okp4-objectarium contract") – `undefined#/responses/program`

*   [RDF XML](./okp4-cognitarium-executemsg-definitions-datainput-oneof-rdf-xml.md "Input in RDF/XML format") – `undefined#/execute/definitions/DataInput/oneOf/0`

*   [Results](./okp4-cognitarium-responses-selectresponse-definitions-results.md "Represents the results of a \[SelectResponse]") – `undefined#/responses/select/definitions/Results`

*   [Select](./okp4-cognitarium-querymsg-oneof-select.md "Returns the resources matching the criteria defined by the provided query") – `undefined#/query/oneOf/1`

*   [SelectQuery](./okp4-cognitarium-querymsg-definitions-selectquery.md "Represents a SELECT query over the triple store, allowing to select variables to return and to filter the results") – `undefined#/query/definitions/SelectQuery`

*   [SelectResponse](./okp4-cognitarium-responses-selectresponse.md "Represents the response of a \[QueryMsg::Select] query") – `undefined#/responses/select`

*   [Simple](./okp4-cognitarium-querymsg-definitions-literal-oneof-simple.md "A simple literal without datatype or language form") – `undefined#/query/definitions/Literal/oneOf/0`

*   [Simple](./okp4-cognitarium-querymsg-definitions-wherecondition-oneof-simple.md "Represents a simple condition") – `undefined#/query/definitions/WhereCondition/oneOf/0`

*   [StoreLimits](./okp4-cognitarium-instantiatemsg-definitions-storelimits.md "Contains limitations regarding store usages") – `undefined#/instantiate/definitions/StoreLimits`

*   [StoreLimits](./okp4-cognitarium-responses-storeresponse-definitions-storelimits.md "Contains limitations regarding store usages") – `undefined#/responses/store/definitions/StoreLimits`

*   [StoreObject](./okp4-objectarium-executemsg-oneof-storeobject.md "StoreObject store an object to the bucket and make the sender the owner of the object") – `undefined#/execute/oneOf/0`

*   [StoreResponse](./okp4-cognitarium-responses-storeresponse.md "Contains information related to triple store") – `undefined#/responses/store`

*   [StoreStat](./okp4-cognitarium-responses-storeresponse-definitions-storestat.md "Contains usage information about the triple store") – `undefined#/responses/store/definitions/StoreStat`

*   [TriplePattern](./okp4-cognitarium-querymsg-definitions-simplewherecondition-oneof-triplepattern.md "Represents a triple pattern, i") – `undefined#/query/definitions/SimpleWhereCondition/oneOf/0`

*   [TriplePattern](./okp4-cognitarium-querymsg-definitions-triplepattern.md "Represents a triple pattern in a \[SimpleWhereCondition]") – `undefined#/query/definitions/TriplePattern`

*   [Turtle](./okp4-cognitarium-executemsg-definitions-datainput-oneof-turtle.md "Input in Turtle format") – `undefined#/execute/definitions/DataInput/oneOf/1`

*   [TypedValue](./okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue.md "A value with a datatype") – `undefined#/query/definitions/Literal/oneOf/2`

*   [URI](./okp4-cognitarium-responses-selectresponse-definitions-value-oneof-uri.md "Represents an IRI") – `undefined#/responses/select/definitions/Value/oneOf/0`

*   [UnpinObject](./okp4-objectarium-executemsg-oneof-unpinobject.md "UnpinObject unpins the object in the bucket for the considered sender") – `undefined#/execute/oneOf/3`

*   [Untitled object in okp4-cognitarium](./okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data.md) – `undefined#/execute/oneOf/0/properties/insert_data`

*   [Untitled object in okp4-cognitarium](./okp4-cognitarium-querymsg-oneof-select-properties-select.md) – `undefined#/query/oneOf/1/properties/select`

*   [Untitled object in okp4-cognitarium](./okp4-cognitarium-querymsg-definitions-literal-oneof-languagetaggedstring-properties-language_tagged_string.md) – `undefined#/query/definitions/Literal/oneOf/1/properties/language_tagged_string`

*   [Untitled object in okp4-cognitarium](./okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue-properties-typed_value.md) – `undefined#/query/definitions/Literal/oneOf/2/properties/typed_value`

*   [Untitled object in okp4-cognitarium](./okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings-items.md) – `undefined#/responses/select/definitions/Results/properties/bindings/items`

*   [Untitled object in okp4-law-stone](./okp4-law-stone-querymsg-oneof-ask-properties-ask.md) – `undefined#/query/oneOf/0/properties/ask`

*   [Untitled object in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-answer.md) – `undefined#/responses/ask/definitions/Answer`

*   [Untitled object in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-result.md) – `undefined#/responses/ask/definitions/Result`

*   [Untitled object in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-substitution.md) – `undefined#/responses/ask/definitions/Substitution`

*   [Untitled object in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-term.md) – `undefined#/responses/ask/definitions/Term`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-instantiatemsg-definitions-bucketconfig.md "BucketConfig is the type of the configuration of a bucket") – `undefined#/instantiate/definitions/BucketConfig`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-instantiatemsg-definitions-bucketlimits.md "BucketLimits is the type of the limits of a bucket") – `undefined#/instantiate/definitions/BucketLimits`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-instantiatemsg-definitions-paginationconfig.md "PaginationConfig is the type carrying configuration for paginated queries") – `undefined#/instantiate/definitions/PaginationConfig`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-executemsg-oneof-storeobject-properties-store_object.md) – `undefined#/execute/oneOf/0/properties/store_object`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-executemsg-oneof-forgetobject-properties-forget_object.md) – `undefined#/execute/oneOf/1/properties/forget_object`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-executemsg-oneof-pinobject-properties-pin_object.md) – `undefined#/execute/oneOf/2/properties/pin_object`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-executemsg-oneof-unpinobject-properties-unpin_object.md) – `undefined#/execute/oneOf/3/properties/unpin_object`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-querymsg-oneof-bucket-properties-bucket.md) – `undefined#/query/oneOf/0/properties/bucket`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-querymsg-oneof-object-properties-object.md) – `undefined#/query/oneOf/1/properties/object`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-querymsg-oneof-objects-properties-objects.md) – `undefined#/query/oneOf/2/properties/objects`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-querymsg-oneof-objectdata-properties-object_data.md) – `undefined#/query/oneOf/3/properties/object_data`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins.md) – `undefined#/query/oneOf/4/properties/object_pins`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-responses-bucketresponse-definitions-bucketconfig.md "BucketConfig is the type of the configuration of a bucket") – `undefined#/responses/bucket/definitions/BucketConfig`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-responses-bucketresponse-definitions-bucketlimits.md "BucketLimits is the type of the limits of a bucket") – `undefined#/responses/bucket/definitions/BucketLimits`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-responses-bucketresponse-definitions-paginationconfig.md "PaginationConfig is the type carrying configuration for paginated queries") – `undefined#/responses/bucket/definitions/PaginationConfig`

*   [Variable](./okp4-cognitarium-querymsg-definitions-objectpattern-oneof-variable.md "A variable") – `undefined#/query/definitions/ObjectPattern/oneOf/0`

*   [Variable](./okp4-cognitarium-querymsg-definitions-predicatepattern-oneof-variable.md "A variable") – `undefined#/query/definitions/PredicatePattern/oneOf/0`

*   [Variable](./okp4-cognitarium-querymsg-definitions-selectitem-oneof-variable.md "Represents a variable") – `undefined#/query/definitions/SelectItem/oneOf/0`

*   [Variable](./okp4-cognitarium-querymsg-definitions-subjectpattern-oneof-variable.md "A variable") – `undefined#/query/definitions/SubjectPattern/oneOf/0`

### Arrays

*   [Untitled array in okp4-cognitarium](./okp4-cognitarium-querymsg-definitions-selectquery-properties-prefixes.md "The prefixes used in the query") – `undefined#/query/definitions/SelectQuery/properties/prefixes`

*   [Untitled array in okp4-cognitarium](./okp4-cognitarium-querymsg-definitions-selectquery-properties-select.md "The items to select") – `undefined#/query/definitions/SelectQuery/properties/select`

*   [Untitled array in okp4-cognitarium](./okp4-cognitarium-querymsg-definitions-selectquery-properties-where.md "The WHERE clause") – `undefined#/query/definitions/SelectQuery/properties/where`

*   [Untitled array in okp4-cognitarium](./okp4-cognitarium-responses-selectresponse-definitions-head-properties-vars.md "The variables selected in the query") – `undefined#/responses/select/definitions/Head/properties/vars`

*   [Untitled array in okp4-cognitarium](./okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings.md "The bindings of the results") – `undefined#/responses/select/definitions/Results/properties/bindings`

*   [Untitled array in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-answer-properties-results.md) – `undefined#/responses/ask/definitions/Answer/properties/results`

*   [Untitled array in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-answer-properties-variables.md) – `undefined#/responses/ask/definitions/Answer/properties/variables`

*   [Untitled array in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-result-properties-substitutions.md) – `undefined#/responses/ask/definitions/Result/properties/substitutions`

*   [Untitled array in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-term-properties-arguments.md) – `undefined#/responses/ask/definitions/Term/properties/arguments`

*   [Untitled array in okp4-objectarium](./okp4-objectarium-responses-objectpinsresponse-properties-data.md "The list of addresses that pinned the object") – `undefined#/responses/object_pins/properties/data`

*   [Untitled array in okp4-objectarium](./okp4-objectarium-responses-objectsresponse-properties-data.md "The list of objects in the bucket") – `undefined#/responses/objects/properties/data`

## Version Note

The schemas linked above follow the JSON Schema Spec version: `http://json-schema.org/draft-07/schema#`
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/execute/definitions/Binary
```

Binary is a wrapper around <code>Vec&lt;u8&gt;</code> to add base64 de/serialization with serde. It also adds some helper methods to help encode inline.

This is only needed as serde-json-{core,wasm} has a horrible encoding for <code>Vec&lt;u8&gt;</code>. See also <https://github.com/CosmWasm/cosmwasm/blob/main/docs/MESSAGE_TYPES.md>.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## Binary Type

`string`
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/execute/definitions/DataInput/oneOf/2/properties/n_triples
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## n\_triples Type

unknown
# N-Triples Schema

```txt
undefined#/execute/definitions/DataInput/oneOf/2
```

Input in [N-Triples](https://www.w3.org/TR/n-triples/) format.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 2 Type

`object` ([N-Triples](okp4-cognitarium-executemsg-definitions-datainput-oneof-n-triples.md))

# 2 Properties

| Property                 | Type          | Required | Nullable       | Defined by                                                                                                                                                                            |
| :----------------------- | :------------ | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [n\_triples](#n_triples) | Not specified | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-datainput-oneof-n-triples-properties-n_triples.md "undefined#/execute/definitions/DataInput/oneOf/2/properties/n_triples") |

## n\_triples



`n_triples`

*   is required

*   Type: unknown

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-datainput-oneof-n-triples-properties-n_triples.md "undefined#/execute/definitions/DataInput/oneOf/2/properties/n_triples")

### n\_triples Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/execute/definitions/DataInput/oneOf/0/properties/rdf_xml
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## rdf\_xml Type

unknown
# RDF XML Schema

```txt
undefined#/execute/definitions/DataInput/oneOf/0
```

Input in [RDF/XML](https://www.w3.org/TR/rdf-syntax-grammar/) format.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`object` ([RDF XML](okp4-cognitarium-executemsg-definitions-datainput-oneof-rdf-xml.md))

# 0 Properties

| Property             | Type          | Required | Nullable       | Defined by                                                                                                                                                                      |
| :------------------- | :------------ | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [rdf\_xml](#rdf_xml) | Not specified | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-datainput-oneof-rdf-xml-properties-rdf_xml.md "undefined#/execute/definitions/DataInput/oneOf/0/properties/rdf_xml") |

## rdf\_xml



`rdf_xml`

*   is required

*   Type: unknown

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-datainput-oneof-rdf-xml-properties-rdf_xml.md "undefined#/execute/definitions/DataInput/oneOf/0/properties/rdf_xml")

### rdf\_xml Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/execute/definitions/DataInput/oneOf/1/properties/turtle
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## turtle Type

unknown
# Turtle Schema

```txt
undefined#/execute/definitions/DataInput/oneOf/1
```

Input in [Turtle](https://www.w3.org/TR/turtle/) format.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`object` ([Turtle](okp4-cognitarium-executemsg-definitions-datainput-oneof-turtle.md))

# 1 Properties

| Property          | Type          | Required | Nullable       | Defined by                                                                                                                                                                   |
| :---------------- | :------------ | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [turtle](#turtle) | Not specified | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-datainput-oneof-turtle-properties-turtle.md "undefined#/execute/definitions/DataInput/oneOf/1/properties/turtle") |

## turtle



`turtle`

*   is required

*   Type: unknown

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-definitions-datainput-oneof-turtle-properties-turtle.md "undefined#/execute/definitions/DataInput/oneOf/1/properties/turtle")

### turtle Type

unknown
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
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/execute/definitions
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## definitions Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/execute/oneOf/0/properties/insert_data/properties/input
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## input Type

unknown
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

| Property        | Type          | Required | Nullable       | Defined by                                                                                                                                                                       |
| :-------------- | :------------ | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [input](#input) | Not specified | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data-properties-input.md "undefined#/execute/oneOf/0/properties/insert_data/properties/input") |

## input



`input`

*   is required

*   Type: unknown

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data-properties-input.md "undefined#/execute/oneOf/0/properties/insert_data/properties/input")

### input Type

unknown
# InsertData Schema

```txt
undefined#/execute/oneOf/0
```

Insert the data as RDF triples in the store. For already existing triples it acts as no-op.

Only the smart contract owner (i.e. the address who instantiated it) is authorized to perform this action.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`object` ([InsertData](okp4-cognitarium-executemsg-oneof-insertdata.md))

# 0 Properties

| Property                     | Type     | Required | Nullable       | Defined by                                                                                                                                     |
| :--------------------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| [insert\_data](#insert_data) | `object` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data.md "undefined#/execute/oneOf/0/properties/insert_data") |

## insert\_data



`insert_data`

*   is required

*   Type: `object` ([Details](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data.md "undefined#/execute/oneOf/0/properties/insert_data")

### insert\_data Type

`object` ([Details](okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data.md))
# ExecuteMsg Schema

```txt
undefined#/execute
```

Execute messages

| Abstract               | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :--------------------- | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Cannot be instantiated | Yes        | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## execute Type

merged type ([ExecuteMsg](okp4-cognitarium-executemsg.md))

one (and only one) of

*   [InsertData](okp4-cognitarium-executemsg-oneof-insertdata.md "check type definition")

# ExecuteMsg Definitions

## Definitions group Binary

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/Binary"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group DataInput

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/DataInput"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_byte_size/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_byte_size/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_byte_size
```

The maximum number of bytes the store can contains. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. If `None`, there is no limit on the number of bytes.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_byte\_size Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_byte_size-anyof-1.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_insert_data_byte_size/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_insert_data_byte_size/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_insert_data_byte_size
```

The maximum number of bytes an insert data query can contains. If `None`, there is no limit on the number of bytes.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_insert\_data\_byte\_size Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_byte_size-anyof-1.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_insert_data_triple_count/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_insert_data_triple_count/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_insert_data_triple_count
```

The maximum number of triples an insert data query can contains (after parsing). If `None`, there is no limit on the number of triples.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_insert\_data\_triple\_count Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_triple_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_triple_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_triple_count-anyof-1.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_query_limit/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_query_limit/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_query_limit
```

The maximum limit of a query, i.e. the maximum number of triples returned by a select query. If `None`, there is no limit on the number of triples returned.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_query\_limit Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_limit.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_limit-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_limit-anyof-1.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_query_variable_count/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_query_variable_count/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_query_variable_count
```

The maximum number of variables a query can select. If `None`, there is no limit on the number of variables.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_query\_variable\_count Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_variable_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_variable_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_variable_count-anyof-1.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_triple_byte_size/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_triple_byte_size/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_triple_byte_size
```

The maximum number of bytes the store can contains for a single triple. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. The limit is used to prevent storing very large triples, especially literals. If `None`, there is no limit on the number of bytes.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_triple\_byte\_size Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_byte_size-anyof-1.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_triple_count/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_triple_count/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/StoreLimits/properties/max_triple_count
```

The maximum number of triples the store can contains. If `None`, there is no limit on the number of triples.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_triple\_count Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_count-anyof-1.md "check type definition")
# StoreLimits Schema

```txt
undefined#/instantiate/definitions/StoreLimits
```

Contains limitations regarding store usages.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## StoreLimits Type

`object` ([StoreLimits](okp4-cognitarium-instantiatemsg-definitions-storelimits.md))

# StoreLimits Properties

| Property                                                          | Type   | Required | Nullable       | Defined by                                                                                                                                                                                                      |
| :---------------------------------------------------------------- | :----- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [max\_byte\_size](#max_byte_size)                                 | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_byte_size.md "undefined#/instantiate/definitions/StoreLimits/properties/max_byte_size")                               |
| [max\_insert\_data\_byte\_size](#max_insert_data_byte_size)       | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_byte_size.md "undefined#/instantiate/definitions/StoreLimits/properties/max_insert_data_byte_size")       |
| [max\_insert\_data\_triple\_count](#max_insert_data_triple_count) | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_triple_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_insert_data_triple_count") |
| [max\_query\_limit](#max_query_limit)                             | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_limit.md "undefined#/instantiate/definitions/StoreLimits/properties/max_query_limit")                           |
| [max\_query\_variable\_count](#max_query_variable_count)          | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_variable_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_query_variable_count")         |
| [max\_triple\_byte\_size](#max_triple_byte_size)                  | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_byte_size.md "undefined#/instantiate/definitions/StoreLimits/properties/max_triple_byte_size")                 |
| [max\_triple\_count](#max_triple_count)                           | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_triple_count")                         |

## max\_byte\_size

The maximum number of bytes the store can contains. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. If `None`, there is no limit on the number of bytes.

`max_byte_size`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_byte_size.md "undefined#/instantiate/definitions/StoreLimits/properties/max_byte_size")

### max\_byte\_size Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_byte_size-anyof-1.md "check type definition")

## max\_insert\_data\_byte\_size

The maximum number of bytes an insert data query can contains. If `None`, there is no limit on the number of bytes.

`max_insert_data_byte_size`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_byte_size.md "undefined#/instantiate/definitions/StoreLimits/properties/max_insert_data_byte_size")

### max\_insert\_data\_byte\_size Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_byte_size-anyof-1.md "check type definition")

## max\_insert\_data\_triple\_count

The maximum number of triples an insert data query can contains (after parsing). If `None`, there is no limit on the number of triples.

`max_insert_data_triple_count`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_triple_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_triple_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_insert_data_triple_count")

### max\_insert\_data\_triple\_count Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_triple_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_triple_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_triple_count-anyof-1.md "check type definition")

## max\_query\_limit

The maximum limit of a query, i.e. the maximum number of triples returned by a select query. If `None`, there is no limit on the number of triples returned.

`max_query_limit`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_limit.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_limit.md "undefined#/instantiate/definitions/StoreLimits/properties/max_query_limit")

### max\_query\_limit Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_limit.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_limit-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_limit-anyof-1.md "check type definition")

## max\_query\_variable\_count

The maximum number of variables a query can select. If `None`, there is no limit on the number of variables.

`max_query_variable_count`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_variable_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_variable_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_query_variable_count")

### max\_query\_variable\_count Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_variable_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_variable_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_variable_count-anyof-1.md "check type definition")

## max\_triple\_byte\_size

The maximum number of bytes the store can contains for a single triple. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. The limit is used to prevent storing very large triples, especially literals. If `None`, there is no limit on the number of bytes.

`max_triple_byte_size`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_byte_size.md "undefined#/instantiate/definitions/StoreLimits/properties/max_triple_byte_size")

### max\_triple\_byte\_size Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_byte_size-anyof-1.md "check type definition")

## max\_triple\_count

The maximum number of triples the store can contains. If `None`, there is no limit on the number of triples.

`max_triple_count`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_triple_count")

### max\_triple\_count Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_count-anyof-1.md "check type definition")
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions/Uint128
```

A thin wrapper around u128 that is using strings for JSON encoding/decoding, such that the full u128 range can be used for clients that convert JSON numbers to floats, like JavaScript and jq.

# Examples

Use `from` to create instances of this and `u128` to get the value out:

````# use cosmwasm_std::Uint128; let a = Uint128::from(123u128); assert_eq!(a.u128(), 123);

let b = Uint128::from(42u64); assert_eq!(b.u128(), 42);

let c = Uint128::from(70u32); assert_eq!(c.u128(), 70); ```
````

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## Uint128 Type

`string`
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/definitions
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## definitions Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/properties/limits/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/instantiate/properties/limits
```

Limitations regarding store usage.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## limits Type

merged type ([Details](okp4-cognitarium-instantiatemsg-properties-limits.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-properties-limits-allof-0.md "check type definition")
# InstantiateMsg Schema

```txt
undefined#/instantiate
```

Instantiate message

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## instantiate Type

`object` ([InstantiateMsg](okp4-cognitarium-instantiatemsg.md))

# instantiate Properties

| Property          | Type   | Required | Nullable       | Defined by                                                                                                          |
| :---------------- | :----- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------ |
| [limits](#limits) | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-properties-limits.md "undefined#/instantiate/properties/limits") |

## limits

Limitations regarding store usage.

`limits`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-properties-limits.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-properties-limits.md "undefined#/instantiate/properties/limits")

### limits Type

merged type ([Details](okp4-cognitarium-instantiatemsg-properties-limits.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-properties-limits-allof-0.md "check type definition")

# InstantiateMsg Definitions

## Definitions group StoreLimits

Reference this group by using

```json
{"$ref":"undefined#/instantiate/definitions/StoreLimits"}
```

| Property                                                          | Type   | Required | Nullable       | Defined by                                                                                                                                                                                                      |
| :---------------------------------------------------------------- | :----- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [max\_byte\_size](#max_byte_size)                                 | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_byte_size.md "undefined#/instantiate/definitions/StoreLimits/properties/max_byte_size")                               |
| [max\_insert\_data\_byte\_size](#max_insert_data_byte_size)       | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_byte_size.md "undefined#/instantiate/definitions/StoreLimits/properties/max_insert_data_byte_size")       |
| [max\_insert\_data\_triple\_count](#max_insert_data_triple_count) | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_triple_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_insert_data_triple_count") |
| [max\_query\_limit](#max_query_limit)                             | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_limit.md "undefined#/instantiate/definitions/StoreLimits/properties/max_query_limit")                           |
| [max\_query\_variable\_count](#max_query_variable_count)          | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_variable_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_query_variable_count")         |
| [max\_triple\_byte\_size](#max_triple_byte_size)                  | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_byte_size.md "undefined#/instantiate/definitions/StoreLimits/properties/max_triple_byte_size")                 |
| [max\_triple\_count](#max_triple_count)                           | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_triple_count")                         |

### max\_byte\_size

The maximum number of bytes the store can contains. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. If `None`, there is no limit on the number of bytes.

`max_byte_size`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_byte_size.md "undefined#/instantiate/definitions/StoreLimits/properties/max_byte_size")

#### max\_byte\_size Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_byte_size-anyof-1.md "check type definition")

### max\_insert\_data\_byte\_size

The maximum number of bytes an insert data query can contains. If `None`, there is no limit on the number of bytes.

`max_insert_data_byte_size`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_byte_size.md "undefined#/instantiate/definitions/StoreLimits/properties/max_insert_data_byte_size")

#### max\_insert\_data\_byte\_size Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_byte_size-anyof-1.md "check type definition")

### max\_insert\_data\_triple\_count

The maximum number of triples an insert data query can contains (after parsing). If `None`, there is no limit on the number of triples.

`max_insert_data_triple_count`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_triple_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_triple_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_insert_data_triple_count")

#### max\_insert\_data\_triple\_count Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_triple_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_triple_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_insert_data_triple_count-anyof-1.md "check type definition")

### max\_query\_limit

The maximum limit of a query, i.e. the maximum number of triples returned by a select query. If `None`, there is no limit on the number of triples returned.

`max_query_limit`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_limit.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_limit.md "undefined#/instantiate/definitions/StoreLimits/properties/max_query_limit")

#### max\_query\_limit Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_limit.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_limit-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_limit-anyof-1.md "check type definition")

### max\_query\_variable\_count

The maximum number of variables a query can select. If `None`, there is no limit on the number of variables.

`max_query_variable_count`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_variable_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_variable_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_query_variable_count")

#### max\_query\_variable\_count Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_variable_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_variable_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_query_variable_count-anyof-1.md "check type definition")

### max\_triple\_byte\_size

The maximum number of bytes the store can contains for a single triple. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. The limit is used to prevent storing very large triples, especially literals. If `None`, there is no limit on the number of bytes.

`max_triple_byte_size`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_byte_size.md "undefined#/instantiate/definitions/StoreLimits/properties/max_triple_byte_size")

#### max\_triple\_byte\_size Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_byte_size-anyof-1.md "check type definition")

### max\_triple\_count

The maximum number of triples the store can contains. If `None`, there is no limit on the number of triples.

`max_triple_count`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_count.md "undefined#/instantiate/definitions/StoreLimits/properties/max_triple_count")

#### max\_triple\_count Type

merged type ([Details](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-instantiatemsg-definitions-storelimits-properties-max_triple_count-anyof-1.md "check type definition")

## Definitions group Uint128

Reference this group by using

```json
{"$ref":"undefined#/instantiate/definitions/Uint128"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/query/definitions/IRI/oneOf/1/properties/full
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## full Type

`string`
# Full Schema

```txt
undefined#/query/definitions/IRI/oneOf/1
```

A full IRI.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`object` ([Full](okp4-cognitarium-querymsg-definitions-iri-oneof-full.md))

# 1 Properties

| Property      | Type     | Required | Nullable       | Defined by                                                                                                                                             |
| :------------ | :------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- |
| [full](#full) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-iri-oneof-full-properties-full.md "undefined#/query/definitions/IRI/oneOf/1/properties/full") |

## full



`full`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-iri-oneof-full-properties-full.md "undefined#/query/definitions/IRI/oneOf/1/properties/full")

### full Type

`string`
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/query/definitions/IRI/oneOf/0/properties/prefixed
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## prefixed Type

`string`
# Prefixed Schema

```txt
undefined#/query/definitions/IRI/oneOf/0
```

An IRI prefixed with a prefix. The prefixed IRI is expanded to a full IRI using the prefix definition specified in the query. For example, the prefixed IRI `rdf:type` is expanded to `http://www.w3.org/1999/02/22-rdf-syntax-ns#type`.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`object` ([Prefixed](okp4-cognitarium-querymsg-definitions-iri-oneof-prefixed.md))

# 0 Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                         |
| :-------------------- | :------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [prefixed](#prefixed) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-iri-oneof-prefixed-properties-prefixed.md "undefined#/query/definitions/IRI/oneOf/0/properties/prefixed") |

## prefixed



`prefixed`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-iri-oneof-prefixed-properties-prefixed.md "undefined#/query/definitions/IRI/oneOf/0/properties/prefixed")

### prefixed Type

`string`
# IRI Schema

```txt
undefined#/query/definitions/IRI
```

Represents an IRI.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## IRI Type

merged type ([IRI](okp4-cognitarium-querymsg-definitions-iri.md))

one (and only one) of

*   [Prefixed](okp4-cognitarium-querymsg-definitions-iri-oneof-prefixed.md "check type definition")

*   [Full](okp4-cognitarium-querymsg-definitions-iri-oneof-full.md "check type definition")
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/query/definitions/Literal/oneOf/1/properties/language_tagged_string/properties/language
```

The [language tag](https://www.w3.org/TR/rdf11-concepts/#dfn-language-tag).

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## language Type

`string`
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/query/definitions/Literal/oneOf/1/properties/language_tagged_string/properties/value
```

The [lexical form](https://www.w3.org/TR/rdf11-concepts/#dfn-lexical-form).

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## value Type

`string`
# Untitled object in okp4-cognitarium Schema

```txt
undefined#/query/definitions/Literal/oneOf/1/properties/language_tagged_string
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## language\_tagged\_string Type

`object` ([Details](okp4-cognitarium-querymsg-definitions-literal-oneof-languagetaggedstring-properties-language_tagged_string.md))

# language\_tagged\_string Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                                                                                                                 |
| :-------------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [language](#language) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-languagetaggedstring-properties-language_tagged_string-properties-language.md "undefined#/query/definitions/Literal/oneOf/1/properties/language_tagged_string/properties/language") |
| [value](#value)       | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-languagetaggedstring-properties-language_tagged_string-properties-value.md "undefined#/query/definitions/Literal/oneOf/1/properties/language_tagged_string/properties/value")       |

## language

The [language tag](https://www.w3.org/TR/rdf11-concepts/#dfn-language-tag).

`language`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-languagetaggedstring-properties-language_tagged_string-properties-language.md "undefined#/query/definitions/Literal/oneOf/1/properties/language_tagged_string/properties/language")

### language Type

`string`

## value

The [lexical form](https://www.w3.org/TR/rdf11-concepts/#dfn-lexical-form).

`value`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-languagetaggedstring-properties-language_tagged_string-properties-value.md "undefined#/query/definitions/Literal/oneOf/1/properties/language_tagged_string/properties/value")

### value Type

`string`
# LanguageTaggedString Schema

```txt
undefined#/query/definitions/Literal/oneOf/1
```

A [language-tagged string](https://www.w3.org/TR/rdf11-concepts/#dfn-language-tagged-string)

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`object` ([LanguageTaggedString](okp4-cognitarium-querymsg-definitions-literal-oneof-languagetaggedstring.md))

# 1 Properties

| Property                                            | Type     | Required | Nullable       | Defined by                                                                                                                                                                                                         |
| :-------------------------------------------------- | :------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [language\_tagged\_string](#language_tagged_string) | `object` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-languagetaggedstring-properties-language_tagged_string.md "undefined#/query/definitions/Literal/oneOf/1/properties/language_tagged_string") |

## language\_tagged\_string



`language_tagged_string`

*   is required

*   Type: `object` ([Details](okp4-cognitarium-querymsg-definitions-literal-oneof-languagetaggedstring-properties-language_tagged_string.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-languagetaggedstring-properties-language_tagged_string.md "undefined#/query/definitions/Literal/oneOf/1/properties/language_tagged_string")

### language\_tagged\_string Type

`object` ([Details](okp4-cognitarium-querymsg-definitions-literal-oneof-languagetaggedstring-properties-language_tagged_string.md))
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/query/definitions/Literal/oneOf/0/properties/simple
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## simple Type

`string`
# Simple Schema

```txt
undefined#/query/definitions/Literal/oneOf/0
```

A [simple literal](https://www.w3.org/TR/rdf11-concepts/#dfn-simple-literal) without datatype or language form.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`object` ([Simple](okp4-cognitarium-querymsg-definitions-literal-oneof-simple.md))

# 0 Properties

| Property          | Type     | Required | Nullable       | Defined by                                                                                                                                                           |
| :---------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [simple](#simple) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-simple-properties-simple.md "undefined#/query/definitions/Literal/oneOf/0/properties/simple") |

## simple



`simple`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-simple-properties-simple.md "undefined#/query/definitions/Literal/oneOf/0/properties/simple")

### simple Type

`string`
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/Literal/oneOf/2/properties/typed_value/properties/datatype/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/Literal/oneOf/2/properties/typed_value/properties/datatype
```

The [datatype IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-datatype-iri).

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## datatype Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue-properties-typed_value-properties-datatype.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue-properties-typed_value-properties-datatype-allof-0.md "check type definition")
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/query/definitions/Literal/oneOf/2/properties/typed_value/properties/value
```

The [lexical form](https://www.w3.org/TR/rdf11-concepts/#dfn-lexical-form).

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## value Type

`string`
# Untitled object in okp4-cognitarium Schema

```txt
undefined#/query/definitions/Literal/oneOf/2/properties/typed_value
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## typed\_value Type

`object` ([Details](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue-properties-typed_value.md))

# typed\_value Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                                                                                 |
| :-------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [datatype](#datatype) | Merged   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue-properties-typed_value-properties-datatype.md "undefined#/query/definitions/Literal/oneOf/2/properties/typed_value/properties/datatype") |
| [value](#value)       | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue-properties-typed_value-properties-value.md "undefined#/query/definitions/Literal/oneOf/2/properties/typed_value/properties/value")       |

## datatype

The [datatype IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-datatype-iri).

`datatype`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue-properties-typed_value-properties-datatype.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue-properties-typed_value-properties-datatype.md "undefined#/query/definitions/Literal/oneOf/2/properties/typed_value/properties/datatype")

### datatype Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue-properties-typed_value-properties-datatype.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue-properties-typed_value-properties-datatype-allof-0.md "check type definition")

## value

The [lexical form](https://www.w3.org/TR/rdf11-concepts/#dfn-lexical-form).

`value`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue-properties-typed_value-properties-value.md "undefined#/query/definitions/Literal/oneOf/2/properties/typed_value/properties/value")

### value Type

`string`
# TypedValue Schema

```txt
undefined#/query/definitions/Literal/oneOf/2
```

A value with a datatype.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 2 Type

`object` ([TypedValue](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue.md))

# 2 Properties

| Property                     | Type     | Required | Nullable       | Defined by                                                                                                                                                                         |
| :--------------------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [typed\_value](#typed_value) | `object` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue-properties-typed_value.md "undefined#/query/definitions/Literal/oneOf/2/properties/typed_value") |

## typed\_value



`typed_value`

*   is required

*   Type: `object` ([Details](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue-properties-typed_value.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue-properties-typed_value.md "undefined#/query/definitions/Literal/oneOf/2/properties/typed_value")

### typed\_value Type

`object` ([Details](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue-properties-typed_value.md))
# Literal Schema

```txt
undefined#/query/definitions/Literal
```

An RDF [literal](https://www.w3.org/TR/rdf11-concepts/#dfn-literal).

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## Literal Type

merged type ([Literal](okp4-cognitarium-querymsg-definitions-literal.md))

one (and only one) of

*   [Simple](okp4-cognitarium-querymsg-definitions-literal-oneof-simple.md "check type definition")

*   [LanguageTaggedString](okp4-cognitarium-querymsg-definitions-literal-oneof-languagetaggedstring.md "check type definition")

*   [TypedValue](okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue.md "check type definition")
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/query/definitions/Node/oneOf/1/properties/blank_node
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## blank\_node Type

`string`
# BlankNode Schema

```txt
undefined#/query/definitions/Node/oneOf/1
```

An RDF [blank node](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node).

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`object` ([BlankNode](okp4-cognitarium-querymsg-definitions-node-oneof-blanknode.md))

# 1 Properties

| Property                   | Type     | Required | Nullable       | Defined by                                                                                                                                                                |
| :------------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [blank\_node](#blank_node) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-node-oneof-blanknode-properties-blank_node.md "undefined#/query/definitions/Node/oneOf/1/properties/blank_node") |

## blank\_node



`blank_node`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-node-oneof-blanknode-properties-blank_node.md "undefined#/query/definitions/Node/oneOf/1/properties/blank_node")

### blank\_node Type

`string`
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/Node/oneOf/0/properties/named_node
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## named\_node Type

unknown
# NamedNode Schema

```txt
undefined#/query/definitions/Node/oneOf/0
```

An RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri).

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`object` ([NamedNode](okp4-cognitarium-querymsg-definitions-node-oneof-namednode.md))

# 0 Properties

| Property                   | Type          | Required | Nullable       | Defined by                                                                                                                                                                |
| :------------------------- | :------------ | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [named\_node](#named_node) | Not specified | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-node-oneof-namednode-properties-named_node.md "undefined#/query/definitions/Node/oneOf/0/properties/named_node") |

## named\_node



`named_node`

*   is required

*   Type: unknown

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-node-oneof-namednode-properties-named_node.md "undefined#/query/definitions/Node/oneOf/0/properties/named_node")

### named\_node Type

unknown
# Node Schema

```txt
undefined#/query/definitions/Node
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## Node Type

merged type ([Node](okp4-cognitarium-querymsg-definitions-node.md))

one (and only one) of

*   [NamedNode](okp4-cognitarium-querymsg-definitions-node-oneof-namednode.md "check type definition")

*   [BlankNode](okp4-cognitarium-querymsg-definitions-node-oneof-blanknode.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/ObjectPattern/oneOf/2/properties/literal
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## literal Type

unknown
# Literal Schema

```txt
undefined#/query/definitions/ObjectPattern/oneOf/2
```

An RDF [literal](https://www.w3.org/TR/rdf11-concepts/#dfn-literal), i.e. a simple literal, a language-tagged string or a typed value.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 2 Type

`object` ([Literal](okp4-cognitarium-querymsg-definitions-objectpattern-oneof-literal.md))

# 2 Properties

| Property            | Type          | Required | Nullable       | Defined by                                                                                                                                                                          |
| :------------------ | :------------ | :------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [literal](#literal) | Not specified | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-objectpattern-oneof-literal-properties-literal.md "undefined#/query/definitions/ObjectPattern/oneOf/2/properties/literal") |

## literal



`literal`

*   is required

*   Type: unknown

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-objectpattern-oneof-literal-properties-literal.md "undefined#/query/definitions/ObjectPattern/oneOf/2/properties/literal")

### literal Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/ObjectPattern/oneOf/1/properties/node
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## node Type

unknown
# Node Schema

```txt
undefined#/query/definitions/ObjectPattern/oneOf/1
```

A node, i.e. an IRI or a blank node.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`object` ([Node](okp4-cognitarium-querymsg-definitions-objectpattern-oneof-node.md))

# 1 Properties

| Property      | Type          | Required | Nullable       | Defined by                                                                                                                                                                 |
| :------------ | :------------ | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [node](#node) | Not specified | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-objectpattern-oneof-node-properties-node.md "undefined#/query/definitions/ObjectPattern/oneOf/1/properties/node") |

## node



`node`

*   is required

*   Type: unknown

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-objectpattern-oneof-node-properties-node.md "undefined#/query/definitions/ObjectPattern/oneOf/1/properties/node")

### node Type

unknown
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/query/definitions/ObjectPattern/oneOf/0/properties/variable
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## variable Type

`string`
# Variable Schema

```txt
undefined#/query/definitions/ObjectPattern/oneOf/0
```

A variable.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`object` ([Variable](okp4-cognitarium-querymsg-definitions-objectpattern-oneof-variable.md))

# 0 Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                                             |
| :-------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [variable](#variable) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-objectpattern-oneof-variable-properties-variable.md "undefined#/query/definitions/ObjectPattern/oneOf/0/properties/variable") |

## variable



`variable`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-objectpattern-oneof-variable-properties-variable.md "undefined#/query/definitions/ObjectPattern/oneOf/0/properties/variable")

### variable Type

`string`
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
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/PredicatePattern/oneOf/1/properties/node
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## node Type

unknown
# Node Schema

```txt
undefined#/query/definitions/PredicatePattern/oneOf/1
```

A node, i.e. an IRI or a blank node.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`object` ([Node](okp4-cognitarium-querymsg-definitions-predicatepattern-oneof-node.md))

# 1 Properties

| Property      | Type          | Required | Nullable       | Defined by                                                                                                                                                                       |
| :------------ | :------------ | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [node](#node) | Not specified | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-predicatepattern-oneof-node-properties-node.md "undefined#/query/definitions/PredicatePattern/oneOf/1/properties/node") |

## node



`node`

*   is required

*   Type: unknown

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-predicatepattern-oneof-node-properties-node.md "undefined#/query/definitions/PredicatePattern/oneOf/1/properties/node")

### node Type

unknown
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/query/definitions/PredicatePattern/oneOf/0/properties/variable
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## variable Type

`string`
# Variable Schema

```txt
undefined#/query/definitions/PredicatePattern/oneOf/0
```

A variable.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`object` ([Variable](okp4-cognitarium-querymsg-definitions-predicatepattern-oneof-variable.md))

# 0 Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                                                   |
| :-------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [variable](#variable) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-predicatepattern-oneof-variable-properties-variable.md "undefined#/query/definitions/PredicatePattern/oneOf/0/properties/variable") |

## variable



`variable`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-predicatepattern-oneof-variable-properties-variable.md "undefined#/query/definitions/PredicatePattern/oneOf/0/properties/variable")

### variable Type

`string`
# PredicatePattern Schema

```txt
undefined#/query/definitions/PredicatePattern
```

Represents a predicate pattern in a \[TriplePattern] that can be either a variable or a node.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## PredicatePattern Type

merged type ([PredicatePattern](okp4-cognitarium-querymsg-definitions-predicatepattern.md))

one (and only one) of

*   [Variable](okp4-cognitarium-querymsg-definitions-predicatepattern-oneof-variable.md "check type definition")

*   [Node](okp4-cognitarium-querymsg-definitions-predicatepattern-oneof-node.md "check type definition")
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/query/definitions/Prefix/properties/namespace
```

The namespace associated with the prefix.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## namespace Type

`string`
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/query/definitions/Prefix/properties/prefix
```

The prefix.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## prefix Type

`string`
# Prefix Schema

```txt
undefined#/query/definitions/Prefix
```

Represents a prefix in a \[SelectQuery]. A prefix is a shortcut for a namespace used in the query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## Prefix Type

`object` ([Prefix](okp4-cognitarium-querymsg-definitions-prefix.md))

# Prefix Properties

| Property                | Type     | Required | Nullable       | Defined by                                                                                                                                          |
| :---------------------- | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| [namespace](#namespace) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-namespace.md "undefined#/query/definitions/Prefix/properties/namespace") |
| [prefix](#prefix)       | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-prefix.md "undefined#/query/definitions/Prefix/properties/prefix")       |

## namespace

The namespace associated with the prefix.

`namespace`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-namespace.md "undefined#/query/definitions/Prefix/properties/namespace")

### namespace Type

`string`

## prefix

The prefix.

`prefix`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-prefix.md "undefined#/query/definitions/Prefix/properties/prefix")

### prefix Type

`string`
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/query/definitions/SelectItem/oneOf/0/properties/variable
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## variable Type

`string`
# Variable Schema

```txt
undefined#/query/definitions/SelectItem/oneOf/0
```

Represents a variable.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`object` ([Variable](okp4-cognitarium-querymsg-definitions-selectitem-oneof-variable.md))

# 0 Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                                       |
| :-------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [variable](#variable) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectitem-oneof-variable-properties-variable.md "undefined#/query/definitions/SelectItem/oneOf/0/properties/variable") |

## variable



`variable`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectitem-oneof-variable-properties-variable.md "undefined#/query/definitions/SelectItem/oneOf/0/properties/variable")

### variable Type

`string`
# SelectItem Schema

```txt
undefined#/query/definitions/SelectItem
```

Represents an item to select in a \[SelectQuery].

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## SelectItem Type

merged type ([SelectItem](okp4-cognitarium-querymsg-definitions-selectitem.md))

one (and only one) of

*   [Variable](okp4-cognitarium-querymsg-definitions-selectitem-oneof-variable.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/SelectQuery/properties/limit
```

The maximum number of results to return. If `None`, there is no limit. Note: the value of the limit cannot exceed the maximum query limit defined in the store limitations.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## limit Type

`integer`

## limit Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint64`
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/SelectQuery/properties/prefixes/items
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## items Type

unknown
# Untitled array in okp4-cognitarium Schema

```txt
undefined#/query/definitions/SelectQuery/properties/prefixes
```

The prefixes used in the query.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## prefixes Type

unknown\[]
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/SelectQuery/properties/select/items
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## items Type

unknown
# Untitled array in okp4-cognitarium Schema

```txt
undefined#/query/definitions/SelectQuery/properties/select
```

The items to select. Note: the number of items to select cannot exceed the maximum query variable count defined in the store limitations.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## select Type

unknown\[]
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/SelectQuery/properties/where/items
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## items Type

unknown
# Untitled array in okp4-cognitarium Schema

```txt
undefined#/query/definitions/SelectQuery/properties/where
```

The WHERE clause. If `None`, there is no WHERE clause, i.e. all triples are returned without filtering.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## where Type

unknown\[]
# SelectQuery Schema

```txt
undefined#/query/definitions/SelectQuery
```

Represents a SELECT query over the triple store, allowing to select variables to return and to filter the results.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## SelectQuery Type

`object` ([SelectQuery](okp4-cognitarium-querymsg-definitions-selectquery.md))

# SelectQuery Properties

| Property              | Type      | Required | Nullable       | Defined by                                                                                                                                                  |
| :-------------------- | :-------- | :------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [limit](#limit)       | `integer` | Optional | can be null    | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-limit.md "undefined#/query/definitions/SelectQuery/properties/limit")       |
| [prefixes](#prefixes) | `array`   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-prefixes.md "undefined#/query/definitions/SelectQuery/properties/prefixes") |
| [select](#select)     | `array`   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-select.md "undefined#/query/definitions/SelectQuery/properties/select")     |
| [where](#where)       | `array`   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-where.md "undefined#/query/definitions/SelectQuery/properties/where")       |

## limit

The maximum number of results to return. If `None`, there is no limit. Note: the value of the limit cannot exceed the maximum query limit defined in the store limitations.

`limit`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-limit.md "undefined#/query/definitions/SelectQuery/properties/limit")

### limit Type

`integer`

### limit Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint64`

## prefixes

The prefixes used in the query.

`prefixes`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-prefixes.md "undefined#/query/definitions/SelectQuery/properties/prefixes")

### prefixes Type

unknown\[]

## select

The items to select. Note: the number of items to select cannot exceed the maximum query variable count defined in the store limitations.

`select`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-select.md "undefined#/query/definitions/SelectQuery/properties/select")

### select Type

unknown\[]

## where

The WHERE clause. If `None`, there is no WHERE clause, i.e. all triples are returned without filtering.

`where`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-where.md "undefined#/query/definitions/SelectQuery/properties/where")

### where Type

unknown\[]
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/SimpleWhereCondition/oneOf/0/properties/triple_pattern
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## triple\_pattern Type

unknown
# TriplePattern Schema

```txt
undefined#/query/definitions/SimpleWhereCondition/oneOf/0
```

Represents a triple pattern, i.e. a condition on a triple based on its subject, predicate and object.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`object` ([TriplePattern](okp4-cognitarium-querymsg-definitions-simplewherecondition-oneof-triplepattern.md))

# 0 Properties

| Property                           | Type          | Required | Nullable       | Defined by                                                                                                                                                                                                            |
| :--------------------------------- | :------------ | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [triple\_pattern](#triple_pattern) | Not specified | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-simplewherecondition-oneof-triplepattern-properties-triple_pattern.md "undefined#/query/definitions/SimpleWhereCondition/oneOf/0/properties/triple_pattern") |

## triple\_pattern



`triple_pattern`

*   is required

*   Type: unknown

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-simplewherecondition-oneof-triplepattern-properties-triple_pattern.md "undefined#/query/definitions/SimpleWhereCondition/oneOf/0/properties/triple_pattern")

### triple\_pattern Type

unknown
# SimpleWhereCondition Schema

```txt
undefined#/query/definitions/SimpleWhereCondition
```

Represents a simple condition in a \[WhereCondition].

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## SimpleWhereCondition Type

merged type ([SimpleWhereCondition](okp4-cognitarium-querymsg-definitions-simplewherecondition.md))

one (and only one) of

*   [TriplePattern](okp4-cognitarium-querymsg-definitions-simplewherecondition-oneof-triplepattern.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/SubjectPattern/oneOf/1/properties/node
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## node Type

unknown
# Node Schema

```txt
undefined#/query/definitions/SubjectPattern/oneOf/1
```

A node, i.e. an IRI or a blank node.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`object` ([Node](okp4-cognitarium-querymsg-definitions-subjectpattern-oneof-node.md))

# 1 Properties

| Property      | Type          | Required | Nullable       | Defined by                                                                                                                                                                   |
| :------------ | :------------ | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [node](#node) | Not specified | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-subjectpattern-oneof-node-properties-node.md "undefined#/query/definitions/SubjectPattern/oneOf/1/properties/node") |

## node



`node`

*   is required

*   Type: unknown

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-subjectpattern-oneof-node-properties-node.md "undefined#/query/definitions/SubjectPattern/oneOf/1/properties/node")

### node Type

unknown
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/query/definitions/SubjectPattern/oneOf/0/properties/variable
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## variable Type

`string`
# Variable Schema

```txt
undefined#/query/definitions/SubjectPattern/oneOf/0
```

A variable.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`object` ([Variable](okp4-cognitarium-querymsg-definitions-subjectpattern-oneof-variable.md))

# 0 Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                                               |
| :-------------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [variable](#variable) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-subjectpattern-oneof-variable-properties-variable.md "undefined#/query/definitions/SubjectPattern/oneOf/0/properties/variable") |

## variable



`variable`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-subjectpattern-oneof-variable-properties-variable.md "undefined#/query/definitions/SubjectPattern/oneOf/0/properties/variable")

### variable Type

`string`
# SubjectPattern Schema

```txt
undefined#/query/definitions/SubjectPattern
```

Represents a subject pattern in a \[TriplePattern] that can be either a variable or a node.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## SubjectPattern Type

merged type ([SubjectPattern](okp4-cognitarium-querymsg-definitions-subjectpattern.md))

one (and only one) of

*   [Variable](okp4-cognitarium-querymsg-definitions-subjectpattern-oneof-variable.md "check type definition")

*   [Node](okp4-cognitarium-querymsg-definitions-subjectpattern-oneof-node.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/TriplePattern/properties/object/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/TriplePattern/properties/object
```

The object of the triple pattern.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## object Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object-allof-0.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/TriplePattern/properties/predicate/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/TriplePattern/properties/predicate
```

The predicate of the triple pattern.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## predicate Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate-allof-0.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/TriplePattern/properties/subject/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/TriplePattern/properties/subject
```

The subject of the triple pattern.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## subject Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject-allof-0.md "check type definition")
# TriplePattern Schema

```txt
undefined#/query/definitions/TriplePattern
```

Represents a triple pattern in a \[SimpleWhereCondition].

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## TriplePattern Type

`object` ([TriplePattern](okp4-cognitarium-querymsg-definitions-triplepattern.md))

# TriplePattern Properties

| Property                | Type   | Required | Nullable       | Defined by                                                                                                                                                        |
| :---------------------- | :----- | :------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [object](#object)       | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object.md "undefined#/query/definitions/TriplePattern/properties/object")       |
| [predicate](#predicate) | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate.md "undefined#/query/definitions/TriplePattern/properties/predicate") |
| [subject](#subject)     | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject.md "undefined#/query/definitions/TriplePattern/properties/subject")     |

## object

The object of the triple pattern.

`object`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object.md "undefined#/query/definitions/TriplePattern/properties/object")

### object Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object-allof-0.md "check type definition")

## predicate

The predicate of the triple pattern.

`predicate`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate.md "undefined#/query/definitions/TriplePattern/properties/predicate")

### predicate Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate-allof-0.md "check type definition")

## subject

The subject of the triple pattern.

`subject`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject.md "undefined#/query/definitions/TriplePattern/properties/subject")

### subject Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject-allof-0.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions/WhereCondition/oneOf/0/properties/simple
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## simple Type

unknown
# Simple Schema

```txt
undefined#/query/definitions/WhereCondition/oneOf/0
```

Represents a simple condition.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`object` ([Simple](okp4-cognitarium-querymsg-definitions-wherecondition-oneof-simple.md))

# 0 Properties

| Property          | Type          | Required | Nullable       | Defined by                                                                                                                                                                         |
| :---------------- | :------------ | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [simple](#simple) | Not specified | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-wherecondition-oneof-simple-properties-simple.md "undefined#/query/definitions/WhereCondition/oneOf/0/properties/simple") |

## simple



`simple`

*   is required

*   Type: unknown

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-wherecondition-oneof-simple-properties-simple.md "undefined#/query/definitions/WhereCondition/oneOf/0/properties/simple")

### simple Type

unknown
# WhereCondition Schema

```txt
undefined#/query/definitions/WhereCondition
```

Represents a condition in a \[WhereClause].

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## WhereCondition Type

merged type ([WhereCondition](okp4-cognitarium-querymsg-definitions-wherecondition.md))

one (and only one) of

*   [Simple](okp4-cognitarium-querymsg-definitions-wherecondition-oneof-simple.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/definitions
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## definitions Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/oneOf/1/properties/select/properties/query/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/query/oneOf/1/properties/select/properties/query
```

The query to execute.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## query Type

merged type ([Details](okp4-cognitarium-querymsg-oneof-select-properties-select-properties-query.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-oneof-select-properties-select-properties-query-allof-0.md "check type definition")
# Untitled object in okp4-cognitarium Schema

```txt
undefined#/query/oneOf/1/properties/select
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## select Type

`object` ([Details](okp4-cognitarium-querymsg-oneof-select-properties-select.md))

# select Properties

| Property        | Type   | Required | Nullable       | Defined by                                                                                                                                                     |
| :-------------- | :----- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [query](#query) | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-oneof-select-properties-select-properties-query.md "undefined#/query/oneOf/1/properties/select/properties/query") |

## query

The query to execute.

`query`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-querymsg-oneof-select-properties-select-properties-query.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-oneof-select-properties-select-properties-query.md "undefined#/query/oneOf/1/properties/select/properties/query")

### query Type

merged type ([Details](okp4-cognitarium-querymsg-oneof-select-properties-select-properties-query.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-oneof-select-properties-select-properties-query-allof-0.md "check type definition")
# Select Schema

```txt
undefined#/query/oneOf/1
```

Returns the resources matching the criteria defined by the provided query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`object` ([Select](okp4-cognitarium-querymsg-oneof-select.md))

# 1 Properties

| Property          | Type     | Required | Nullable       | Defined by                                                                                                                   |
| :---------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| [select](#select) | `object` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-oneof-select-properties-select.md "undefined#/query/oneOf/1/properties/select") |

## select



`select`

*   is required

*   Type: `object` ([Details](okp4-cognitarium-querymsg-oneof-select-properties-select.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-oneof-select-properties-select.md "undefined#/query/oneOf/1/properties/select")

### select Type

`object` ([Details](okp4-cognitarium-querymsg-oneof-select-properties-select.md))
# Store Schema

```txt
undefined#/query/oneOf/0
```

Returns information about the triple store.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`string` ([Store](okp4-cognitarium-querymsg-oneof-store.md))

## 0 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value     | Explanation |
| :-------- | :---------- |
| `"store"` |             |
# QueryMsg Schema

```txt
undefined#/query
```

Query messages

| Abstract               | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :--------------------- | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Cannot be instantiated | Yes        | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## query Type

merged type ([QueryMsg](okp4-cognitarium-querymsg.md))

one (and only one) of

*   [Store](okp4-cognitarium-querymsg-oneof-store.md "check type definition")

*   [Select](okp4-cognitarium-querymsg-oneof-select.md "check type definition")

# QueryMsg Definitions

## Definitions group IRI

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/IRI"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group Literal

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/Literal"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group Node

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/Node"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group ObjectPattern

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/ObjectPattern"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group PredicatePattern

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/PredicatePattern"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group Prefix

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/Prefix"}
```

| Property                | Type     | Required | Nullable       | Defined by                                                                                                                                          |
| :---------------------- | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| [namespace](#namespace) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-namespace.md "undefined#/query/definitions/Prefix/properties/namespace") |
| [prefix](#prefix)       | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-prefix.md "undefined#/query/definitions/Prefix/properties/prefix")       |

### namespace

The namespace associated with the prefix.

`namespace`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-namespace.md "undefined#/query/definitions/Prefix/properties/namespace")

#### namespace Type

`string`

### prefix

The prefix.

`prefix`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-prefix-properties-prefix.md "undefined#/query/definitions/Prefix/properties/prefix")

#### prefix Type

`string`

## Definitions group SelectItem

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/SelectItem"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group SelectQuery

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/SelectQuery"}
```

| Property              | Type      | Required | Nullable       | Defined by                                                                                                                                                  |
| :-------------------- | :-------- | :------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [limit](#limit)       | `integer` | Optional | can be null    | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-limit.md "undefined#/query/definitions/SelectQuery/properties/limit")       |
| [prefixes](#prefixes) | `array`   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-prefixes.md "undefined#/query/definitions/SelectQuery/properties/prefixes") |
| [select](#select)     | `array`   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-select.md "undefined#/query/definitions/SelectQuery/properties/select")     |
| [where](#where)       | `array`   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-where.md "undefined#/query/definitions/SelectQuery/properties/where")       |

### limit

The maximum number of results to return. If `None`, there is no limit. Note: the value of the limit cannot exceed the maximum query limit defined in the store limitations.

`limit`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-limit.md "undefined#/query/definitions/SelectQuery/properties/limit")

#### limit Type

`integer`

#### limit Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint64`

### prefixes

The prefixes used in the query.

`prefixes`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-prefixes.md "undefined#/query/definitions/SelectQuery/properties/prefixes")

#### prefixes Type

unknown\[]

### select

The items to select. Note: the number of items to select cannot exceed the maximum query variable count defined in the store limitations.

`select`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-select.md "undefined#/query/definitions/SelectQuery/properties/select")

#### select Type

unknown\[]

### where

The WHERE clause. If `None`, there is no WHERE clause, i.e. all triples are returned without filtering.

`where`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-selectquery-properties-where.md "undefined#/query/definitions/SelectQuery/properties/where")

#### where Type

unknown\[]

## Definitions group SimpleWhereCondition

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/SimpleWhereCondition"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group SubjectPattern

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/SubjectPattern"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group TriplePattern

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/TriplePattern"}
```

| Property                | Type   | Required | Nullable       | Defined by                                                                                                                                                        |
| :---------------------- | :----- | :------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [object](#object)       | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object.md "undefined#/query/definitions/TriplePattern/properties/object")       |
| [predicate](#predicate) | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate.md "undefined#/query/definitions/TriplePattern/properties/predicate") |
| [subject](#subject)     | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject.md "undefined#/query/definitions/TriplePattern/properties/subject")     |

### object

The object of the triple pattern.

`object`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object.md "undefined#/query/definitions/TriplePattern/properties/object")

#### object Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-object-allof-0.md "check type definition")

### predicate

The predicate of the triple pattern.

`predicate`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate.md "undefined#/query/definitions/TriplePattern/properties/predicate")

#### predicate Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-predicate-allof-0.md "check type definition")

### subject

The subject of the triple pattern.

`subject`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject.md "undefined#/query/definitions/TriplePattern/properties/subject")

#### subject Type

merged type ([Details](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-querymsg-definitions-triplepattern-properties-subject-allof-0.md "check type definition")

## Definitions group WhereCondition

Reference this group by using

```json
{"$ref":"undefined#/query/definitions/WhereCondition"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Head/properties/vars/items
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## items Type

`string`
# Untitled array in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Head/properties/vars
```

The variables selected in the query.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## vars Type

`string[]`
# Head Schema

```txt
undefined#/responses/select/definitions/Head
```

Represents the head of a \[SelectResponse].

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## Head Type

`object` ([Head](okp4-cognitarium-responses-selectresponse-definitions-head.md))

# Head Properties

| Property      | Type    | Required | Nullable       | Defined by                                                                                                                                                       |
| :------------ | :------ | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [vars](#vars) | `array` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-head-properties-vars.md "undefined#/responses/select/definitions/Head/properties/vars") |

## vars

The variables selected in the query.

`vars`

*   is required

*   Type: `string[]`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-head-properties-vars.md "undefined#/responses/select/definitions/Head/properties/vars")

### vars Type

`string[]`
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/IRI/oneOf/1/properties/full
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## full Type

`string`
# Full Schema

```txt
undefined#/responses/select/definitions/IRI/oneOf/1
```

A full IRI.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`object` ([Full](okp4-cognitarium-responses-selectresponse-definitions-iri-oneof-full.md))

# 1 Properties

| Property      | Type     | Required | Nullable       | Defined by                                                                                                                                                                        |
| :------------ | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [full](#full) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-iri-oneof-full-properties-full.md "undefined#/responses/select/definitions/IRI/oneOf/1/properties/full") |

## full



`full`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-iri-oneof-full-properties-full.md "undefined#/responses/select/definitions/IRI/oneOf/1/properties/full")

### full Type

`string`
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/IRI/oneOf/0/properties/prefixed
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## prefixed Type

`string`
# Prefixed Schema

```txt
undefined#/responses/select/definitions/IRI/oneOf/0
```

An IRI prefixed with a prefix. The prefixed IRI is expanded to a full IRI using the prefix definition specified in the query. For example, the prefixed IRI `rdf:type` is expanded to `http://www.w3.org/1999/02/22-rdf-syntax-ns#type`.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`object` ([Prefixed](okp4-cognitarium-responses-selectresponse-definitions-iri-oneof-prefixed.md))

# 0 Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                                                    |
| :-------------------- | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [prefixed](#prefixed) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-iri-oneof-prefixed-properties-prefixed.md "undefined#/responses/select/definitions/IRI/oneOf/0/properties/prefixed") |

## prefixed



`prefixed`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-iri-oneof-prefixed-properties-prefixed.md "undefined#/responses/select/definitions/IRI/oneOf/0/properties/prefixed")

### prefixed Type

`string`
# IRI Schema

```txt
undefined#/responses/select/definitions/IRI
```

Represents an IRI.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## IRI Type

merged type ([IRI](okp4-cognitarium-responses-selectresponse-definitions-iri.md))

one (and only one) of

*   [Prefixed](okp4-cognitarium-responses-selectresponse-definitions-iri-oneof-prefixed.md "check type definition")

*   [Full](okp4-cognitarium-responses-selectresponse-definitions-iri-oneof-full.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Results/properties/bindings/items/additionalProperties
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## additionalProperties Type

unknown
# Untitled object in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Results/properties/bindings/items
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## items Type

`object` ([Details](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings-items.md))

# items Properties

| Property              | Type          | Required | Nullable       | Defined by                                                                                                                                                                                                                           |
| :-------------------- | :------------ | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Additional Properties | Not specified | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings-items-additionalproperties.md "undefined#/responses/select/definitions/Results/properties/bindings/items/additionalProperties") |

## Additional Properties

Additional properties are allowed, as long as they follow this schema:



*   is optional

*   Type: unknown

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings-items-additionalproperties.md "undefined#/responses/select/definitions/Results/properties/bindings/items/additionalProperties")

### additionalProperties Type

unknown
# Untitled array in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Results/properties/bindings
```

The bindings of the results.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## bindings Type

`object[]` ([Details](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings-items.md))
# Results Schema

```txt
undefined#/responses/select/definitions/Results
```

Represents the results of a \[SelectResponse].

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## Results Type

`object` ([Results](okp4-cognitarium-responses-selectresponse-definitions-results.md))

# Results Properties

| Property              | Type    | Required | Nullable       | Defined by                                                                                                                                                                     |
| :-------------------- | :------ | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [bindings](#bindings) | `array` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings.md "undefined#/responses/select/definitions/Results/properties/bindings") |

## bindings

The bindings of the results.

`bindings`

*   is required

*   Type: `object[]` ([Details](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings-items.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings.md "undefined#/responses/select/definitions/Results/properties/bindings")

### bindings Type

`object[]` ([Details](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings-items.md))
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/2/properties/type
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## type Type

`string`

## type Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value          | Explanation |
| :------------- | :---------- |
| `"blank_node"` |             |
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/2/properties/value
```

The identifier of the blank node.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## value Type

`string`
# BlankNode Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/2
```

Represents a blank node.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 2 Type

`object` ([BlankNode](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-blanknode.md))

# 2 Properties

| Property        | Type     | Required | Nullable       | Defined by                                                                                                                                                                                   |
| :-------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [type](#type)   | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-blanknode-properties-type.md "undefined#/responses/select/definitions/Value/oneOf/2/properties/type")   |
| [value](#value) | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-blanknode-properties-value.md "undefined#/responses/select/definitions/Value/oneOf/2/properties/value") |

## type



`type`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-blanknode-properties-type.md "undefined#/responses/select/definitions/Value/oneOf/2/properties/type")

### type Type

`string`

### type Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value          | Explanation |
| :------------- | :---------- |
| `"blank_node"` |             |

## value

The identifier of the blank node.

`value`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-blanknode-properties-value.md "undefined#/responses/select/definitions/Value/oneOf/2/properties/value")

### value Type

`string`
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/1/properties/datatype/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/1/properties/datatype/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/1/properties/datatype
```

The datatype of the literal.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## datatype Type

merged type ([Details](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-datatype.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-datatype-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-datatype-anyof-1.md "check type definition")
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/1/properties/type
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## type Type

`string`

## type Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value       | Explanation |
| :---------- | :---------- |
| `"literal"` |             |
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/1/properties/value
```

The value of the literal.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## value Type

`string`
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/1/properties/xml:lang
```

The language tag of the literal.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## xml:lang Type

`string`
# Literal Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/1
```

Represents a literal S with optional language tag L or datatype IRI D.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`object` ([Literal](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal.md))

# 1 Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                                                       |
| :-------------------- | :------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [datatype](#datatype) | Merged   | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-datatype.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/datatype") |
| [type](#type)         | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-type.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/type")         |
| [value](#value)       | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-value.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/value")       |
| [xml:lang](#xmllang)  | `string` | Optional | can be null    | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-xmllang.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/xml:lang")  |

## datatype

The datatype of the literal.

`datatype`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-datatype.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-datatype.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/datatype")

### datatype Type

merged type ([Details](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-datatype.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-datatype-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-datatype-anyof-1.md "check type definition")

## type



`type`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-type.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/type")

### type Type

`string`

### type Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value       | Explanation |
| :---------- | :---------- |
| `"literal"` |             |

## value

The value of the literal.

`value`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-value.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/value")

### value Type

`string`

## xml:lang

The language tag of the literal.

`xml:lang`

*   is optional

*   Type: `string`

*   can be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal-properties-xmllang.md "undefined#/responses/select/definitions/Value/oneOf/1/properties/xml:lang")

### xml:lang Type

`string`
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/0/properties/type
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## type Type

`string`

## type Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value     | Explanation |
| :-------- | :---------- |
| `"u_r_i"` |             |
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/0/properties/value/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/0/properties/value
```

The value of the IRI.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## value Type

merged type ([Details](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-uri-properties-value.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-uri-properties-value-allof-0.md "check type definition")
# URI Schema

```txt
undefined#/responses/select/definitions/Value/oneOf/0
```

Represents an IRI.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

`object` ([URI](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-uri.md))

# 0 Properties

| Property        | Type     | Required | Nullable       | Defined by                                                                                                                                                                             |
| :-------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [type](#type)   | `string` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-uri-properties-type.md "undefined#/responses/select/definitions/Value/oneOf/0/properties/type")   |
| [value](#value) | Merged   | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-uri-properties-value.md "undefined#/responses/select/definitions/Value/oneOf/0/properties/value") |

## type



`type`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-uri-properties-type.md "undefined#/responses/select/definitions/Value/oneOf/0/properties/type")

### type Type

`string`

### type Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value     | Explanation |
| :-------- | :---------- |
| `"u_r_i"` |             |

## value

The value of the IRI.

`value`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-uri-properties-value.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-uri-properties-value.md "undefined#/responses/select/definitions/Value/oneOf/0/properties/value")

### value Type

merged type ([Details](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-uri-properties-value.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-uri-properties-value-allof-0.md "check type definition")
# Value Schema

```txt
undefined#/responses/select/definitions/Value
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## Value Type

merged type ([Value](okp4-cognitarium-responses-selectresponse-definitions-value.md))

one (and only one) of

*   [URI](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-uri.md "check type definition")

*   [Literal](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal.md "check type definition")

*   [BlankNode](okp4-cognitarium-responses-selectresponse-definitions-value-oneof-blanknode.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/select/definitions
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## definitions Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/select/properties/head/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/select/properties/head
```

The head of the response, i.e. the set of variables mentioned in the results.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## head Type

merged type ([Details](okp4-cognitarium-responses-selectresponse-properties-head.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-selectresponse-properties-head-allof-0.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/select/properties/results/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/select/properties/results
```

The results of the select query.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## results Type

merged type ([Details](okp4-cognitarium-responses-selectresponse-properties-results.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-selectresponse-properties-results-allof-0.md "check type definition")
# SelectResponse Schema

```txt
undefined#/responses/select
```

Represents the response of a \[QueryMsg::Select] query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## select Type

`object` ([SelectResponse](okp4-cognitarium-responses-selectresponse.md))

# select Properties

| Property            | Type   | Required | Nullable       | Defined by                                                                                                                           |
| :------------------ | :----- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------- |
| [head](#head)       | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-properties-head.md "undefined#/responses/select/properties/head")       |
| [results](#results) | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-properties-results.md "undefined#/responses/select/properties/results") |

## head

The head of the response, i.e. the set of variables mentioned in the results.

`head`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-selectresponse-properties-head.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-properties-head.md "undefined#/responses/select/properties/head")

### head Type

merged type ([Details](okp4-cognitarium-responses-selectresponse-properties-head.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-selectresponse-properties-head-allof-0.md "check type definition")

## results

The results of the select query.

`results`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-selectresponse-properties-results.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-properties-results.md "undefined#/responses/select/properties/results")

### results Type

merged type ([Details](okp4-cognitarium-responses-selectresponse-properties-results.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-selectresponse-properties-results-allof-0.md "check type definition")

# SelectResponse Definitions

## Definitions group Head

Reference this group by using

```json
{"$ref":"undefined#/responses/select/definitions/Head"}
```

| Property      | Type    | Required | Nullable       | Defined by                                                                                                                                                       |
| :------------ | :------ | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [vars](#vars) | `array` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-head-properties-vars.md "undefined#/responses/select/definitions/Head/properties/vars") |

### vars

The variables selected in the query.

`vars`

*   is required

*   Type: `string[]`

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-head-properties-vars.md "undefined#/responses/select/definitions/Head/properties/vars")

#### vars Type

`string[]`

## Definitions group IRI

Reference this group by using

```json
{"$ref":"undefined#/responses/select/definitions/IRI"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group Results

Reference this group by using

```json
{"$ref":"undefined#/responses/select/definitions/Results"}
```

| Property              | Type    | Required | Nullable       | Defined by                                                                                                                                                                     |
| :-------------------- | :------ | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [bindings](#bindings) | `array` | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings.md "undefined#/responses/select/definitions/Results/properties/bindings") |

### bindings

The bindings of the results.

`bindings`

*   is required

*   Type: `object[]` ([Details](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings-items.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings.md "undefined#/responses/select/definitions/Results/properties/bindings")

#### bindings Type

`object[]` ([Details](okp4-cognitarium-responses-selectresponse-definitions-results-properties-bindings-items.md))

## Definitions group Value

Reference this group by using

```json
{"$ref":"undefined#/responses/select/definitions/Value"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_byte_size/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_byte_size/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_byte_size
```

The maximum number of bytes the store can contains. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. If `None`, there is no limit on the number of bytes.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size-anyof-1.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_byte_size/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_byte_size/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_byte_size
```

The maximum number of bytes an insert data query can contains. If `None`, there is no limit on the number of bytes.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_insert\_data\_byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size-anyof-1.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_triple_count/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_triple_count/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_triple_count
```

The maximum number of triples an insert data query can contains (after parsing). If `None`, there is no limit on the number of triples.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_insert\_data\_triple\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count-anyof-1.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_query_limit/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_query_limit/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_query_limit
```

The maximum limit of a query, i.e. the maximum number of triples returned by a select query. If `None`, there is no limit on the number of triples returned.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_query\_limit Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit-anyof-1.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_query_variable_count/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_query_variable_count/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_query_variable_count
```

The maximum number of variables a query can select. If `None`, there is no limit on the number of variables.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_query\_variable\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count-anyof-1.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_triple_byte_size/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_triple_byte_size/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_triple_byte_size
```

The maximum number of bytes the store can contains for a single triple. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. The limit is used to prevent storing very large triples, especially literals. If `None`, there is no limit on the number of bytes.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_triple\_byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size-anyof-1.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_triple_count/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_triple_count/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreLimits/properties/max_triple_count
```

The maximum number of triples the store can contains. If `None`, there is no limit on the number of triples.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## max\_triple\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count-anyof-1.md "check type definition")
# StoreLimits Schema

```txt
undefined#/responses/store/definitions/StoreLimits
```

Contains limitations regarding store usages.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## StoreLimits Type

`object` ([StoreLimits](okp4-cognitarium-responses-storeresponse-definitions-storelimits.md))

# StoreLimits Properties

| Property                                                          | Type   | Required | Nullable       | Defined by                                                                                                                                                                                                                   |
| :---------------------------------------------------------------- | :----- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [max\_byte\_size](#max_byte_size)                                 | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_byte_size")                               |
| [max\_insert\_data\_byte\_size](#max_insert_data_byte_size)       | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_byte_size")       |
| [max\_insert\_data\_triple\_count](#max_insert_data_triple_count) | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_triple_count") |
| [max\_query\_limit](#max_query_limit)                             | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit.md "undefined#/responses/store/definitions/StoreLimits/properties/max_query_limit")                           |
| [max\_query\_variable\_count](#max_query_variable_count)          | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_query_variable_count")         |
| [max\_triple\_byte\_size](#max_triple_byte_size)                  | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_triple_byte_size")                 |
| [max\_triple\_count](#max_triple_count)                           | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_triple_count")                         |

## max\_byte\_size

The maximum number of bytes the store can contains. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. If `None`, there is no limit on the number of bytes.

`max_byte_size`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_byte_size")

### max\_byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size-anyof-1.md "check type definition")

## max\_insert\_data\_byte\_size

The maximum number of bytes an insert data query can contains. If `None`, there is no limit on the number of bytes.

`max_insert_data_byte_size`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_byte_size")

### max\_insert\_data\_byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size-anyof-1.md "check type definition")

## max\_insert\_data\_triple\_count

The maximum number of triples an insert data query can contains (after parsing). If `None`, there is no limit on the number of triples.

`max_insert_data_triple_count`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_triple_count")

### max\_insert\_data\_triple\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count-anyof-1.md "check type definition")

## max\_query\_limit

The maximum limit of a query, i.e. the maximum number of triples returned by a select query. If `None`, there is no limit on the number of triples returned.

`max_query_limit`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit.md "undefined#/responses/store/definitions/StoreLimits/properties/max_query_limit")

### max\_query\_limit Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit-anyof-1.md "check type definition")

## max\_query\_variable\_count

The maximum number of variables a query can select. If `None`, there is no limit on the number of variables.

`max_query_variable_count`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_query_variable_count")

### max\_query\_variable\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count-anyof-1.md "check type definition")

## max\_triple\_byte\_size

The maximum number of bytes the store can contains for a single triple. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. The limit is used to prevent storing very large triples, especially literals. If `None`, there is no limit on the number of bytes.

`max_triple_byte_size`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_triple_byte_size")

### max\_triple\_byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size-anyof-1.md "check type definition")

## max\_triple\_count

The maximum number of triples the store can contains. If `None`, there is no limit on the number of triples.

`max_triple_count`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_triple_count")

### max\_triple\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count-anyof-1.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreStat/properties/byte_size/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreStat/properties/byte_size
```

The total triple size in the store, in bytes.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size-allof-0.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreStat/properties/namespace_count/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreStat/properties/namespace_count
```

The total number of IRI namespace present in the store.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## namespace\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count-allof-0.md "check type definition")
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreStat/properties/triple_count/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/StoreStat/properties/triple_count
```

The total number of triple present in the store.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## triple\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count-allof-0.md "check type definition")
# StoreStat Schema

```txt
undefined#/responses/store/definitions/StoreStat
```

Contains usage information about the triple store.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## StoreStat Type

`object` ([StoreStat](okp4-cognitarium-responses-storeresponse-definitions-storestat.md))

# StoreStat Properties

| Property                             | Type   | Required | Nullable       | Defined by                                                                                                                                                                                     |
| :----------------------------------- | :----- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [byte\_size](#byte_size)             | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size.md "undefined#/responses/store/definitions/StoreStat/properties/byte_size")             |
| [namespace\_count](#namespace_count) | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count.md "undefined#/responses/store/definitions/StoreStat/properties/namespace_count") |
| [triple\_count](#triple_count)       | Merged | Required | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count.md "undefined#/responses/store/definitions/StoreStat/properties/triple_count")       |

## byte\_size

The total triple size in the store, in bytes.

`byte_size`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size.md "undefined#/responses/store/definitions/StoreStat/properties/byte_size")

### byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-byte_size-allof-0.md "check type definition")

## namespace\_count

The total number of IRI namespace present in the store.

`namespace_count`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count.md "undefined#/responses/store/definitions/StoreStat/properties/namespace_count")

### namespace\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-namespace_count-allof-0.md "check type definition")

## triple\_count

The total number of triple present in the store.

`triple_count`

*   is required

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count.md "undefined#/responses/store/definitions/StoreStat/properties/triple_count")

### triple\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storestat-properties-triple_count-allof-0.md "check type definition")
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions/Uint128
```

A thin wrapper around u128 that is using strings for JSON encoding/decoding, such that the full u128 range can be used for clients that convert JSON numbers to floats, like JavaScript and jq.

# Examples

Use `from` to create instances of this and `u128` to get the value out:

````# use cosmwasm_std::Uint128; let a = Uint128::from(123u128); assert_eq!(a.u128(), 123);

let b = Uint128::from(42u64); assert_eq!(b.u128(), 42);

let c = Uint128::from(70u32); assert_eq!(c.u128(), 70); ```
````

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## Uint128 Type

`string`
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/definitions
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## definitions Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/properties/limits/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/properties/limits
```

The store limits.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## limits Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-properties-limits.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-properties-limits-allof-0.md "check type definition")
# Untitled string in okp4-cognitarium Schema

```txt
undefined#/responses/store/properties/owner
```

The store owner.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## owner Type

`string`
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/properties/stat/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses/store/properties/stat
```

The store current usage.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## stat Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-properties-stat.md))

all of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-properties-stat-allof-0.md "check type definition")
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

| Property                                                          | Type   | Required | Nullable       | Defined by                                                                                                                                                                                                                   |
| :---------------------------------------------------------------- | :----- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [max\_byte\_size](#max_byte_size)                                 | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_byte_size")                               |
| [max\_insert\_data\_byte\_size](#max_insert_data_byte_size)       | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_byte_size")       |
| [max\_insert\_data\_triple\_count](#max_insert_data_triple_count) | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_triple_count") |
| [max\_query\_limit](#max_query_limit)                             | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit.md "undefined#/responses/store/definitions/StoreLimits/properties/max_query_limit")                           |
| [max\_query\_variable\_count](#max_query_variable_count)          | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_query_variable_count")         |
| [max\_triple\_byte\_size](#max_triple_byte_size)                  | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_triple_byte_size")                 |
| [max\_triple\_count](#max_triple_count)                           | Merged | Optional | cannot be null | [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_triple_count")                         |

### max\_byte\_size

The maximum number of bytes the store can contains. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. If `None`, there is no limit on the number of bytes.

`max_byte_size`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_byte_size")

#### max\_byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_byte_size-anyof-1.md "check type definition")

### max\_insert\_data\_byte\_size

The maximum number of bytes an insert data query can contains. If `None`, there is no limit on the number of bytes.

`max_insert_data_byte_size`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_byte_size")

#### max\_insert\_data\_byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_byte_size-anyof-1.md "check type definition")

### max\_insert\_data\_triple\_count

The maximum number of triples an insert data query can contains (after parsing). If `None`, there is no limit on the number of triples.

`max_insert_data_triple_count`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_insert_data_triple_count")

#### max\_insert\_data\_triple\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_insert_data_triple_count-anyof-1.md "check type definition")

### max\_query\_limit

The maximum limit of a query, i.e. the maximum number of triples returned by a select query. If `None`, there is no limit on the number of triples returned.

`max_query_limit`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit.md "undefined#/responses/store/definitions/StoreLimits/properties/max_query_limit")

#### max\_query\_limit Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_limit-anyof-1.md "check type definition")

### max\_query\_variable\_count

The maximum number of variables a query can select. If `None`, there is no limit on the number of variables.

`max_query_variable_count`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_query_variable_count")

#### max\_query\_variable\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_query_variable_count-anyof-1.md "check type definition")

### max\_triple\_byte\_size

The maximum number of bytes the store can contains for a single triple. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. The limit is used to prevent storing very large triples, especially literals. If `None`, there is no limit on the number of bytes.

`max_triple_byte_size`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size.md "undefined#/responses/store/definitions/StoreLimits/properties/max_triple_byte_size")

#### max\_triple\_byte\_size Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_byte_size-anyof-1.md "check type definition")

### max\_triple\_count

The maximum number of triples the store can contains. If `None`, there is no limit on the number of triples.

`max_triple_count`

*   is optional

*   Type: merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count.md))

*   cannot be null

*   defined in: [okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count.md "undefined#/responses/store/definitions/StoreLimits/properties/max_triple_count")

#### max\_triple\_count Type

merged type ([Details](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count.md))

any of

*   [Untitled undefined type in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count-anyof-0.md "check type definition")

*   [Untitled null in okp4-cognitarium](okp4-cognitarium-responses-storeresponse-definitions-storelimits-properties-max_triple_count-anyof-1.md "check type definition")

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
# Untitled undefined type in okp4-cognitarium Schema

```txt
undefined#/responses
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json\*](schema/okp4-cognitarium.json "open original schema") |

## responses Type

unknown
# okp4-cognitarium Schema

```txt
undefined
```

# Triplestore

## Overview

The `okp4-cognitarium` smart contract enables the storage of RDF graphs triples (i.e. `subject`-`predicate`-`object`) in any [Cosmos blockchains](https://cosmos.network/) using the [CosmWasm](https://cosmwasm.com/) framework.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                   |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-cognitarium.json](schema/okp4-cognitarium.json "open original schema") |

## okp4-cognitarium Type

unknown ([okp4-cognitarium](okp4-cognitarium.md))
# BreakStone Schema

```txt
undefined#/execute/oneOf/0
```

Break the stone making this contract unusable, by clearing all the related resources: - Unpin all the pinned objects on `okp4-objectarium` contracts, if any. - Forget the main program (i.e. or at least unpin it). Only the contract admin is authorized to break it, if any. If already broken, this is a no-op.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## 0 Type

`string` ([BreakStone](okp4-law-stone-executemsg-oneof-breakstone.md))

## 0 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value           | Explanation |
| :-------------- | :---------- |
| `"break_stone"` |             |
# ExecuteMsg Schema

```txt
undefined#/execute
```

Execute messages

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## execute Type

merged type ([ExecuteMsg](okp4-law-stone-executemsg.md))

one (and only one) of

*   [BreakStone](okp4-law-stone-executemsg-oneof-breakstone.md "check type definition")
# Untitled string in okp4-law-stone Schema

```txt
undefined#/instantiate/definitions/Binary
```

Binary is a wrapper around <code>Vec&lt;u8&gt;</code> to add base64 de/serialization with serde. It also adds some helper methods to help encode inline.

This is only needed as serde-json-{core,wasm} has a horrible encoding for <code>Vec&lt;u8&gt;</code>. See also <https://github.com/CosmWasm/cosmwasm/blob/main/docs/MESSAGE_TYPES.md>.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## Binary Type

`string`
# Untitled undefined type in okp4-law-stone Schema

```txt
undefined#/instantiate/definitions
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## definitions Type

unknown
# Untitled undefined type in okp4-law-stone Schema

```txt
undefined#/instantiate/properties/program/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-law-stone Schema

```txt
undefined#/instantiate/properties/program
```

The Prolog program carrying law rules and facts.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## program Type

merged type ([Details](okp4-law-stone-instantiatemsg-properties-program.md))

all of

*   [Untitled undefined type in okp4-law-stone](okp4-law-stone-instantiatemsg-properties-program-allof-0.md "check type definition")
# Untitled string in okp4-law-stone Schema

```txt
undefined#/instantiate/properties/storage_address
```

The `okp4-objectarium` contract address on which to store the law program.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## storage\_address Type

`string`
# InstantiateMsg Schema

```txt
undefined#/instantiate
```

Instantiate message

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## instantiate Type

`object` ([InstantiateMsg](okp4-law-stone-instantiatemsg.md))

# instantiate Properties

| Property                             | Type     | Required | Nullable       | Defined by                                                                                                                        |
| :----------------------------------- | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------- |
| [program](#program)                  | Merged   | Required | cannot be null | [okp4-law-stone](okp4-law-stone-instantiatemsg-properties-program.md "undefined#/instantiate/properties/program")                 |
| [storage\_address](#storage_address) | `string` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-instantiatemsg-properties-storage_address.md "undefined#/instantiate/properties/storage_address") |

## program

The Prolog program carrying law rules and facts.

`program`

*   is required

*   Type: merged type ([Details](okp4-law-stone-instantiatemsg-properties-program.md))

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-instantiatemsg-properties-program.md "undefined#/instantiate/properties/program")

### program Type

merged type ([Details](okp4-law-stone-instantiatemsg-properties-program.md))

all of

*   [Untitled undefined type in okp4-law-stone](okp4-law-stone-instantiatemsg-properties-program-allof-0.md "check type definition")

## storage\_address

The `okp4-objectarium` contract address on which to store the law program.

`storage_address`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-instantiatemsg-properties-storage_address.md "undefined#/instantiate/properties/storage_address")

### storage\_address Type

`string`

# InstantiateMsg Definitions

## Definitions group Binary

Reference this group by using

```json
{"$ref":"undefined#/instantiate/definitions/Binary"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
# Untitled string in okp4-law-stone Schema

```txt
undefined#/query/oneOf/0/properties/ask/properties/query
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## query Type

`string`
# Untitled object in okp4-law-stone Schema

```txt
undefined#/query/oneOf/0/properties/ask
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## ask Type

`object` ([Details](okp4-law-stone-querymsg-oneof-ask-properties-ask.md))

# ask Properties

| Property        | Type     | Required | Nullable       | Defined by                                                                                                                                        |
| :-------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| [query](#query) | `string` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-querymsg-oneof-ask-properties-ask-properties-query.md "undefined#/query/oneOf/0/properties/ask/properties/query") |

## query



`query`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-querymsg-oneof-ask-properties-ask-properties-query.md "undefined#/query/oneOf/0/properties/ask/properties/query")

### query Type

`string`
# Ask Schema

```txt
undefined#/query/oneOf/0
```

If not broken, ask the logic module the provided query with the law program loaded.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## 0 Type

`object` ([Ask](okp4-law-stone-querymsg-oneof-ask.md))

# 0 Properties

| Property    | Type     | Required | Nullable       | Defined by                                                                                                      |
| :---------- | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------- |
| [ask](#ask) | `object` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-querymsg-oneof-ask-properties-ask.md "undefined#/query/oneOf/0/properties/ask") |

## ask



`ask`

*   is required

*   Type: `object` ([Details](okp4-law-stone-querymsg-oneof-ask-properties-ask.md))

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-querymsg-oneof-ask-properties-ask.md "undefined#/query/oneOf/0/properties/ask")

### ask Type

`object` ([Details](okp4-law-stone-querymsg-oneof-ask-properties-ask.md))
# Program Schema

```txt
undefined#/query/oneOf/1
```

If not broken, returns the law program location information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## 1 Type

`string` ([Program](okp4-law-stone-querymsg-oneof-program.md))

## 1 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value       | Explanation |
| :---------- | :---------- |
| `"program"` |             |
# QueryMsg Schema

```txt
undefined#/query
```

Query messages

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## query Type

merged type ([QueryMsg](okp4-law-stone-querymsg.md))

one (and only one) of

*   [Ask](okp4-law-stone-querymsg-oneof-ask.md "check type definition")

*   [Program](okp4-law-stone-querymsg-oneof-program.md "check type definition")
# Untitled boolean in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Answer/properties/has_more
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## has\_more Type

`boolean`
# Untitled undefined type in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Answer/properties/results/items
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## items Type

unknown
# Untitled array in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Answer/properties/results
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## results Type

unknown\[]
# Untitled boolean in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Answer/properties/success
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## success Type

`boolean`
# Untitled string in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Answer/properties/variables/items
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## items Type

`string`
# Untitled array in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Answer/properties/variables
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## variables Type

`string[]`
# Untitled object in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Answer
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## Answer Type

`object` ([Details](okp4-law-stone-responses-askresponse-definitions-answer.md))

# Answer Properties

| Property                | Type      | Required | Nullable       | Defined by                                                                                                                                                           |
| :---------------------- | :-------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [has\_more](#has_more)  | `boolean` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-has_more.md "undefined#/responses/ask/definitions/Answer/properties/has_more")   |
| [results](#results)     | `array`   | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-results.md "undefined#/responses/ask/definitions/Answer/properties/results")     |
| [success](#success)     | `boolean` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-success.md "undefined#/responses/ask/definitions/Answer/properties/success")     |
| [variables](#variables) | `array`   | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-variables.md "undefined#/responses/ask/definitions/Answer/properties/variables") |

## has\_more



`has_more`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-has_more.md "undefined#/responses/ask/definitions/Answer/properties/has_more")

### has\_more Type

`boolean`

## results



`results`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-results.md "undefined#/responses/ask/definitions/Answer/properties/results")

### results Type

unknown\[]

## success



`success`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-success.md "undefined#/responses/ask/definitions/Answer/properties/success")

### success Type

`boolean`

## variables



`variables`

*   is required

*   Type: `string[]`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-variables.md "undefined#/responses/ask/definitions/Answer/properties/variables")

### variables Type

`string[]`
# Untitled undefined type in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Result/properties/substitutions/items
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## items Type

unknown
# Untitled array in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Result/properties/substitutions
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## substitutions Type

unknown\[]
# Untitled object in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Result
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## Result Type

`object` ([Details](okp4-law-stone-responses-askresponse-definitions-result.md))

# Result Properties

| Property                        | Type    | Required | Nullable       | Defined by                                                                                                                                                                   |
| :------------------------------ | :------ | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [substitutions](#substitutions) | `array` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-result-properties-substitutions.md "undefined#/responses/ask/definitions/Result/properties/substitutions") |

## substitutions



`substitutions`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-result-properties-substitutions.md "undefined#/responses/ask/definitions/Result/properties/substitutions")

### substitutions Type

unknown\[]
# Untitled undefined type in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Substitution/properties/term
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## term Type

unknown
# Untitled string in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Substitution/properties/variable
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## variable Type

`string`
# Untitled object in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Substitution
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## Substitution Type

`object` ([Details](okp4-law-stone-responses-askresponse-definitions-substitution.md))

# Substitution Properties

| Property              | Type          | Required | Nullable       | Defined by                                                                                                                                                                     |
| :-------------------- | :------------ | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [term](#term)         | Not specified | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-substitution-properties-term.md "undefined#/responses/ask/definitions/Substitution/properties/term")         |
| [variable](#variable) | `string`      | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-substitution-properties-variable.md "undefined#/responses/ask/definitions/Substitution/properties/variable") |

## term



`term`

*   is required

*   Type: unknown

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-substitution-properties-term.md "undefined#/responses/ask/definitions/Substitution/properties/term")

### term Type

unknown

## variable



`variable`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-substitution-properties-variable.md "undefined#/responses/ask/definitions/Substitution/properties/variable")

### variable Type

`string`
# Untitled undefined type in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Term/properties/arguments/items
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## items Type

unknown
# Untitled array in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Term/properties/arguments
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## arguments Type

unknown\[]
# Untitled string in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Term/properties/name
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## name Type

`string`
# Untitled object in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions/Term
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## Term Type

`object` ([Details](okp4-law-stone-responses-askresponse-definitions-term.md))

# Term Properties

| Property                | Type     | Required | Nullable       | Defined by                                                                                                                                                       |
| :---------------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [arguments](#arguments) | `array`  | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-term-properties-arguments.md "undefined#/responses/ask/definitions/Term/properties/arguments") |
| [name](#name)           | `string` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-term-properties-name.md "undefined#/responses/ask/definitions/Term/properties/name")           |

## arguments



`arguments`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-term-properties-arguments.md "undefined#/responses/ask/definitions/Term/properties/arguments")

### arguments Type

unknown\[]

## name



`name`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-term-properties-name.md "undefined#/responses/ask/definitions/Term/properties/name")

### name Type

`string`
# Untitled undefined type in okp4-law-stone Schema

```txt
undefined#/responses/ask/definitions
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## definitions Type

unknown
# Untitled undefined type in okp4-law-stone Schema

```txt
undefined#/responses/ask/properties/answer/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-law-stone Schema

```txt
undefined#/responses/ask/properties/answer/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-law-stone Schema

```txt
undefined#/responses/ask/properties/answer
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## answer Type

merged type ([Details](okp4-law-stone-responses-askresponse-properties-answer.md))

any of

*   [Untitled undefined type in okp4-law-stone](okp4-law-stone-responses-askresponse-properties-answer-anyof-0.md "check type definition")

*   [Untitled null in okp4-law-stone](okp4-law-stone-responses-askresponse-properties-answer-anyof-1.md "check type definition")
# Untitled integer in okp4-law-stone Schema

```txt
undefined#/responses/ask/properties/gas_used
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## gas\_used Type

`integer`

## gas\_used Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint64`
# Untitled integer in okp4-law-stone Schema

```txt
undefined#/responses/ask/properties/height
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## height Type

`integer`

## height Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint64`
# AskResponse Schema

```txt
undefined#/responses/ask
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## ask Type

`object` ([AskResponse](okp4-law-stone-responses-askresponse.md))

# ask Properties

| Property               | Type      | Required | Nullable       | Defined by                                                                                                                   |
| :--------------------- | :-------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| [answer](#answer)      | Merged    | Optional | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-properties-answer.md "undefined#/responses/ask/properties/answer")     |
| [gas\_used](#gas_used) | `integer` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-properties-gas_used.md "undefined#/responses/ask/properties/gas_used") |
| [height](#height)      | `integer` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-properties-height.md "undefined#/responses/ask/properties/height")     |

## answer



`answer`

*   is optional

*   Type: merged type ([Details](okp4-law-stone-responses-askresponse-properties-answer.md))

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-properties-answer.md "undefined#/responses/ask/properties/answer")

### answer Type

merged type ([Details](okp4-law-stone-responses-askresponse-properties-answer.md))

any of

*   [Untitled undefined type in okp4-law-stone](okp4-law-stone-responses-askresponse-properties-answer-anyof-0.md "check type definition")

*   [Untitled null in okp4-law-stone](okp4-law-stone-responses-askresponse-properties-answer-anyof-1.md "check type definition")

## gas\_used



`gas_used`

*   is required

*   Type: `integer`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-properties-gas_used.md "undefined#/responses/ask/properties/gas_used")

### gas\_used Type

`integer`

### gas\_used Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint64`

## height



`height`

*   is required

*   Type: `integer`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-properties-height.md "undefined#/responses/ask/properties/height")

### height Type

`integer`

### height Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint64`

# AskResponse Definitions

## Definitions group Answer

Reference this group by using

```json
{"$ref":"undefined#/responses/ask/definitions/Answer"}
```

| Property                | Type      | Required | Nullable       | Defined by                                                                                                                                                           |
| :---------------------- | :-------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [has\_more](#has_more)  | `boolean` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-has_more.md "undefined#/responses/ask/definitions/Answer/properties/has_more")   |
| [results](#results)     | `array`   | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-results.md "undefined#/responses/ask/definitions/Answer/properties/results")     |
| [success](#success)     | `boolean` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-success.md "undefined#/responses/ask/definitions/Answer/properties/success")     |
| [variables](#variables) | `array`   | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-variables.md "undefined#/responses/ask/definitions/Answer/properties/variables") |

### has\_more



`has_more`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-has_more.md "undefined#/responses/ask/definitions/Answer/properties/has_more")

#### has\_more Type

`boolean`

### results



`results`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-results.md "undefined#/responses/ask/definitions/Answer/properties/results")

#### results Type

unknown\[]

### success



`success`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-success.md "undefined#/responses/ask/definitions/Answer/properties/success")

#### success Type

`boolean`

### variables



`variables`

*   is required

*   Type: `string[]`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-answer-properties-variables.md "undefined#/responses/ask/definitions/Answer/properties/variables")

#### variables Type

`string[]`

## Definitions group Result

Reference this group by using

```json
{"$ref":"undefined#/responses/ask/definitions/Result"}
```

| Property                        | Type    | Required | Nullable       | Defined by                                                                                                                                                                   |
| :------------------------------ | :------ | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [substitutions](#substitutions) | `array` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-result-properties-substitutions.md "undefined#/responses/ask/definitions/Result/properties/substitutions") |

### substitutions



`substitutions`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-result-properties-substitutions.md "undefined#/responses/ask/definitions/Result/properties/substitutions")

#### substitutions Type

unknown\[]

## Definitions group Substitution

Reference this group by using

```json
{"$ref":"undefined#/responses/ask/definitions/Substitution"}
```

| Property              | Type          | Required | Nullable       | Defined by                                                                                                                                                                     |
| :-------------------- | :------------ | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [term](#term)         | Not specified | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-substitution-properties-term.md "undefined#/responses/ask/definitions/Substitution/properties/term")         |
| [variable](#variable) | `string`      | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-substitution-properties-variable.md "undefined#/responses/ask/definitions/Substitution/properties/variable") |

### term



`term`

*   is required

*   Type: unknown

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-substitution-properties-term.md "undefined#/responses/ask/definitions/Substitution/properties/term")

#### term Type

unknown

### variable



`variable`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-substitution-properties-variable.md "undefined#/responses/ask/definitions/Substitution/properties/variable")

#### variable Type

`string`

## Definitions group Term

Reference this group by using

```json
{"$ref":"undefined#/responses/ask/definitions/Term"}
```

| Property                | Type     | Required | Nullable       | Defined by                                                                                                                                                       |
| :---------------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [arguments](#arguments) | `array`  | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-term-properties-arguments.md "undefined#/responses/ask/definitions/Term/properties/arguments") |
| [name](#name)           | `string` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-term-properties-name.md "undefined#/responses/ask/definitions/Term/properties/name")           |

### arguments



`arguments`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-term-properties-arguments.md "undefined#/responses/ask/definitions/Term/properties/arguments")

#### arguments Type

unknown\[]

### name



`name`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-askresponse-definitions-term-properties-name.md "undefined#/responses/ask/definitions/Term/properties/name")

#### name Type

`string`
# Untitled string in okp4-law-stone Schema

```txt
undefined#/responses/program/properties/object_id
```

The program object id in the `okp4-objectarium` contract.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## object\_id Type

`string`
# Untitled string in okp4-law-stone Schema

```txt
undefined#/responses/program/properties/storage_address
```

The `okp4-objectarium` contract address on which the law program is stored.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## storage\_address Type

`string`
# ProgramResponse Schema

```txt
undefined#/responses/program
```

ProgramResponse carry elements to locate the program in a `okp4-objectarium` contract.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## program Type

`object` ([ProgramResponse](okp4-law-stone-responses-programresponse.md))

# program Properties

| Property                             | Type     | Required | Nullable       | Defined by                                                                                                                                         |
| :----------------------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| [object\_id](#object_id)             | `string` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-programresponse-properties-object_id.md "undefined#/responses/program/properties/object_id")             |
| [storage\_address](#storage_address) | `string` | Required | cannot be null | [okp4-law-stone](okp4-law-stone-responses-programresponse-properties-storage_address.md "undefined#/responses/program/properties/storage_address") |

## object\_id

The program object id in the `okp4-objectarium` contract.

`object_id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-programresponse-properties-object_id.md "undefined#/responses/program/properties/object_id")

### object\_id Type

`string`

## storage\_address

The `okp4-objectarium` contract address on which the law program is stored.

`storage_address`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-law-stone](okp4-law-stone-responses-programresponse-properties-storage_address.md "undefined#/responses/program/properties/storage_address")

### storage\_address Type

`string`
# Untitled undefined type in okp4-law-stone Schema

```txt
undefined#/responses
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json\*](schema/okp4-law-stone.json "open original schema") |

## responses Type

unknown
# okp4-law-stone Schema

```txt
undefined
```

# Law Stone

## Overview

The `okp4-law-stone` smart contract aims to provide GaaS (i.e. Governance as a Service) in any [Cosmos blockchains](https://cosmos.network/) using the [CosmWasm](https://cosmwasm.com/) framework and the [Logic](https://docs.okp4.network/modules/next/logic) OKP4 module.

This contract is built around a Prolog program describing the law by rules and facts. The law stone is immutable, this means it can only been questioned, there is no update mechanisms.

The `okp4-law-stone` responsibility is to guarantee the availability of its rules in order to question them, but not to ensure the rules application.

To ensure reliability over time, the associated Prolog program is stored and pinned in a `okp4-objectarium` contract. Moreover, all the eventual loaded files must be stored in a `okp4-objectarium` contract as well, allowing the contract to pin them.

To be able to free the underlying resources (i.e. objects in `okp4-objectarium`) if not used anymore, the contract admin can break the stone.

➡️ Checkout the [examples](https://github.com/okp4/contracts/tree/main/contracts/okp4-law-stone/examples/) for usage information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                               |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-law-stone.json](schema/okp4-law-stone.json "open original schema") |

## okp4-law-stone Type

unknown ([okp4-law-stone](okp4-law-stone.md))
# Untitled string in okp4-objectarium Schema

```txt
undefined#/execute/definitions/Binary
```

Binary is a wrapper around <code>Vec&lt;u8&gt;</code> to add base64 de/serialization with serde. It also adds some helper methods to help encode inline.

This is only needed as serde-json-{core,wasm} has a horrible encoding for <code>Vec&lt;u8&gt;</code>. See also <https://github.com/CosmWasm/cosmwasm/blob/main/docs/MESSAGE_TYPES.md>.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## Binary Type

`string`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/execute/definitions
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## definitions Type

unknown
# Untitled string in okp4-objectarium Schema

```txt
undefined#/execute/oneOf/1/properties/forget_object/properties/id
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## id Type

`string`
# Untitled object in okp4-objectarium Schema

```txt
undefined#/execute/oneOf/1/properties/forget_object
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## forget\_object Type

`object` ([Details](okp4-objectarium-executemsg-oneof-forgetobject-properties-forget_object.md))

# forget\_object Properties

| Property  | Type     | Required | Nullable       | Defined by                                                                                                                                                                       |
| :-------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [id](#id) | `string` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-executemsg-oneof-forgetobject-properties-forget_object-properties-id.md "undefined#/execute/oneOf/1/properties/forget_object/properties/id") |

## id



`id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-executemsg-oneof-forgetobject-properties-forget_object-properties-id.md "undefined#/execute/oneOf/1/properties/forget_object/properties/id")

### id Type

`string`
# ForgetObject Schema

```txt
undefined#/execute/oneOf/1
```

ForgetObject first unpin the object from the bucket for the considered sender, then remove it from the storage if it is not pinned anymore. If the object is pinned for other senders, it is not removed from the storage and an error is returned. If the object is not pinned for the sender, this is a no-op.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`object` ([ForgetObject](okp4-objectarium-executemsg-oneof-forgetobject.md))

# 1 Properties

| Property                         | Type     | Required | Nullable       | Defined by                                                                                                                                           |
| :------------------------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| [forget\_object](#forget_object) | `object` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-executemsg-oneof-forgetobject-properties-forget_object.md "undefined#/execute/oneOf/1/properties/forget_object") |

## forget\_object



`forget_object`

*   is required

*   Type: `object` ([Details](okp4-objectarium-executemsg-oneof-forgetobject-properties-forget_object.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-executemsg-oneof-forgetobject-properties-forget_object.md "undefined#/execute/oneOf/1/properties/forget_object")

### forget\_object Type

`object` ([Details](okp4-objectarium-executemsg-oneof-forgetobject-properties-forget_object.md))
# Untitled string in okp4-objectarium Schema

```txt
undefined#/execute/oneOf/2/properties/pin_object/properties/id
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## id Type

`string`
# Untitled object in okp4-objectarium Schema

```txt
undefined#/execute/oneOf/2/properties/pin_object
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## pin\_object Type

`object` ([Details](okp4-objectarium-executemsg-oneof-pinobject-properties-pin_object.md))

# pin\_object Properties

| Property  | Type     | Required | Nullable       | Defined by                                                                                                                                                              |
| :-------- | :------- | :------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [id](#id) | `string` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-executemsg-oneof-pinobject-properties-pin_object-properties-id.md "undefined#/execute/oneOf/2/properties/pin_object/properties/id") |

## id



`id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-executemsg-oneof-pinobject-properties-pin_object-properties-id.md "undefined#/execute/oneOf/2/properties/pin_object/properties/id")

### id Type

`string`
# PinObject Schema

```txt
undefined#/execute/oneOf/2
```

PinObject pins the object in the bucket for the considered sender. If the object is already pinned for the sender, this is a no-op. While an object is pinned, it cannot be removed from the storage.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 2 Type

`object` ([PinObject](okp4-objectarium-executemsg-oneof-pinobject.md))

# 2 Properties

| Property                   | Type     | Required | Nullable       | Defined by                                                                                                                                  |
| :------------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| [pin\_object](#pin_object) | `object` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-executemsg-oneof-pinobject-properties-pin_object.md "undefined#/execute/oneOf/2/properties/pin_object") |

## pin\_object



`pin_object`

*   is required

*   Type: `object` ([Details](okp4-objectarium-executemsg-oneof-pinobject-properties-pin_object.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-executemsg-oneof-pinobject-properties-pin_object.md "undefined#/execute/oneOf/2/properties/pin_object")

### pin\_object Type

`object` ([Details](okp4-objectarium-executemsg-oneof-pinobject-properties-pin_object.md))
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/execute/oneOf/0/properties/store_object/properties/data
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## data Type

unknown
# Untitled boolean in okp4-objectarium Schema

```txt
undefined#/execute/oneOf/0/properties/store_object/properties/pin
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## pin Type

`boolean`
# Untitled object in okp4-objectarium Schema

```txt
undefined#/execute/oneOf/0/properties/store_object
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## store\_object Type

`object` ([Details](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object.md))

# store\_object Properties

| Property      | Type          | Required | Nullable       | Defined by                                                                                                                                                                        |
| :------------ | :------------ | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [data](#data) | Not specified | Required | cannot be null | [okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-data.md "undefined#/execute/oneOf/0/properties/store_object/properties/data") |
| [pin](#pin)   | `boolean`     | Required | cannot be null | [okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-pin.md "undefined#/execute/oneOf/0/properties/store_object/properties/pin")   |

## data



`data`

*   is required

*   Type: unknown

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-data.md "undefined#/execute/oneOf/0/properties/store_object/properties/data")

### data Type

unknown

## pin



`pin`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object-properties-pin.md "undefined#/execute/oneOf/0/properties/store_object/properties/pin")

### pin Type

`boolean`
# StoreObject Schema

```txt
undefined#/execute/oneOf/0
```

StoreObject store an object to the bucket and make the sender the owner of the object. The object is referenced by the hash of its content and this value is returned. If the object is already stored, an error is returned. If pin is true, the object is pinned for the sender.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

`object` ([StoreObject](okp4-objectarium-executemsg-oneof-storeobject.md))

# 0 Properties

| Property                       | Type     | Required | Nullable       | Defined by                                                                                                                                        |
| :----------------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| [store\_object](#store_object) | `object` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object.md "undefined#/execute/oneOf/0/properties/store_object") |

## store\_object



`store_object`

*   is required

*   Type: `object` ([Details](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object.md "undefined#/execute/oneOf/0/properties/store_object")

### store\_object Type

`object` ([Details](okp4-objectarium-executemsg-oneof-storeobject-properties-store_object.md))
# Untitled string in okp4-objectarium Schema

```txt
undefined#/execute/oneOf/3/properties/unpin_object/properties/id
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## id Type

`string`
# Untitled object in okp4-objectarium Schema

```txt
undefined#/execute/oneOf/3/properties/unpin_object
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## unpin\_object Type

`object` ([Details](okp4-objectarium-executemsg-oneof-unpinobject-properties-unpin_object.md))

# unpin\_object Properties

| Property  | Type     | Required | Nullable       | Defined by                                                                                                                                                                    |
| :-------- | :------- | :------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [id](#id) | `string` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-executemsg-oneof-unpinobject-properties-unpin_object-properties-id.md "undefined#/execute/oneOf/3/properties/unpin_object/properties/id") |

## id



`id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-executemsg-oneof-unpinobject-properties-unpin_object-properties-id.md "undefined#/execute/oneOf/3/properties/unpin_object/properties/id")

### id Type

`string`
# UnpinObject Schema

```txt
undefined#/execute/oneOf/3
```

UnpinObject unpins the object in the bucket for the considered sender. If the object is not pinned for the sender, this is a no-op. The object can be removed from the storage if it is not pinned anymore.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 3 Type

`object` ([UnpinObject](okp4-objectarium-executemsg-oneof-unpinobject.md))

# 3 Properties

| Property                       | Type     | Required | Nullable       | Defined by                                                                                                                                        |
| :----------------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| [unpin\_object](#unpin_object) | `object` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-executemsg-oneof-unpinobject-properties-unpin_object.md "undefined#/execute/oneOf/3/properties/unpin_object") |

## unpin\_object



`unpin_object`

*   is required

*   Type: `object` ([Details](okp4-objectarium-executemsg-oneof-unpinobject-properties-unpin_object.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-executemsg-oneof-unpinobject-properties-unpin_object.md "undefined#/execute/oneOf/3/properties/unpin_object")

### unpin\_object Type

`object` ([Details](okp4-objectarium-executemsg-oneof-unpinobject-properties-unpin_object.md))
# ExecuteMsg Schema

```txt
undefined#/execute
```

Execute messages

| Abstract               | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :--------------------- | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Cannot be instantiated | Yes        | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## execute Type

merged type ([ExecuteMsg](okp4-objectarium-executemsg.md))

one (and only one) of

*   [StoreObject](okp4-objectarium-executemsg-oneof-storeobject.md "check type definition")

*   [ForgetObject](okp4-objectarium-executemsg-oneof-forgetobject.md "check type definition")

*   [PinObject](okp4-objectarium-executemsg-oneof-pinobject.md "check type definition")

*   [UnpinObject](okp4-objectarium-executemsg-oneof-unpinobject.md "check type definition")

# ExecuteMsg Definitions

## Definitions group Binary

Reference this group by using

```json
{"$ref":"undefined#/execute/definitions/Binary"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketConfig/properties/hash_algorithm/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketConfig/properties/hash_algorithm/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketConfig/properties/hash_algorithm
```

The algorithm used to hash the content of the objects to generate the id of the objects. The algorithm is optional and if not set, the default algorithm is used.

The default algorithm is Sha256 .

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## hash\_algorithm Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm-anyof-1.md "check type definition")
# Untitled object in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketConfig
```

BucketConfig is the type of the configuration of a bucket.

The configuration is set at the instantiation of the bucket, and is immutable and cannot be changed. The configuration is optional and if not set, the default configuration is used.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## BucketConfig Type

`object` ([Details](okp4-objectarium-instantiatemsg-definitions-bucketconfig.md))

# BucketConfig Properties

| Property                           | Type   | Required | Nullable       | Defined by                                                                                                                                                                            |
| :--------------------------------- | :----- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [hash\_algorithm](#hash_algorithm) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm.md "undefined#/instantiate/definitions/BucketConfig/properties/hash_algorithm") |

## hash\_algorithm

The algorithm used to hash the content of the objects to generate the id of the objects. The algorithm is optional and if not set, the default algorithm is used.

The default algorithm is Sha256 .

`hash_algorithm`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm.md "undefined#/instantiate/definitions/BucketConfig/properties/hash_algorithm")

### hash\_algorithm Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm-anyof-1.md "check type definition")
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketLimits/properties/max_object_pins/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketLimits/properties/max_object_pins/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketLimits/properties/max_object_pins
```

The maximum number of pins in the bucket for an object.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## max\_object\_pins Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_pins.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_pins-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_pins-anyof-1.md "check type definition")
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketLimits/properties/max_object_size/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketLimits/properties/max_object_size/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketLimits/properties/max_object_size
```

The maximum size of the objects in the bucket.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## max\_object\_size Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_size-anyof-1.md "check type definition")
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketLimits/properties/max_objects/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketLimits/properties/max_objects/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketLimits/properties/max_objects
```

The maximum number of objects in the bucket.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## max\_objects Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_objects.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_objects-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_objects-anyof-1.md "check type definition")
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketLimits/properties/max_total_size/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketLimits/properties/max_total_size/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketLimits/properties/max_total_size
```

The maximum total size of the objects in the bucket.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## max\_total\_size Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_total_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_total_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_total_size-anyof-1.md "check type definition")
# Untitled object in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/BucketLimits
```

BucketLimits is the type of the limits of a bucket.

The limits are optional and if not set, there is no limit.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## BucketLimits Type

`object` ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits.md))

# BucketLimits Properties

| Property                              | Type   | Required | Nullable       | Defined by                                                                                                                                                                              |
| :------------------------------------ | :----- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [max\_object\_pins](#max_object_pins) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_pins.md "undefined#/instantiate/definitions/BucketLimits/properties/max_object_pins") |
| [max\_object\_size](#max_object_size) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_size.md "undefined#/instantiate/definitions/BucketLimits/properties/max_object_size") |
| [max\_objects](#max_objects)          | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_objects.md "undefined#/instantiate/definitions/BucketLimits/properties/max_objects")         |
| [max\_total\_size](#max_total_size)   | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_total_size.md "undefined#/instantiate/definitions/BucketLimits/properties/max_total_size")   |

## max\_object\_pins

The maximum number of pins in the bucket for an object.

`max_object_pins`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_pins.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_pins.md "undefined#/instantiate/definitions/BucketLimits/properties/max_object_pins")

### max\_object\_pins Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_pins.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_pins-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_pins-anyof-1.md "check type definition")

## max\_object\_size

The maximum size of the objects in the bucket.

`max_object_size`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_size.md "undefined#/instantiate/definitions/BucketLimits/properties/max_object_size")

### max\_object\_size Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_size-anyof-1.md "check type definition")

## max\_objects

The maximum number of objects in the bucket.

`max_objects`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_objects.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_objects.md "undefined#/instantiate/definitions/BucketLimits/properties/max_objects")

### max\_objects Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_objects.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_objects-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_objects-anyof-1.md "check type definition")

## max\_total\_size

The maximum total size of the objects in the bucket.

`max_total_size`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_total_size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_total_size.md "undefined#/instantiate/definitions/BucketLimits/properties/max_total_size")

### max\_total\_size Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_total_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_total_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_total_size-anyof-1.md "check type definition")
# MD5 Schema

```txt
undefined#/instantiate/definitions/HashAlgorithm/oneOf/0
```

Represents the MD5 algorithm. MD5 is a widely used cryptographic hash function that produces a 128-bit hash value. The computational cost of MD5 is relatively low compared to other hash functions, but its short hash length makes it easier to find hash collisions. It is now considered insecure for cryptographic purposes, but can still used in non-security contexts.

MD5 hashes are stored on-chain as 32 hexadecimal characters.

See [the MD5 Wikipedia page](https://en.wikipedia.org/wiki/MD5) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

`string` ([MD5](okp4-objectarium-instantiatemsg-definitions-hashalgorithm-oneof-md5.md))

## 0 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value    | Explanation |
| :------- | :---------- |
| `"m_d5"` |             |
# SHA1 Schema

```txt
undefined#/instantiate/definitions/HashAlgorithm/oneOf/1
```

Represents the SHA-224 algorithm. SHA-224 is a variant of the SHA-2 family of hash functions that produces a 224-bit hash value. It is similar to SHA-256, but with a shorter output size. The computational cost of SHA-224 is moderate, and its relatively short hash length makes it easier to store and transmit.

SHA-224 hashes are stored on-chain as 56 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`string` ([SHA1](okp4-objectarium-instantiatemsg-definitions-hashalgorithm-oneof-sha1.md))

## 1 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value      | Explanation |
| :--------- | :---------- |
| `"sha224"` |             |
# SHA256 Schema

```txt
undefined#/instantiate/definitions/HashAlgorithm/oneOf/2
```

Represents the SHA-256 algorithm. SHA-256 is a member of the SHA-2 family of hash functions that produces a 256-bit hash value. It is widely used in cryptography and other security-related applications. The computational cost of SHA-256 is moderate, and its hash length strikes a good balance between security and convenience.

SHA-256 hashes are stored on-chain as 64 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 2 Type

`string` ([SHA256](okp4-objectarium-instantiatemsg-definitions-hashalgorithm-oneof-sha256.md))

## 2 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value      | Explanation |
| :--------- | :---------- |
| `"sha256"` |             |
# SHA384 Schema

```txt
undefined#/instantiate/definitions/HashAlgorithm/oneOf/3
```

Represents the SHA-384 algorithm. SHA-384 is a variant of the SHA-2 family of hash functions that produces a 384-bit hash value. It is similar to SHA-512, but with a shorter output size. The computational cost of SHA-384 is relatively high, but its longer hash length provides better security against hash collisions.

SHA-384 hashes are stored on-chain as 96 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 3 Type

`string` ([SHA384](okp4-objectarium-instantiatemsg-definitions-hashalgorithm-oneof-sha384.md))

## 3 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value      | Explanation |
| :--------- | :---------- |
| `"sha384"` |             |
# SHA512 Schema

```txt
undefined#/instantiate/definitions/HashAlgorithm/oneOf/4
```

Represents the SHA-512 algorithm. SHA-512 is a member of the SHA-2 family of hash functions that produces a 512-bit hash value. It is widely used in cryptography and other security-related applications. The computational cost of SHA-512 is relatively high, but its longer hash length provides better security against hash collisions.

SHA-512 hashes are stored on-chain as 128 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 4 Type

`string` ([SHA512](okp4-objectarium-instantiatemsg-definitions-hashalgorithm-oneof-sha512.md))

## 4 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value      | Explanation |
| :--------- | :---------- |
| `"sha512"` |             |
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/HashAlgorithm
```

HashAlgorithm is an enumeration that defines the different hash algorithms supported for hashing the content of objects.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## HashAlgorithm Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-hashalgorithm.md))

one (and only one) of

*   [MD5](okp4-objectarium-instantiatemsg-definitions-hashalgorithm-oneof-md5.md "check type definition")

*   [SHA1](okp4-objectarium-instantiatemsg-definitions-hashalgorithm-oneof-sha1.md "check type definition")

*   [SHA256](okp4-objectarium-instantiatemsg-definitions-hashalgorithm-oneof-sha256.md "check type definition")

*   [SHA384](okp4-objectarium-instantiatemsg-definitions-hashalgorithm-oneof-sha384.md "check type definition")

*   [SHA512](okp4-objectarium-instantiatemsg-definitions-hashalgorithm-oneof-sha512.md "check type definition")
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/PaginationConfig/properties/default_page_size
```

The default number of elements in a page.

Shall be less or equal than `max_page_size`. Default to '10' if not set.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## default\_page\_size Type

`integer`

## default\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/PaginationConfig/properties/max_page_size
```

The maximum elements a page can contains.

Shall be less than `u32::MAX - 1`. Default to '30' if not set.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## max\_page\_size Type

`integer`

## max\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
# Untitled object in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/PaginationConfig
```

PaginationConfig is the type carrying configuration for paginated queries.

The fields are optional and if not set, there is a default configuration.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## PaginationConfig Type

`object` ([Details](okp4-objectarium-instantiatemsg-definitions-paginationconfig.md))

# PaginationConfig Properties

| Property                                  | Type      | Required | Nullable    | Defined by                                                                                                                                                                                          |
| :---------------------------------------- | :-------- | :------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [default\_page\_size](#default_page_size) | `integer` | Optional | can be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-paginationconfig-properties-default_page_size.md "undefined#/instantiate/definitions/PaginationConfig/properties/default_page_size") |
| [max\_page\_size](#max_page_size)         | `integer` | Optional | can be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-paginationconfig-properties-max_page_size.md "undefined#/instantiate/definitions/PaginationConfig/properties/max_page_size")         |

## default\_page\_size

The default number of elements in a page.

Shall be less or equal than `max_page_size`. Default to '10' if not set.

`default_page_size`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-paginationconfig-properties-default_page_size.md "undefined#/instantiate/definitions/PaginationConfig/properties/default_page_size")

### default\_page\_size Type

`integer`

### default\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

## max\_page\_size

The maximum elements a page can contains.

Shall be less than `u32::MAX - 1`. Default to '30' if not set.

`max_page_size`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-paginationconfig-properties-max_page_size.md "undefined#/instantiate/definitions/PaginationConfig/properties/max_page_size")

### max\_page\_size Type

`integer`

### max\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
# Untitled string in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions/Uint128
```

A thin wrapper around u128 that is using strings for JSON encoding/decoding, such that the full u128 range can be used for clients that convert JSON numbers to floats, like JavaScript and jq.

# Examples

Use `from` to create instances of this and `u128` to get the value out:

````# use cosmwasm_std::Uint128; let a = Uint128::from(123u128); assert_eq!(a.u128(), 123);

let b = Uint128::from(42u64); assert_eq!(b.u128(), 42);

let c = Uint128::from(70u32); assert_eq!(c.u128(), 70); ```
````

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## Uint128 Type

`string`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/definitions
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## definitions Type

unknown
# Untitled string in okp4-objectarium Schema

```txt
undefined#/instantiate/properties/bucket
```

The name of the bucket. The name could not be empty or contains whitespaces. If name contains whitespace, they will be removed.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## bucket Type

`string`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/properties/config/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/properties/config
```

The configuration of the bucket.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## config Type

merged type ([Details](okp4-objectarium-instantiatemsg-properties-config.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-properties-config-allof-0.md "check type definition")
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/properties/limits/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/properties/limits
```

The limits of the bucket.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## limits Type

merged type ([Details](okp4-objectarium-instantiatemsg-properties-limits.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-properties-limits-allof-0.md "check type definition")
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/properties/pagination/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/instantiate/properties/pagination
```

The configuration for paginated query.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## pagination Type

merged type ([Details](okp4-objectarium-instantiatemsg-properties-pagination.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-properties-pagination-allof-0.md "check type definition")
# InstantiateMsg Schema

```txt
undefined#/instantiate
```

Instantiate messages

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## instantiate Type

`object` ([InstantiateMsg](okp4-objectarium-instantiatemsg.md))

# instantiate Properties

| Property                  | Type     | Required | Nullable       | Defined by                                                                                                                  |
| :------------------------ | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------- |
| [bucket](#bucket)         | `string` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-properties-bucket.md "undefined#/instantiate/properties/bucket")         |
| [config](#config)         | Merged   | Required | cannot be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-properties-config.md "undefined#/instantiate/properties/config")         |
| [limits](#limits)         | Merged   | Required | cannot be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-properties-limits.md "undefined#/instantiate/properties/limits")         |
| [pagination](#pagination) | Merged   | Required | cannot be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-properties-pagination.md "undefined#/instantiate/properties/pagination") |

## bucket

The name of the bucket. The name could not be empty or contains whitespaces. If name contains whitespace, they will be removed.

`bucket`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-properties-bucket.md "undefined#/instantiate/properties/bucket")

### bucket Type

`string`

## config

The configuration of the bucket.

`config`

*   is required

*   Type: merged type ([Details](okp4-objectarium-instantiatemsg-properties-config.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-properties-config.md "undefined#/instantiate/properties/config")

### config Type

merged type ([Details](okp4-objectarium-instantiatemsg-properties-config.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-properties-config-allof-0.md "check type definition")

## limits

The limits of the bucket.

`limits`

*   is required

*   Type: merged type ([Details](okp4-objectarium-instantiatemsg-properties-limits.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-properties-limits.md "undefined#/instantiate/properties/limits")

### limits Type

merged type ([Details](okp4-objectarium-instantiatemsg-properties-limits.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-properties-limits-allof-0.md "check type definition")

## pagination

The configuration for paginated query.

`pagination`

*   is required

*   Type: merged type ([Details](okp4-objectarium-instantiatemsg-properties-pagination.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-properties-pagination.md "undefined#/instantiate/properties/pagination")

### pagination Type

merged type ([Details](okp4-objectarium-instantiatemsg-properties-pagination.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-properties-pagination-allof-0.md "check type definition")

# InstantiateMsg Definitions

## Definitions group BucketConfig

Reference this group by using

```json
{"$ref":"undefined#/instantiate/definitions/BucketConfig"}
```

| Property                           | Type   | Required | Nullable       | Defined by                                                                                                                                                                            |
| :--------------------------------- | :----- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [hash\_algorithm](#hash_algorithm) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm.md "undefined#/instantiate/definitions/BucketConfig/properties/hash_algorithm") |

### hash\_algorithm

The algorithm used to hash the content of the objects to generate the id of the objects. The algorithm is optional and if not set, the default algorithm is used.

The default algorithm is Sha256 .

`hash_algorithm`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm.md "undefined#/instantiate/definitions/BucketConfig/properties/hash_algorithm")

#### hash\_algorithm Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketconfig-properties-hash_algorithm-anyof-1.md "check type definition")

## Definitions group BucketLimits

Reference this group by using

```json
{"$ref":"undefined#/instantiate/definitions/BucketLimits"}
```

| Property                              | Type   | Required | Nullable       | Defined by                                                                                                                                                                              |
| :------------------------------------ | :----- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [max\_object\_pins](#max_object_pins) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_pins.md "undefined#/instantiate/definitions/BucketLimits/properties/max_object_pins") |
| [max\_object\_size](#max_object_size) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_size.md "undefined#/instantiate/definitions/BucketLimits/properties/max_object_size") |
| [max\_objects](#max_objects)          | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_objects.md "undefined#/instantiate/definitions/BucketLimits/properties/max_objects")         |
| [max\_total\_size](#max_total_size)   | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_total_size.md "undefined#/instantiate/definitions/BucketLimits/properties/max_total_size")   |

### max\_object\_pins

The maximum number of pins in the bucket for an object.

`max_object_pins`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_pins.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_pins.md "undefined#/instantiate/definitions/BucketLimits/properties/max_object_pins")

#### max\_object\_pins Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_pins.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_pins-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_pins-anyof-1.md "check type definition")

### max\_object\_size

The maximum size of the objects in the bucket.

`max_object_size`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_size.md "undefined#/instantiate/definitions/BucketLimits/properties/max_object_size")

#### max\_object\_size Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_object_size-anyof-1.md "check type definition")

### max\_objects

The maximum number of objects in the bucket.

`max_objects`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_objects.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_objects.md "undefined#/instantiate/definitions/BucketLimits/properties/max_objects")

#### max\_objects Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_objects.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_objects-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_objects-anyof-1.md "check type definition")

### max\_total\_size

The maximum total size of the objects in the bucket.

`max_total_size`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_total_size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_total_size.md "undefined#/instantiate/definitions/BucketLimits/properties/max_total_size")

#### max\_total\_size Type

merged type ([Details](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_total_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_total_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-bucketlimits-properties-max_total_size-anyof-1.md "check type definition")

## Definitions group HashAlgorithm

Reference this group by using

```json
{"$ref":"undefined#/instantiate/definitions/HashAlgorithm"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group PaginationConfig

Reference this group by using

```json
{"$ref":"undefined#/instantiate/definitions/PaginationConfig"}
```

| Property                                  | Type      | Required | Nullable    | Defined by                                                                                                                                                                                          |
| :---------------------------------------- | :-------- | :------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [default\_page\_size](#default_page_size) | `integer` | Optional | can be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-paginationconfig-properties-default_page_size.md "undefined#/instantiate/definitions/PaginationConfig/properties/default_page_size") |
| [max\_page\_size](#max_page_size)         | `integer` | Optional | can be null | [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-paginationconfig-properties-max_page_size.md "undefined#/instantiate/definitions/PaginationConfig/properties/max_page_size")         |

### default\_page\_size

The default number of elements in a page.

Shall be less or equal than `max_page_size`. Default to '10' if not set.

`default_page_size`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-paginationconfig-properties-default_page_size.md "undefined#/instantiate/definitions/PaginationConfig/properties/default_page_size")

#### default\_page\_size Type

`integer`

#### default\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

### max\_page\_size

The maximum elements a page can contains.

Shall be less than `u32::MAX - 1`. Default to '30' if not set.

`max_page_size`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-instantiatemsg-definitions-paginationconfig-properties-max_page_size.md "undefined#/instantiate/definitions/PaginationConfig/properties/max_page_size")

#### max\_page\_size Type

`integer`

#### max\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

## Definitions group Uint128

Reference this group by using

```json
{"$ref":"undefined#/instantiate/definitions/Uint128"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
# Untitled object in okp4-objectarium Schema

```txt
undefined#/query/oneOf/0/properties/bucket
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## bucket Type

`object` ([Details](okp4-objectarium-querymsg-oneof-bucket-properties-bucket.md))
# Bucket Schema

```txt
undefined#/query/oneOf/0
```

Bucket returns the bucket information.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

`object` ([Bucket](okp4-objectarium-querymsg-oneof-bucket.md))

# 0 Properties

| Property          | Type     | Required | Nullable       | Defined by                                                                                                                   |
| :---------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| [bucket](#bucket) | `object` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-querymsg-oneof-bucket-properties-bucket.md "undefined#/query/oneOf/0/properties/bucket") |

## bucket



`bucket`

*   is required

*   Type: `object` ([Details](okp4-objectarium-querymsg-oneof-bucket-properties-bucket.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-bucket-properties-bucket.md "undefined#/query/oneOf/0/properties/bucket")

### bucket Type

`object` ([Details](okp4-objectarium-querymsg-oneof-bucket-properties-bucket.md))
# Untitled string in okp4-objectarium Schema

```txt
undefined#/query/oneOf/1/properties/object/properties/id
```

The id of the object to get.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## id Type

`string`
# Untitled object in okp4-objectarium Schema

```txt
undefined#/query/oneOf/1/properties/object
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## object Type

`object` ([Details](okp4-objectarium-querymsg-oneof-object-properties-object.md))

# object Properties

| Property  | Type     | Required | Nullable       | Defined by                                                                                                                                               |
| :-------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [id](#id) | `string` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-querymsg-oneof-object-properties-object-properties-id.md "undefined#/query/oneOf/1/properties/object/properties/id") |

## id

The id of the object to get.

`id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-object-properties-object-properties-id.md "undefined#/query/oneOf/1/properties/object/properties/id")

### id Type

`string`
# Object Schema

```txt
undefined#/query/oneOf/1
```

Object returns the object information with the given id.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`object` ([Object](okp4-objectarium-querymsg-oneof-object.md))

# 1 Properties

| Property          | Type     | Required | Nullable       | Defined by                                                                                                                   |
| :---------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| [object](#object) | `object` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-querymsg-oneof-object-properties-object.md "undefined#/query/oneOf/1/properties/object") |

## object



`object`

*   is required

*   Type: `object` ([Details](okp4-objectarium-querymsg-oneof-object-properties-object.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-object-properties-object.md "undefined#/query/oneOf/1/properties/object")

### object Type

`object` ([Details](okp4-objectarium-querymsg-oneof-object-properties-object.md))
# Untitled string in okp4-objectarium Schema

```txt
undefined#/query/oneOf/3/properties/object_data/properties/id
```

The id of the object to get.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## id Type

`string`
# Untitled object in okp4-objectarium Schema

```txt
undefined#/query/oneOf/3/properties/object_data
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## object\_data Type

`object` ([Details](okp4-objectarium-querymsg-oneof-objectdata-properties-object_data.md))

# object\_data Properties

| Property  | Type     | Required | Nullable       | Defined by                                                                                                                                                             |
| :-------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [id](#id) | `string` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectdata-properties-object_data-properties-id.md "undefined#/query/oneOf/3/properties/object_data/properties/id") |

## id

The id of the object to get.

`id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectdata-properties-object_data-properties-id.md "undefined#/query/oneOf/3/properties/object_data/properties/id")

### id Type

`string`
# ObjectData Schema

```txt
undefined#/query/oneOf/3
```

ObjectData returns the content of the object with the given id.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 3 Type

`object` ([ObjectData](okp4-objectarium-querymsg-oneof-objectdata.md))

# 3 Properties

| Property                     | Type     | Required | Nullable       | Defined by                                                                                                                                 |
| :--------------------------- | :------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| [object\_data](#object_data) | `object` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectdata-properties-object_data.md "undefined#/query/oneOf/3/properties/object_data") |

## object\_data



`object_data`

*   is required

*   Type: `object` ([Details](okp4-objectarium-querymsg-oneof-objectdata-properties-object_data.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectdata-properties-object_data.md "undefined#/query/oneOf/3/properties/object_data")

### object\_data Type

`object` ([Details](okp4-objectarium-querymsg-oneof-objectdata-properties-object_data.md))
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/query/oneOf/4/properties/object_pins/properties/after
```

The point in the sequence to start returning pins.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## after Type

`string`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/query/oneOf/4/properties/object_pins/properties/first
```

The number of pins to return.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## first Type

`integer`

## first Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
# Untitled string in okp4-objectarium Schema

```txt
undefined#/query/oneOf/4/properties/object_pins/properties/id
```

The id of the object to get the pins for.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## id Type

`string`
# Untitled object in okp4-objectarium Schema

```txt
undefined#/query/oneOf/4/properties/object_pins
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## object\_pins Type

`object` ([Details](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins.md))

# object\_pins Properties

| Property        | Type      | Required | Nullable       | Defined by                                                                                                                                                                   |
| :-------------- | :-------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [after](#after) | `string`  | Optional | can be null    | [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins-properties-after.md "undefined#/query/oneOf/4/properties/object_pins/properties/after") |
| [first](#first) | `integer` | Optional | can be null    | [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins-properties-first.md "undefined#/query/oneOf/4/properties/object_pins/properties/first") |
| [id](#id)       | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins-properties-id.md "undefined#/query/oneOf/4/properties/object_pins/properties/id")       |

## after

The point in the sequence to start returning pins.

`after`

*   is optional

*   Type: `string`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins-properties-after.md "undefined#/query/oneOf/4/properties/object_pins/properties/after")

### after Type

`string`

## first

The number of pins to return.

`first`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins-properties-first.md "undefined#/query/oneOf/4/properties/object_pins/properties/first")

### first Type

`integer`

### first Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

## id

The id of the object to get the pins for.

`id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins-properties-id.md "undefined#/query/oneOf/4/properties/object_pins/properties/id")

### id Type

`string`
# ObjectPins Schema

```txt
undefined#/query/oneOf/4
```

ObjectPins returns the list of addresses that pinned the object with the given id with support for pagination.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 4 Type

`object` ([ObjectPins](okp4-objectarium-querymsg-oneof-objectpins.md))

# 4 Properties

| Property                     | Type     | Required | Nullable       | Defined by                                                                                                                                 |
| :--------------------------- | :------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| [object\_pins](#object_pins) | `object` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins.md "undefined#/query/oneOf/4/properties/object_pins") |

## object\_pins



`object_pins`

*   is required

*   Type: `object` ([Details](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins.md "undefined#/query/oneOf/4/properties/object_pins")

### object\_pins Type

`object` ([Details](okp4-objectarium-querymsg-oneof-objectpins-properties-object_pins.md))
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/query/oneOf/2/properties/objects/properties/address
```

The owner of the objects to get.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## address Type

`string`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/query/oneOf/2/properties/objects/properties/after
```

The point in the sequence to start returning objects.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## after Type

`string`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/query/oneOf/2/properties/objects/properties/first
```

The number of objects to return.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## first Type

`integer`

## first Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
# Untitled object in okp4-objectarium Schema

```txt
undefined#/query/oneOf/2/properties/objects
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## objects Type

`object` ([Details](okp4-objectarium-querymsg-oneof-objects-properties-objects.md))

# objects Properties

| Property            | Type      | Required | Nullable    | Defined by                                                                                                                                                            |
| :------------------ | :-------- | :------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [address](#address) | `string`  | Optional | can be null | [okp4-objectarium](okp4-objectarium-querymsg-oneof-objects-properties-objects-properties-address.md "undefined#/query/oneOf/2/properties/objects/properties/address") |
| [after](#after)     | `string`  | Optional | can be null | [okp4-objectarium](okp4-objectarium-querymsg-oneof-objects-properties-objects-properties-after.md "undefined#/query/oneOf/2/properties/objects/properties/after")     |
| [first](#first)     | `integer` | Optional | can be null | [okp4-objectarium](okp4-objectarium-querymsg-oneof-objects-properties-objects-properties-first.md "undefined#/query/oneOf/2/properties/objects/properties/first")     |

## address

The owner of the objects to get.

`address`

*   is optional

*   Type: `string`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-objects-properties-objects-properties-address.md "undefined#/query/oneOf/2/properties/objects/properties/address")

### address Type

`string`

## after

The point in the sequence to start returning objects.

`after`

*   is optional

*   Type: `string`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-objects-properties-objects-properties-after.md "undefined#/query/oneOf/2/properties/objects/properties/after")

### after Type

`string`

## first

The number of objects to return.

`first`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-objects-properties-objects-properties-first.md "undefined#/query/oneOf/2/properties/objects/properties/first")

### first Type

`integer`

### first Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
# Objects Schema

```txt
undefined#/query/oneOf/2
```

Objects returns the list of objects in the bucket with support for pagination.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 2 Type

`object` ([Objects](okp4-objectarium-querymsg-oneof-objects.md))

# 2 Properties

| Property            | Type     | Required | Nullable       | Defined by                                                                                                                      |
| :------------------ | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| [objects](#objects) | `object` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-querymsg-oneof-objects-properties-objects.md "undefined#/query/oneOf/2/properties/objects") |

## objects



`objects`

*   is required

*   Type: `object` ([Details](okp4-objectarium-querymsg-oneof-objects-properties-objects.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-querymsg-oneof-objects-properties-objects.md "undefined#/query/oneOf/2/properties/objects")

### objects Type

`object` ([Details](okp4-objectarium-querymsg-oneof-objects-properties-objects.md))
# QueryMsg Schema

```txt
undefined#/query
```

Query messages

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## query Type

merged type ([QueryMsg](okp4-objectarium-querymsg.md))

one (and only one) of

*   [Bucket](okp4-objectarium-querymsg-oneof-bucket.md "check type definition")

*   [Object](okp4-objectarium-querymsg-oneof-object.md "check type definition")

*   [Objects](okp4-objectarium-querymsg-oneof-objects.md "check type definition")

*   [ObjectData](okp4-objectarium-querymsg-oneof-objectdata.md "check type definition")

*   [ObjectPins](okp4-objectarium-querymsg-oneof-objectpins.md "check type definition")
# Binary Schema

```txt
undefined#/responses/object_data
```

Binary is a wrapper around <code>Vec&lt;u8&gt;</code> to add base64 de/serialization with serde. It also adds some helper methods to help encode inline.

This is only needed as serde-json-{core,wasm} has a horrible encoding for <code>Vec&lt;u8&gt;</code>. See also <https://github.com/CosmWasm/cosmwasm/blob/main/docs/MESSAGE_TYPES.md>.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## object\_data Type

`string` ([Binary](okp4-objectarium-responses-binary.md))
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketConfig/properties/hash_algorithm/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketConfig/properties/hash_algorithm/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketConfig/properties/hash_algorithm
```

The algorithm used to hash the content of the objects to generate the id of the objects. The algorithm is optional and if not set, the default algorithm is used.

The default algorithm is Sha256 .

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## hash\_algorithm Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm-anyof-1.md "check type definition")
# Untitled object in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketConfig
```

BucketConfig is the type of the configuration of a bucket.

The configuration is set at the instantiation of the bucket, and is immutable and cannot be changed. The configuration is optional and if not set, the default configuration is used.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## BucketConfig Type

`object` ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig.md))

# BucketConfig Properties

| Property                           | Type   | Required | Nullable       | Defined by                                                                                                                                                                                           |
| :--------------------------------- | :----- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [hash\_algorithm](#hash_algorithm) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md "undefined#/responses/bucket/definitions/BucketConfig/properties/hash_algorithm") |

## hash\_algorithm

The algorithm used to hash the content of the objects to generate the id of the objects. The algorithm is optional and if not set, the default algorithm is used.

The default algorithm is Sha256 .

`hash_algorithm`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md "undefined#/responses/bucket/definitions/BucketConfig/properties/hash_algorithm")

### hash\_algorithm Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm-anyof-1.md "check type definition")
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_pins/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_pins/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_pins
```

The maximum number of pins in the bucket for an object.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## max\_object\_pins Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins-anyof-1.md "check type definition")
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_size/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_size/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_size
```

The maximum size of the objects in the bucket.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## max\_object\_size Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size-anyof-1.md "check type definition")
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketLimits/properties/max_objects/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketLimits/properties/max_objects/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketLimits/properties/max_objects
```

The maximum number of objects in the bucket.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## max\_objects Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects-anyof-1.md "check type definition")
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketLimits/properties/max_total_size/anyOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled null in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketLimits/properties/max_total_size/anyOf/1
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`null`, the value must be null
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketLimits/properties/max_total_size
```

The maximum total size of the objects in the bucket.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## max\_total\_size Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size-anyof-1.md "check type definition")
# Untitled object in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/BucketLimits
```

BucketLimits is the type of the limits of a bucket.

The limits are optional and if not set, there is no limit.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## BucketLimits Type

`object` ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits.md))

# BucketLimits Properties

| Property                              | Type   | Required | Nullable       | Defined by                                                                                                                                                                                             |
| :------------------------------------ | :----- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [max\_object\_pins](#max_object_pins) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_pins") |
| [max\_object\_size](#max_object_size) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_size") |
| [max\_objects](#max_objects)          | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_objects")         |
| [max\_total\_size](#max_total_size)   | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_total_size")   |

## max\_object\_pins

The maximum number of pins in the bucket for an object.

`max_object_pins`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_pins")

### max\_object\_pins Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins-anyof-1.md "check type definition")

## max\_object\_size

The maximum size of the objects in the bucket.

`max_object_size`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_size")

### max\_object\_size Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size-anyof-1.md "check type definition")

## max\_objects

The maximum number of objects in the bucket.

`max_objects`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_objects")

### max\_objects Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects-anyof-1.md "check type definition")

## max\_total\_size

The maximum total size of the objects in the bucket.

`max_total_size`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_total_size")

### max\_total\_size Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size-anyof-1.md "check type definition")
# MD5 Schema

```txt
undefined#/responses/bucket/definitions/HashAlgorithm/oneOf/0
```

Represents the MD5 algorithm. MD5 is a widely used cryptographic hash function that produces a 128-bit hash value. The computational cost of MD5 is relatively low compared to other hash functions, but its short hash length makes it easier to find hash collisions. It is now considered insecure for cryptographic purposes, but can still used in non-security contexts.

MD5 hashes are stored on-chain as 32 hexadecimal characters.

See [the MD5 Wikipedia page](https://en.wikipedia.org/wiki/MD5) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

`string` ([MD5](okp4-objectarium-responses-bucketresponse-definitions-hashalgorithm-oneof-md5.md))

## 0 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value    | Explanation |
| :------- | :---------- |
| `"m_d5"` |             |
# SHA1 Schema

```txt
undefined#/responses/bucket/definitions/HashAlgorithm/oneOf/1
```

Represents the SHA-224 algorithm. SHA-224 is a variant of the SHA-2 family of hash functions that produces a 224-bit hash value. It is similar to SHA-256, but with a shorter output size. The computational cost of SHA-224 is moderate, and its relatively short hash length makes it easier to store and transmit.

SHA-224 hashes are stored on-chain as 56 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`string` ([SHA1](okp4-objectarium-responses-bucketresponse-definitions-hashalgorithm-oneof-sha1.md))

## 1 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value      | Explanation |
| :--------- | :---------- |
| `"sha224"` |             |
# SHA256 Schema

```txt
undefined#/responses/bucket/definitions/HashAlgorithm/oneOf/2
```

Represents the SHA-256 algorithm. SHA-256 is a member of the SHA-2 family of hash functions that produces a 256-bit hash value. It is widely used in cryptography and other security-related applications. The computational cost of SHA-256 is moderate, and its hash length strikes a good balance between security and convenience.

SHA-256 hashes are stored on-chain as 64 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 2 Type

`string` ([SHA256](okp4-objectarium-responses-bucketresponse-definitions-hashalgorithm-oneof-sha256.md))

## 2 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value      | Explanation |
| :--------- | :---------- |
| `"sha256"` |             |
# SHA384 Schema

```txt
undefined#/responses/bucket/definitions/HashAlgorithm/oneOf/3
```

Represents the SHA-384 algorithm. SHA-384 is a variant of the SHA-2 family of hash functions that produces a 384-bit hash value. It is similar to SHA-512, but with a shorter output size. The computational cost of SHA-384 is relatively high, but its longer hash length provides better security against hash collisions.

SHA-384 hashes are stored on-chain as 96 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 3 Type

`string` ([SHA384](okp4-objectarium-responses-bucketresponse-definitions-hashalgorithm-oneof-sha384.md))

## 3 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value      | Explanation |
| :--------- | :---------- |
| `"sha384"` |             |
# SHA512 Schema

```txt
undefined#/responses/bucket/definitions/HashAlgorithm/oneOf/4
```

Represents the SHA-512 algorithm. SHA-512 is a member of the SHA-2 family of hash functions that produces a 512-bit hash value. It is widely used in cryptography and other security-related applications. The computational cost of SHA-512 is relatively high, but its longer hash length provides better security against hash collisions.

SHA-512 hashes are stored on-chain as 128 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 4 Type

`string` ([SHA512](okp4-objectarium-responses-bucketresponse-definitions-hashalgorithm-oneof-sha512.md))

## 4 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value      | Explanation |
| :--------- | :---------- |
| `"sha512"` |             |
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/HashAlgorithm
```

HashAlgorithm is an enumeration that defines the different hash algorithms supported for hashing the content of objects.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## HashAlgorithm Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-hashalgorithm.md))

one (and only one) of

*   [MD5](okp4-objectarium-responses-bucketresponse-definitions-hashalgorithm-oneof-md5.md "check type definition")

*   [SHA1](okp4-objectarium-responses-bucketresponse-definitions-hashalgorithm-oneof-sha1.md "check type definition")

*   [SHA256](okp4-objectarium-responses-bucketresponse-definitions-hashalgorithm-oneof-sha256.md "check type definition")

*   [SHA384](okp4-objectarium-responses-bucketresponse-definitions-hashalgorithm-oneof-sha384.md "check type definition")

*   [SHA512](okp4-objectarium-responses-bucketresponse-definitions-hashalgorithm-oneof-sha512.md "check type definition")
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/PaginationConfig/properties/default_page_size
```

The default number of elements in a page.

Shall be less or equal than `max_page_size`. Default to '10' if not set.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## default\_page\_size Type

`integer`

## default\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/PaginationConfig/properties/max_page_size
```

The maximum elements a page can contains.

Shall be less than `u32::MAX - 1`. Default to '30' if not set.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## max\_page\_size Type

`integer`

## max\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
# Untitled object in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/PaginationConfig
```

PaginationConfig is the type carrying configuration for paginated queries.

The fields are optional and if not set, there is a default configuration.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## PaginationConfig Type

`object` ([Details](okp4-objectarium-responses-bucketresponse-definitions-paginationconfig.md))

# PaginationConfig Properties

| Property                                  | Type      | Required | Nullable    | Defined by                                                                                                                                                                                                         |
| :---------------------------------------- | :-------- | :------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [default\_page\_size](#default_page_size) | `integer` | Optional | can be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-paginationconfig-properties-default_page_size.md "undefined#/responses/bucket/definitions/PaginationConfig/properties/default_page_size") |
| [max\_page\_size](#max_page_size)         | `integer` | Optional | can be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-paginationconfig-properties-max_page_size.md "undefined#/responses/bucket/definitions/PaginationConfig/properties/max_page_size")         |

## default\_page\_size

The default number of elements in a page.

Shall be less or equal than `max_page_size`. Default to '10' if not set.

`default_page_size`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-paginationconfig-properties-default_page_size.md "undefined#/responses/bucket/definitions/PaginationConfig/properties/default_page_size")

### default\_page\_size Type

`integer`

### default\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

## max\_page\_size

The maximum elements a page can contains.

Shall be less than `u32::MAX - 1`. Default to '30' if not set.

`max_page_size`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-paginationconfig-properties-max_page_size.md "undefined#/responses/bucket/definitions/PaginationConfig/properties/max_page_size")

### max\_page\_size Type

`integer`

### max\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
# Untitled string in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions/Uint128
```

A thin wrapper around u128 that is using strings for JSON encoding/decoding, such that the full u128 range can be used for clients that convert JSON numbers to floats, like JavaScript and jq.

# Examples

Use `from` to create instances of this and `u128` to get the value out:

````# use cosmwasm_std::Uint128; let a = Uint128::from(123u128); assert_eq!(a.u128(), 123);

let b = Uint128::from(42u64); assert_eq!(b.u128(), 42);

let c = Uint128::from(70u32); assert_eq!(c.u128(), 70); ```
````

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## Uint128 Type

`string`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/definitions
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## definitions Type

unknown
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/properties/config/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/properties/config
```

The configuration of the bucket.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## config Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-properties-config.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-config-allof-0.md "check type definition")
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/properties/limits/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/properties/limits
```

The limits of the bucket.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## limits Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-properties-limits.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-limits-allof-0.md "check type definition")
# Untitled string in okp4-objectarium Schema

```txt
undefined#/responses/bucket/properties/name
```

The name of the bucket.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## name Type

`string`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/properties/pagination/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/bucket/properties/pagination
```

The configuration for paginated query.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## pagination Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-properties-pagination.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-pagination-allof-0.md "check type definition")
# BucketResponse Schema

```txt
undefined#/responses/bucket
```

BucketResponse is the response of the Bucket query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## bucket Type

`object` ([BucketResponse](okp4-objectarium-responses-bucketresponse.md))

# bucket Properties

| Property                  | Type     | Required | Nullable       | Defined by                                                                                                                                 |
| :------------------------ | :------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| [config](#config)         | Merged   | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-config.md "undefined#/responses/bucket/properties/config")         |
| [limits](#limits)         | Merged   | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-limits.md "undefined#/responses/bucket/properties/limits")         |
| [name](#name)             | `string` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-name.md "undefined#/responses/bucket/properties/name")             |
| [pagination](#pagination) | Merged   | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-pagination.md "undefined#/responses/bucket/properties/pagination") |

## config

The configuration of the bucket.

`config`

*   is required

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-properties-config.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-config.md "undefined#/responses/bucket/properties/config")

### config Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-properties-config.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-config-allof-0.md "check type definition")

## limits

The limits of the bucket.

`limits`

*   is required

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-properties-limits.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-limits.md "undefined#/responses/bucket/properties/limits")

### limits Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-properties-limits.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-limits-allof-0.md "check type definition")

## name

The name of the bucket.

`name`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-name.md "undefined#/responses/bucket/properties/name")

### name Type

`string`

## pagination

The configuration for paginated query.

`pagination`

*   is required

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-properties-pagination.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-pagination.md "undefined#/responses/bucket/properties/pagination")

### pagination Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-properties-pagination.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-properties-pagination-allof-0.md "check type definition")

# BucketResponse Definitions

## Definitions group BucketConfig

Reference this group by using

```json
{"$ref":"undefined#/responses/bucket/definitions/BucketConfig"}
```

| Property                           | Type   | Required | Nullable       | Defined by                                                                                                                                                                                           |
| :--------------------------------- | :----- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [hash\_algorithm](#hash_algorithm) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md "undefined#/responses/bucket/definitions/BucketConfig/properties/hash_algorithm") |

### hash\_algorithm

The algorithm used to hash the content of the objects to generate the id of the objects. The algorithm is optional and if not set, the default algorithm is used.

The default algorithm is Sha256 .

`hash_algorithm`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md "undefined#/responses/bucket/definitions/BucketConfig/properties/hash_algorithm")

#### hash\_algorithm Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketconfig-properties-hash_algorithm-anyof-1.md "check type definition")

## Definitions group BucketLimits

Reference this group by using

```json
{"$ref":"undefined#/responses/bucket/definitions/BucketLimits"}
```

| Property                              | Type   | Required | Nullable       | Defined by                                                                                                                                                                                             |
| :------------------------------------ | :----- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [max\_object\_pins](#max_object_pins) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_pins") |
| [max\_object\_size](#max_object_size) | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_size") |
| [max\_objects](#max_objects)          | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_objects")         |
| [max\_total\_size](#max_total_size)   | Merged | Optional | cannot be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_total_size")   |

### max\_object\_pins

The maximum number of pins in the bucket for an object.

`max_object_pins`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_pins")

#### max\_object\_pins Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_pins-anyof-1.md "check type definition")

### max\_object\_size

The maximum size of the objects in the bucket.

`max_object_size`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_object_size")

#### max\_object\_size Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_object_size-anyof-1.md "check type definition")

### max\_objects

The maximum number of objects in the bucket.

`max_objects`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_objects")

#### max\_objects Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_objects-anyof-1.md "check type definition")

### max\_total\_size

The maximum total size of the objects in the bucket.

`max_total_size`

*   is optional

*   Type: merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md "undefined#/responses/bucket/definitions/BucketLimits/properties/max_total_size")

#### max\_total\_size Type

merged type ([Details](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size.md))

any of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size-anyof-0.md "check type definition")

*   [Untitled null in okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-bucketlimits-properties-max_total_size-anyof-1.md "check type definition")

## Definitions group HashAlgorithm

Reference this group by using

```json
{"$ref":"undefined#/responses/bucket/definitions/HashAlgorithm"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group PaginationConfig

Reference this group by using

```json
{"$ref":"undefined#/responses/bucket/definitions/PaginationConfig"}
```

| Property                                  | Type      | Required | Nullable    | Defined by                                                                                                                                                                                                         |
| :---------------------------------------- | :-------- | :------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [default\_page\_size](#default_page_size) | `integer` | Optional | can be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-paginationconfig-properties-default_page_size.md "undefined#/responses/bucket/definitions/PaginationConfig/properties/default_page_size") |
| [max\_page\_size](#max_page_size)         | `integer` | Optional | can be null | [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-paginationconfig-properties-max_page_size.md "undefined#/responses/bucket/definitions/PaginationConfig/properties/max_page_size")         |

### default\_page\_size

The default number of elements in a page.

Shall be less or equal than `max_page_size`. Default to '10' if not set.

`default_page_size`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-paginationconfig-properties-default_page_size.md "undefined#/responses/bucket/definitions/PaginationConfig/properties/default_page_size")

#### default\_page\_size Type

`integer`

#### default\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

### max\_page\_size

The maximum elements a page can contains.

Shall be less than `u32::MAX - 1`. Default to '30' if not set.

`max_page_size`

*   is optional

*   Type: `integer`

*   can be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-bucketresponse-definitions-paginationconfig-properties-max_page_size.md "undefined#/responses/bucket/definitions/PaginationConfig/properties/max_page_size")

#### max\_page\_size Type

`integer`

#### max\_page\_size Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

## Definitions group Uint128

Reference this group by using

```json
{"$ref":"undefined#/responses/bucket/definitions/Uint128"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
# Untitled string in okp4-objectarium Schema

```txt
undefined#/responses/object_pins/definitions/PageInfo/properties/cursor
```

The cursor to the next page.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## cursor Type

`string`
# Untitled boolean in okp4-objectarium Schema

```txt
undefined#/responses/object_pins/definitions/PageInfo/properties/has_next_page
```

Tells if there is a next page.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## has\_next\_page Type

`boolean`
# PageInfo Schema

```txt
undefined#/responses/object_pins/definitions/PageInfo
```

PageInfo is the page information returned for paginated queries.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## PageInfo Type

`object` ([PageInfo](okp4-objectarium-responses-objectpinsresponse-definitions-pageinfo.md))

# PageInfo Properties

| Property                          | Type      | Required | Nullable       | Defined by                                                                                                                                                                                          |
| :-------------------------------- | :-------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [cursor](#cursor)                 | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectpinsresponse-definitions-pageinfo-properties-cursor.md "undefined#/responses/object_pins/definitions/PageInfo/properties/cursor")               |
| [has\_next\_page](#has_next_page) | `boolean` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectpinsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/object_pins/definitions/PageInfo/properties/has_next_page") |

## cursor

The cursor to the next page.

`cursor`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectpinsresponse-definitions-pageinfo-properties-cursor.md "undefined#/responses/object_pins/definitions/PageInfo/properties/cursor")

### cursor Type

`string`

## has\_next\_page

Tells if there is a next page.

`has_next_page`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectpinsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/object_pins/definitions/PageInfo/properties/has_next_page")

### has\_next\_page Type

`boolean`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/object_pins/definitions
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## definitions Type

unknown
# Untitled string in okp4-objectarium Schema

```txt
undefined#/responses/object_pins/properties/data/items
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## items Type

`string`
# Untitled array in okp4-objectarium Schema

```txt
undefined#/responses/object_pins/properties/data
```

The list of addresses that pinned the object.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## data Type

`string[]`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/object_pins/properties/page_info/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/object_pins/properties/page_info
```

The page information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## page\_info Type

merged type ([Details](okp4-objectarium-responses-objectpinsresponse-properties-page_info.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-objectpinsresponse-properties-page_info-allof-0.md "check type definition")
# ObjectPinsResponse Schema

```txt
undefined#/responses/object_pins
```

ObjectPinsResponse is the response of the GetObjectPins query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## object\_pins Type

`object` ([ObjectPinsResponse](okp4-objectarium-responses-objectpinsresponse.md))

# object\_pins Properties

| Property                 | Type    | Required | Nullable       | Defined by                                                                                                                                        |
| :----------------------- | :------ | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| [data](#data)            | `array` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectpinsresponse-properties-data.md "undefined#/responses/object_pins/properties/data")           |
| [page\_info](#page_info) | Merged  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectpinsresponse-properties-page_info.md "undefined#/responses/object_pins/properties/page_info") |

## data

The list of addresses that pinned the object.

`data`

*   is required

*   Type: `string[]`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectpinsresponse-properties-data.md "undefined#/responses/object_pins/properties/data")

### data Type

`string[]`

## page\_info

The page information.

`page_info`

*   is required

*   Type: merged type ([Details](okp4-objectarium-responses-objectpinsresponse-properties-page_info.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectpinsresponse-properties-page_info.md "undefined#/responses/object_pins/properties/page_info")

### page\_info Type

merged type ([Details](okp4-objectarium-responses-objectpinsresponse-properties-page_info.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-objectpinsresponse-properties-page_info-allof-0.md "check type definition")

# ObjectPinsResponse Definitions

## Definitions group PageInfo

Reference this group by using

```json
{"$ref":"undefined#/responses/object_pins/definitions/PageInfo"}
```

| Property                          | Type      | Required | Nullable       | Defined by                                                                                                                                                                                          |
| :-------------------------------- | :-------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [cursor](#cursor)                 | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectpinsresponse-definitions-pageinfo-properties-cursor.md "undefined#/responses/object_pins/definitions/PageInfo/properties/cursor")               |
| [has\_next\_page](#has_next_page) | `boolean` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectpinsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/object_pins/definitions/PageInfo/properties/has_next_page") |

### cursor

The cursor to the next page.

`cursor`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectpinsresponse-definitions-pageinfo-properties-cursor.md "undefined#/responses/object_pins/definitions/PageInfo/properties/cursor")

#### cursor Type

`string`

### has\_next\_page

Tells if there is a next page.

`has_next_page`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectpinsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/object_pins/definitions/PageInfo/properties/has_next_page")

#### has\_next\_page Type

`boolean`
# Untitled string in okp4-objectarium Schema

```txt
undefined#/responses/object/definitions/Uint128
```

A thin wrapper around u128 that is using strings for JSON encoding/decoding, such that the full u128 range can be used for clients that convert JSON numbers to floats, like JavaScript and jq.

# Examples

Use `from` to create instances of this and `u128` to get the value out:

````# use cosmwasm_std::Uint128; let a = Uint128::from(123u128); assert_eq!(a.u128(), 123);

let b = Uint128::from(42u64); assert_eq!(b.u128(), 42);

let c = Uint128::from(70u32); assert_eq!(c.u128(), 70); ```
````

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## Uint128 Type

`string`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/object/definitions
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## definitions Type

unknown
# Untitled string in okp4-objectarium Schema

```txt
undefined#/responses/object/properties/id
```

The id of the object.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## id Type

`string`
# Untitled boolean in okp4-objectarium Schema

```txt
undefined#/responses/object/properties/is_pinned
```

Tells if the object is pinned by at least one address.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## is\_pinned Type

`boolean`
# Untitled string in okp4-objectarium Schema

```txt
undefined#/responses/object/properties/owner
```

The owner of the object.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## owner Type

`string`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/object/properties/size/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/object/properties/size
```

The size of the object.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## size Type

merged type ([Details](okp4-objectarium-responses-objectresponse-properties-size.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-size-allof-0.md "check type definition")
# ObjectResponse Schema

```txt
undefined#/responses/object
```

ObjectResponse is the response of the Object query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## object Type

`object` ([ObjectResponse](okp4-objectarium-responses-objectresponse.md))

# object Properties

| Property                 | Type      | Required | Nullable       | Defined by                                                                                                                               |
| :----------------------- | :-------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| [id](#id)                | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-id.md "undefined#/responses/object/properties/id")               |
| [is\_pinned](#is_pinned) | `boolean` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-is_pinned.md "undefined#/responses/object/properties/is_pinned") |
| [owner](#owner)          | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-owner.md "undefined#/responses/object/properties/owner")         |
| [size](#size)            | Merged    | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-size.md "undefined#/responses/object/properties/size")           |

## id

The id of the object.

`id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-id.md "undefined#/responses/object/properties/id")

### id Type

`string`

## is\_pinned

Tells if the object is pinned by at least one address.

`is_pinned`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-is_pinned.md "undefined#/responses/object/properties/is_pinned")

### is\_pinned Type

`boolean`

## owner

The owner of the object.

`owner`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-owner.md "undefined#/responses/object/properties/owner")

### owner Type

`string`

## size

The size of the object.

`size`

*   is required

*   Type: merged type ([Details](okp4-objectarium-responses-objectresponse-properties-size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-size.md "undefined#/responses/object/properties/size")

### size Type

merged type ([Details](okp4-objectarium-responses-objectresponse-properties-size.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-objectresponse-properties-size-allof-0.md "check type definition")

# ObjectResponse Definitions

## Definitions group Uint128

Reference this group by using

```json
{"$ref":"undefined#/responses/object/definitions/Uint128"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
# Untitled string in okp4-objectarium Schema

```txt
undefined#/responses/objects/definitions/ObjectResponse/properties/id
```

The id of the object.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## id Type

`string`
# Untitled boolean in okp4-objectarium Schema

```txt
undefined#/responses/objects/definitions/ObjectResponse/properties/is_pinned
```

Tells if the object is pinned by at least one address.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## is\_pinned Type

`boolean`
# Untitled string in okp4-objectarium Schema

```txt
undefined#/responses/objects/definitions/ObjectResponse/properties/owner
```

The owner of the object.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## owner Type

`string`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/objects/definitions/ObjectResponse/properties/size/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/objects/definitions/ObjectResponse/properties/size
```

The size of the object.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## size Type

merged type ([Details](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size-allof-0.md "check type definition")
# ObjectResponse Schema

```txt
undefined#/responses/objects/definitions/ObjectResponse
```

ObjectResponse is the response of the Object query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## ObjectResponse Type

`object` ([ObjectResponse](okp4-objectarium-responses-objectsresponse-definitions-objectresponse.md))

# ObjectResponse Properties

| Property                 | Type      | Required | Nullable       | Defined by                                                                                                                                                                                       |
| :----------------------- | :-------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [id](#id)                | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-id.md "undefined#/responses/objects/definitions/ObjectResponse/properties/id")               |
| [is\_pinned](#is_pinned) | `boolean` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-is_pinned.md "undefined#/responses/objects/definitions/ObjectResponse/properties/is_pinned") |
| [owner](#owner)          | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-owner.md "undefined#/responses/objects/definitions/ObjectResponse/properties/owner")         |
| [size](#size)            | Merged    | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size.md "undefined#/responses/objects/definitions/ObjectResponse/properties/size")           |

## id

The id of the object.

`id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-id.md "undefined#/responses/objects/definitions/ObjectResponse/properties/id")

### id Type

`string`

## is\_pinned

Tells if the object is pinned by at least one address.

`is_pinned`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-is_pinned.md "undefined#/responses/objects/definitions/ObjectResponse/properties/is_pinned")

### is\_pinned Type

`boolean`

## owner

The owner of the object.

`owner`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-owner.md "undefined#/responses/objects/definitions/ObjectResponse/properties/owner")

### owner Type

`string`

## size

The size of the object.

`size`

*   is required

*   Type: merged type ([Details](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size.md "undefined#/responses/objects/definitions/ObjectResponse/properties/size")

### size Type

merged type ([Details](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size-allof-0.md "check type definition")
# Untitled string in okp4-objectarium Schema

```txt
undefined#/responses/objects/definitions/PageInfo/properties/cursor
```

The cursor to the next page.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## cursor Type

`string`
# Untitled boolean in okp4-objectarium Schema

```txt
undefined#/responses/objects/definitions/PageInfo/properties/has_next_page
```

Tells if there is a next page.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## has\_next\_page Type

`boolean`
# PageInfo Schema

```txt
undefined#/responses/objects/definitions/PageInfo
```

PageInfo is the page information returned for paginated queries.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## PageInfo Type

`object` ([PageInfo](okp4-objectarium-responses-objectsresponse-definitions-pageinfo.md))

# PageInfo Properties

| Property                          | Type      | Required | Nullable       | Defined by                                                                                                                                                                                   |
| :-------------------------------- | :-------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [cursor](#cursor)                 | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-pageinfo-properties-cursor.md "undefined#/responses/objects/definitions/PageInfo/properties/cursor")               |
| [has\_next\_page](#has_next_page) | `boolean` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/objects/definitions/PageInfo/properties/has_next_page") |

## cursor

The cursor to the next page.

`cursor`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-pageinfo-properties-cursor.md "undefined#/responses/objects/definitions/PageInfo/properties/cursor")

### cursor Type

`string`

## has\_next\_page

Tells if there is a next page.

`has_next_page`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/objects/definitions/PageInfo/properties/has_next_page")

### has\_next\_page Type

`boolean`
# Untitled string in okp4-objectarium Schema

```txt
undefined#/responses/objects/definitions/Uint128
```

A thin wrapper around u128 that is using strings for JSON encoding/decoding, such that the full u128 range can be used for clients that convert JSON numbers to floats, like JavaScript and jq.

# Examples

Use `from` to create instances of this and `u128` to get the value out:

````# use cosmwasm_std::Uint128; let a = Uint128::from(123u128); assert_eq!(a.u128(), 123);

let b = Uint128::from(42u64); assert_eq!(b.u128(), 42);

let c = Uint128::from(70u32); assert_eq!(c.u128(), 70); ```
````

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## Uint128 Type

`string`
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/objects/definitions
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## definitions Type

unknown
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/objects/properties/data/items
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## items Type

unknown
# Untitled array in okp4-objectarium Schema

```txt
undefined#/responses/objects/properties/data
```

The list of objects in the bucket.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## data Type

unknown\[]
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/objects/properties/page_info/allOf/0
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

unknown
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses/objects/properties/page_info
```

The page information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## page\_info Type

merged type ([Details](okp4-objectarium-responses-objectsresponse-properties-page_info.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-objectsresponse-properties-page_info-allof-0.md "check type definition")
# ObjectsResponse Schema

```txt
undefined#/responses/objects
```

ObjectsResponse is the response of the Objects query.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | Yes        | Unknown status | No           | Forbidden         | Forbidden             | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## objects Type

`object` ([ObjectsResponse](okp4-objectarium-responses-objectsresponse.md))

# objects Properties

| Property                 | Type    | Required | Nullable       | Defined by                                                                                                                                 |
| :----------------------- | :------ | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| [data](#data)            | `array` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-properties-data.md "undefined#/responses/objects/properties/data")           |
| [page\_info](#page_info) | Merged  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-properties-page_info.md "undefined#/responses/objects/properties/page_info") |

## data

The list of objects in the bucket.

`data`

*   is required

*   Type: unknown\[]

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-properties-data.md "undefined#/responses/objects/properties/data")

### data Type

unknown\[]

## page\_info

The page information.

`page_info`

*   is required

*   Type: merged type ([Details](okp4-objectarium-responses-objectsresponse-properties-page_info.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-properties-page_info.md "undefined#/responses/objects/properties/page_info")

### page\_info Type

merged type ([Details](okp4-objectarium-responses-objectsresponse-properties-page_info.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-objectsresponse-properties-page_info-allof-0.md "check type definition")

# ObjectsResponse Definitions

## Definitions group ObjectResponse

Reference this group by using

```json
{"$ref":"undefined#/responses/objects/definitions/ObjectResponse"}
```

| Property                 | Type      | Required | Nullable       | Defined by                                                                                                                                                                                       |
| :----------------------- | :-------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [id](#id)                | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-id.md "undefined#/responses/objects/definitions/ObjectResponse/properties/id")               |
| [is\_pinned](#is_pinned) | `boolean` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-is_pinned.md "undefined#/responses/objects/definitions/ObjectResponse/properties/is_pinned") |
| [owner](#owner)          | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-owner.md "undefined#/responses/objects/definitions/ObjectResponse/properties/owner")         |
| [size](#size)            | Merged    | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size.md "undefined#/responses/objects/definitions/ObjectResponse/properties/size")           |

### id

The id of the object.

`id`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-id.md "undefined#/responses/objects/definitions/ObjectResponse/properties/id")

#### id Type

`string`

### is\_pinned

Tells if the object is pinned by at least one address.

`is_pinned`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-is_pinned.md "undefined#/responses/objects/definitions/ObjectResponse/properties/is_pinned")

#### is\_pinned Type

`boolean`

### owner

The owner of the object.

`owner`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-owner.md "undefined#/responses/objects/definitions/ObjectResponse/properties/owner")

#### owner Type

`string`

### size

The size of the object.

`size`

*   is required

*   Type: merged type ([Details](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size.md))

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size.md "undefined#/responses/objects/definitions/ObjectResponse/properties/size")

#### size Type

merged type ([Details](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size.md))

all of

*   [Untitled undefined type in okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-objectresponse-properties-size-allof-0.md "check type definition")

## Definitions group PageInfo

Reference this group by using

```json
{"$ref":"undefined#/responses/objects/definitions/PageInfo"}
```

| Property                          | Type      | Required | Nullable       | Defined by                                                                                                                                                                                   |
| :-------------------------------- | :-------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [cursor](#cursor)                 | `string`  | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-pageinfo-properties-cursor.md "undefined#/responses/objects/definitions/PageInfo/properties/cursor")               |
| [has\_next\_page](#has_next_page) | `boolean` | Required | cannot be null | [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/objects/definitions/PageInfo/properties/has_next_page") |

### cursor

The cursor to the next page.

`cursor`

*   is required

*   Type: `string`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-pageinfo-properties-cursor.md "undefined#/responses/objects/definitions/PageInfo/properties/cursor")

#### cursor Type

`string`

### has\_next\_page

Tells if there is a next page.

`has_next_page`

*   is required

*   Type: `boolean`

*   cannot be null

*   defined in: [okp4-objectarium](okp4-objectarium-responses-objectsresponse-definitions-pageinfo-properties-has_next_page.md "undefined#/responses/objects/definitions/PageInfo/properties/has_next_page")

#### has\_next\_page Type

`boolean`

## Definitions group Uint128

Reference this group by using

```json
{"$ref":"undefined#/responses/objects/definitions/Uint128"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |
# Untitled undefined type in okp4-objectarium Schema

```txt
undefined#/responses
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## responses Type

unknown
# okp4-objectarium Schema

```txt
undefined
```

# Objectarium

## Overview

The `okp4-objectarium` smart contract enables the storage of arbitrary `objects` in any [Cosmos blockchains](https://cosmos.network/) using the [CosmWasm](https://cosmwasm.com/) framework.

This contract allows for storing `objects`, pinning and unpinning `objects` for a given sender address, and it also includes the ability to remove (forget) `objects` if they are no longer pinned.

## Usage

### Instantiate

The `okp4-objectarium` can be instantiated as follows, refer to the schema for more information on configuration, limits and pagination configuration:

```bash
okp4d tx wasm instantiate $CODE_ID \
    --label "my-storage" \
    --from $ADDR \
    --admin $ADMIN_ADDR \
    --gas 1000000 \
    --broadcast-mode block \
    '{"bucket":"my-bucket","limits":{}, "config": {}, "pagination": {}}'
```

### Execute

We can store an object by providing its data in base64 encoded, we can pin the stored object to prevent it from being removed:

```bash
okp4d tx wasm execute $CONTRACT_ADDR \
    --from $ADDR \
    --gas 1000000 \
    --broadcast-mode block \
    "{\"store_object\":{\"data\": \"$(cat my-data | base64)\",\"pin\":true}}"
```

The object id is stable as it is a hash, we can't store an object twice.

With the following commands we can pin and unpin existing objects:

```bash
okp4d tx wasm execute $CONTRACT_ADDR \
    --from $ADDR \
    --gas 1000000 \
    --broadcast-mode block \
    "{\"pin_object\":{\"id\": \"$OBJECT_ID\"}}"

okp4d tx wasm execute $CONTRACT_ADDR \
    --from $ADDR \
    --gas 1000000 \
    --broadcast-mode block \
    "{\"unpin_object\":{\"id\": \"$OBJECT_ID\"}}"
```

And if an object is not pinned, or pinned by the sender of transaction, we can remove it:

```bash
okp4d tx wasm execute $CONTRACT_ADDR \
    --from $ADDR \
    --gas 1000000 \
    --broadcast-mode block \
    "{\"forget_object\":{\"id\": \"$OBJECT_ID\"}}"
```

### Query

Query an object by its id:

```bash
okp4d query wasm contract-state smart $CONTRACT_ADDR \
    "{\"object\": {\"id\": \"$OBJECT_ID\"}}"
```

Or its data:

```bash
okp4d query wasm contract-state smart $CONTRACT_ADDR \
    "{\"object_data\": {\"id\": \"$OBJECT_ID\"}}"
```

We can also list the objects, eventually filtering on the object owner:

```bash
okp4d query wasm contract-state smart $CONTRACT_ADDR \
    "{\"objects\": {\"address\": \"okp41p8u47en82gmzfm259y6z93r9qe63l25dfwwng6\"}}"
```

And navigate in a cursor based pagination:

```bash
okp4d query wasm contract-state smart $CONTRACT_ADDR \
    "{\"objects\": {\"first\": 5, \"after\": \"23Y5t5DBe7DkPwfJo3Sd26Y8Z9epmtpA1FTpdG7DiG6MD8vPRTzzbQ9TccmyoBcePkPK6atUiqcAzJVo3TfYNBGY\"}}"
```

We can also query object pins with the same cursor based pagination:

```bash
okp4d query wasm contract-state smart $CONTRACT_ADDR \
    "{\"object_pins\": {\"id\": \"$OBJECT_ID\", \"first\": 5, \"after\": \"23Y5t5DBe7DkPwfJo3Sd26Y8Z9epmtpA1FTpdG7DiG6MD8vPRTzzbQ9TccmyoBcePkPK6atUiqcAzJVo3TfYNBGY\"}}"
```

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                   |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json](schema/okp4-objectarium.json "open original schema") |

## okp4-objectarium Type

unknown ([okp4-objectarium](okp4-objectarium.md))
