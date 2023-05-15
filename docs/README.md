# README

## Top-level Schemas

*   [okp4-cognitarium](./okp4-cognitarium.md "TriplestoreOverviewThe okp4-cognitarium smart contract enables the storage of RDF graphs triples (i") – `-`

*   [okp4-law-stone](./okp4-law-stone.md "Law StoneOverviewThe okp4-law-stone smart contract aims to provide GaaS (i") – `-`

*   [okp4-objectarium](./okp4-objectarium.md "ObjectariumOverviewThe okp4-objectarium smart contract enables the storage of arbitrary objects in any Cosmos blockchains using the CosmWasm framework") – `-`

## Other Schemas

### Objects

*   [Ask](./okp4-law-stone-querymsg-oneof-ask.md "If not broken, ask the logic module the provided query with the law program loaded") – `undefined#/query/oneOf/0`

*   [AskResponse](./okp4-law-stone-responses-askresponse.md) – `undefined#/responses/ask`

*   [BlankNode](./okp4-cognitarium-executemsg-definitions-node-oneof-blanknode.md "An RDF blank node") – `undefined#/execute/definitions/Node/oneOf/1`

*   [BlankNode](./okp4-cognitarium-querymsg-definitions-node-oneof-blanknode.md "An RDF blank node") – `undefined#/query/definitions/Node/oneOf/1`

*   [BlankNode](./okp4-cognitarium-responses-selectresponse-definitions-value-oneof-blanknode.md "Represents a blank node") – `undefined#/responses/select/definitions/Value/oneOf/2`

*   [Bucket](./okp4-objectarium-querymsg-oneof-bucket.md "Bucket returns the bucket information") – `undefined#/query/oneOf/0`

*   [BucketResponse](./okp4-objectarium-responses-bucketresponse.md "BucketResponse is the response of the Bucket query") – `undefined#/responses/bucket`

*   [DeleteData](./okp4-cognitarium-executemsg-oneof-deletedata.md "Delete the data (RDF triples) from the store matching the patterns defined by the provided query") – `undefined#/execute/oneOf/1`

*   [Describe](./okp4-cognitarium-querymsg-oneof-describe.md "Returns a description of the resource identified by the provided IRI as a set of RDF triples serialized in the provided format") – `undefined#/query/oneOf/2`

*   [DescribeQuery](./okp4-cognitarium-querymsg-definitions-describequery.md "Represents a DESCRIBE query over the triple store, allowing to retrieve a description of a resource as a set of triples serialized in a specific format") – `undefined#/query/definitions/DescribeQuery`

*   [DescribeResponse](./okp4-cognitarium-responses-describeresponse.md "Represents the response of a \[QueryMsg::Describe] query") – `undefined#/responses/describe`

*   [ForgetObject](./okp4-objectarium-executemsg-oneof-forgetobject.md "ForgetObject first unpin the object from the bucket for the considered sender, then remove it from the storage if it is not pinned anymore") – `undefined#/execute/oneOf/1`

*   [Full](./okp4-cognitarium-executemsg-definitions-iri-oneof-full.md "A full IRI") – `undefined#/execute/definitions/IRI/oneOf/1`

*   [Full](./okp4-cognitarium-querymsg-definitions-iri-oneof-full.md "A full IRI") – `undefined#/query/definitions/IRI/oneOf/1`

*   [Full](./okp4-cognitarium-responses-selectresponse-definitions-iri-oneof-full.md "A full IRI") – `undefined#/responses/select/definitions/IRI/oneOf/1`

*   [Head](./okp4-cognitarium-responses-selectresponse-definitions-head.md "Represents the head of a \[SelectResponse]") – `undefined#/responses/select/definitions/Head`

*   [InsertData](./okp4-cognitarium-executemsg-oneof-insertdata.md "Insert the data as RDF triples in the store") – `undefined#/execute/oneOf/0`

*   [InstantiateMsg](./okp4-cognitarium-instantiatemsg.md "Instantiate message") – `undefined#/instantiate`

*   [InstantiateMsg](./okp4-law-stone-instantiatemsg.md "Instantiate message") – `undefined#/instantiate`

*   [InstantiateMsg](./okp4-objectarium-instantiatemsg.md "Instantiate messages") – `undefined#/instantiate`

*   [LanguageTaggedString](./okp4-cognitarium-executemsg-definitions-literal-oneof-languagetaggedstring.md "A language-tagged string") – `undefined#/execute/definitions/Literal/oneOf/1`

*   [LanguageTaggedString](./okp4-cognitarium-querymsg-definitions-literal-oneof-languagetaggedstring.md "A language-tagged string") – `undefined#/query/definitions/Literal/oneOf/1`

*   [Literal](./okp4-cognitarium-executemsg-definitions-varornodeorliteral-oneof-literal.md "An RDF literal, i") – `undefined#/execute/definitions/VarOrNodeOrLiteral/oneOf/2`

*   [Literal](./okp4-cognitarium-querymsg-definitions-varornodeorliteral-oneof-literal.md "An RDF literal, i") – `undefined#/query/definitions/VarOrNodeOrLiteral/oneOf/2`

*   [Literal](./okp4-cognitarium-responses-selectresponse-definitions-value-oneof-literal.md "Represents a literal S with optional language tag L or datatype IRI D") – `undefined#/responses/select/definitions/Value/oneOf/1`

*   [NamedNode](./okp4-cognitarium-executemsg-definitions-node-oneof-namednode.md "An RDF IRI") – `undefined#/execute/definitions/Node/oneOf/0`

*   [NamedNode](./okp4-cognitarium-querymsg-definitions-node-oneof-namednode.md "An RDF IRI") – `undefined#/query/definitions/Node/oneOf/0`

*   [Node](./okp4-cognitarium-executemsg-definitions-varornode-oneof-node.md "A node, i") – `undefined#/execute/definitions/VarOrNode/oneOf/1`

*   [Node](./okp4-cognitarium-executemsg-definitions-varornodeorliteral-oneof-node.md "A node, i") – `undefined#/execute/definitions/VarOrNodeOrLiteral/oneOf/1`

*   [Node](./okp4-cognitarium-querymsg-definitions-varornode-oneof-node.md "A node, i") – `undefined#/query/definitions/VarOrNode/oneOf/1`

*   [Node](./okp4-cognitarium-querymsg-definitions-varornodeorliteral-oneof-node.md "A node, i") – `undefined#/query/definitions/VarOrNodeOrLiteral/oneOf/1`

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

*   [Prefix](./okp4-cognitarium-executemsg-definitions-prefix.md "Represents a prefix in a \[SelectQuery]") – `undefined#/execute/definitions/Prefix`

*   [Prefix](./okp4-cognitarium-querymsg-definitions-prefix.md "Represents a prefix in a \[SelectQuery]") – `undefined#/query/definitions/Prefix`

*   [Prefixed](./okp4-cognitarium-executemsg-definitions-iri-oneof-prefixed.md "An IRI prefixed with a prefix") – `undefined#/execute/definitions/IRI/oneOf/0`

*   [Prefixed](./okp4-cognitarium-querymsg-definitions-iri-oneof-prefixed.md "An IRI prefixed with a prefix") – `undefined#/query/definitions/IRI/oneOf/0`

*   [Prefixed](./okp4-cognitarium-responses-selectresponse-definitions-iri-oneof-prefixed.md "An IRI prefixed with a prefix") – `undefined#/responses/select/definitions/IRI/oneOf/0`

*   [ProgramResponse](./okp4-law-stone-responses-programresponse.md "ProgramResponse carry elements to locate the program in a okp4-objectarium contract") – `undefined#/responses/program`

*   [Results](./okp4-cognitarium-responses-selectresponse-definitions-results.md "Represents the results of a \[SelectResponse]") – `undefined#/responses/select/definitions/Results`

*   [Select](./okp4-cognitarium-querymsg-oneof-select.md "Returns the resources matching the criteria defined by the provided query") – `undefined#/query/oneOf/1`

*   [SelectQuery](./okp4-cognitarium-querymsg-definitions-selectquery.md "Represents a SELECT query over the triple store, allowing to select variables to return and to filter the results") – `undefined#/query/definitions/SelectQuery`

*   [SelectResponse](./okp4-cognitarium-responses-selectresponse.md "Represents the response of a \[QueryMsg::Select] query") – `undefined#/responses/select`

*   [Simple](./okp4-cognitarium-executemsg-definitions-literal-oneof-simple.md "A simple literal without datatype or language form") – `undefined#/execute/definitions/Literal/oneOf/0`

*   [Simple](./okp4-cognitarium-executemsg-definitions-wherecondition-oneof-simple.md "Represents a simple condition") – `undefined#/execute/definitions/WhereCondition/oneOf/0`

*   [Simple](./okp4-cognitarium-querymsg-definitions-literal-oneof-simple.md "A simple literal without datatype or language form") – `undefined#/query/definitions/Literal/oneOf/0`

*   [Simple](./okp4-cognitarium-querymsg-definitions-wherecondition-oneof-simple.md "Represents a simple condition") – `undefined#/query/definitions/WhereCondition/oneOf/0`

*   [StoreLimits](./okp4-cognitarium-responses-storeresponse-definitions-storelimits.md "Contains limitations regarding store usages") – `undefined#/responses/store/definitions/StoreLimits`

*   [StoreLimitsInput](./okp4-cognitarium-instantiatemsg-definitions-storelimitsinput.md "Contains requested limitations regarding store usages") – `undefined#/instantiate/definitions/StoreLimitsInput`

*   [StoreObject](./okp4-objectarium-executemsg-oneof-storeobject.md "StoreObject store an object to the bucket and make the sender the owner of the object") – `undefined#/execute/oneOf/0`

*   [StoreResponse](./okp4-cognitarium-responses-storeresponse.md "Contains information related to triple store") – `undefined#/responses/store`

*   [StoreStat](./okp4-cognitarium-responses-storeresponse-definitions-storestat.md "Contains usage information about the triple store") – `undefined#/responses/store/definitions/StoreStat`

*   [TriplePattern](./okp4-cognitarium-executemsg-definitions-simplewherecondition-oneof-triplepattern.md "Represents a triple pattern, i") – `undefined#/execute/definitions/SimpleWhereCondition/oneOf/0`

*   [TriplePattern](./okp4-cognitarium-executemsg-definitions-triplepattern.md "Represents a triple pattern in a \[SimpleWhereCondition]") – `undefined#/execute/definitions/TriplePattern`

*   [TriplePattern](./okp4-cognitarium-querymsg-definitions-simplewherecondition-oneof-triplepattern.md "Represents a triple pattern, i") – `undefined#/query/definitions/SimpleWhereCondition/oneOf/0`

*   [TriplePattern](./okp4-cognitarium-querymsg-definitions-triplepattern.md "Represents a triple pattern in a \[SimpleWhereCondition]") – `undefined#/query/definitions/TriplePattern`

*   [TypedValue](./okp4-cognitarium-executemsg-definitions-literal-oneof-typedvalue.md "A value with a datatype") – `undefined#/execute/definitions/Literal/oneOf/2`

*   [TypedValue](./okp4-cognitarium-querymsg-definitions-literal-oneof-typedvalue.md "A value with a datatype") – `undefined#/query/definitions/Literal/oneOf/2`

*   [URI](./okp4-cognitarium-responses-selectresponse-definitions-value-oneof-uri.md "Represents an IRI") – `undefined#/responses/select/definitions/Value/oneOf/0`

*   [UnpinObject](./okp4-objectarium-executemsg-oneof-unpinobject.md "UnpinObject unpins the object in the bucket for the considered sender") – `undefined#/execute/oneOf/3`

*   [Untitled object in okp4-cognitarium](./okp4-cognitarium-executemsg-oneof-insertdata-properties-insert_data.md) – `undefined#/execute/oneOf/0/properties/insert_data`

*   [Untitled object in okp4-cognitarium](./okp4-cognitarium-executemsg-oneof-deletedata-properties-delete_data.md) – `undefined#/execute/oneOf/1/properties/delete_data`

*   [Untitled object in okp4-cognitarium](./okp4-cognitarium-executemsg-definitions-literal-oneof-languagetaggedstring-properties-language_tagged_string.md) – `undefined#/execute/definitions/Literal/oneOf/1/properties/language_tagged_string`

*   [Untitled object in okp4-cognitarium](./okp4-cognitarium-executemsg-definitions-literal-oneof-typedvalue-properties-typed_value.md) – `undefined#/execute/definitions/Literal/oneOf/2/properties/typed_value`

*   [Untitled object in okp4-cognitarium](./okp4-cognitarium-querymsg-oneof-select-properties-select.md) – `undefined#/query/oneOf/1/properties/select`

*   [Untitled object in okp4-cognitarium](./okp4-cognitarium-querymsg-oneof-describe-properties-describe.md) – `undefined#/query/oneOf/2/properties/describe`

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

*   [Variable](./okp4-cognitarium-executemsg-definitions-varornode-oneof-variable.md "A variable") – `undefined#/execute/definitions/VarOrNode/oneOf/0`

*   [Variable](./okp4-cognitarium-executemsg-definitions-varornodeorliteral-oneof-variable.md "A variable") – `undefined#/execute/definitions/VarOrNodeOrLiteral/oneOf/0`

*   [Variable](./okp4-cognitarium-querymsg-definitions-selectitem-oneof-variable.md "Represents a variable") – `undefined#/query/definitions/SelectItem/oneOf/0`

*   [Variable](./okp4-cognitarium-querymsg-definitions-varornode-oneof-variable.md "A variable") – `undefined#/query/definitions/VarOrNode/oneOf/0`

*   [Variable](./okp4-cognitarium-querymsg-definitions-varornodeorliteral-oneof-variable.md "A variable") – `undefined#/query/definitions/VarOrNodeOrLiteral/oneOf/0`

### Arrays

*   [Untitled array in okp4-cognitarium](./okp4-cognitarium-executemsg-oneof-deletedata-properties-delete_data-properties-delete.md "The items to delete") – `undefined#/execute/oneOf/1/properties/delete_data/properties/delete`

*   [Untitled array in okp4-cognitarium](./okp4-cognitarium-executemsg-oneof-deletedata-properties-delete_data-properties-prefixes.md "The prefixes used in the operation") – `undefined#/execute/oneOf/1/properties/delete_data/properties/prefixes`

*   [Untitled array in okp4-cognitarium](./okp4-cognitarium-querymsg-definitions-describequery-properties-prefixes.md "The prefixes used in the query") – `undefined#/query/definitions/DescribeQuery/properties/prefixes`

*   [Untitled array in okp4-cognitarium](./okp4-cognitarium-querymsg-definitions-describequery-properties-where.md "The WHERE clause") – `undefined#/query/definitions/DescribeQuery/properties/where`

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
