# README

## Top-level Schemas

*   [okp4-law-stone](./okp4-law-stone.md "Law StoneOverviewThe okp4-law-stone smart contract aims to provide GaaS (i") – `-`

*   [okp4-objectarium](./okp4-objectarium.md "ObjectariumOverviewThe okp4-objectarium smart contract enables the storage of arbitrary objects in any Cosmos blockchains using the CosmWasm framework") – `-`

*   [okp4-triplestore](./okp4-triplestore.md "TriplestoreOverviewThe okp4-triplestore smart contract enables the storage of RDF graphs triples (i") – `-`

## Other Schemas

### Objects

*   [Ask](./okp4-law-stone-querymsg-oneof-ask.md "If not broken, ask the logic module the provided query with the law program loaded") – `undefined#/query/oneOf/0`

*   [AskResponse](./okp4-law-stone-responses-askresponse.md) – `undefined#/responses/ask`

*   [Bucket](./okp4-objectarium-querymsg-oneof-bucket.md "Bucket returns the bucket information") – `undefined#/query/oneOf/0`

*   [BucketResponse](./okp4-objectarium-responses-bucketresponse.md "BucketResponse is the response of the Bucket query") – `undefined#/responses/bucket`

*   [ForgetObject](./okp4-objectarium-executemsg-oneof-forgetobject.md "ForgetObject first unpin the object from the bucket for the considered sender, then remove it from the storage if it is not pinned anymore") – `undefined#/execute/oneOf/1`

*   [Insert](./okp4-triplestore-executemsg-oneof-insert.md "Insert the Tuples extracted from the provided RDF graph") – `undefined#/execute/oneOf/0`

*   [InstantiateMsg](./okp4-law-stone-instantiatemsg.md "Instantiate message") – `undefined#/instantiate`

*   [InstantiateMsg](./okp4-objectarium-instantiatemsg.md "Instantiate messages") – `undefined#/instantiate`

*   [InstantiateMsg](./okp4-triplestore-instantiatemsg.md "Instantiate message") – `undefined#/instantiate`

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

*   [ProgramResponse](./okp4-law-stone-responses-programresponse.md "ProgramResponse carry elements to locate the program in a okp4-objectarium contract") – `undefined#/responses/program`

*   [Remove](./okp4-triplestore-executemsg-oneof-remove.md "Remove all the Tuples linked to the resources matching the criteria defined in the provided queries") – `undefined#/execute/oneOf/1`

*   [ResourceQuery](./okp4-triplestore-executemsg-definitions-resourcequery.md "A named query targeting resources") – `undefined#/execute/definitions/ResourceQuery`

*   [ResourceQuery](./okp4-triplestore-querymsg-definitions-resourcequery.md "A named query targeting resources") – `undefined#/query/definitions/ResourceQuery`

*   [Resources](./okp4-triplestore-querymsg-oneof-resources.md "Returns the resources matching the criteria defined in the provided queries formatted according to the provided format") – `undefined#/query/oneOf/0`

*   [StoreLimits](./okp4-triplestore-instantiatemsg-definitions-storelimits.md "Contains limitations regarding store usages") – `undefined#/instantiate/definitions/StoreLimits`

*   [StoreObject](./okp4-objectarium-executemsg-oneof-storeobject.md "StoreObject store an object to the bucket and make the sender the owner of the object") – `undefined#/execute/oneOf/0`

*   [UnpinObject](./okp4-objectarium-executemsg-oneof-unpinobject.md "UnpinObject unpins the object in the bucket for the considered sender") – `undefined#/execute/oneOf/3`

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

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-oneof-insert-properties-insert.md) – `undefined#/execute/oneOf/0/properties/insert`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-oneof-remove-properties-remove.md) – `undefined#/execute/oneOf/1/properties/remove`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-graphinput-oneof-0.md "Input in RDF/XML format") – `undefined#/execute/definitions/GraphInput/oneOf/0`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-graphinput-oneof-1.md "Input in Turtle format with support of the Turtle star syntax") – `undefined#/execute/definitions/GraphInput/oneOf/1`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-graphinput-oneof-2.md "Input in N-Triples format with support of the N-Triples star syntax") – `undefined#/execute/definitions/GraphInput/oneOf/2`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-literal-oneof-0.md "A simple string literal value") – `undefined#/execute/definitions/Literal/oneOf/0`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-literal-oneof-1.md "An internationalized string value") – `undefined#/execute/definitions/Literal/oneOf/1`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-literal-oneof-1-properties-i18_n_value.md) – `undefined#/execute/definitions/Literal/oneOf/1/properties/i18_n_value`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-literal-oneof-2.md "A typed value") – `undefined#/execute/definitions/Literal/oneOf/2`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-literal-oneof-2-properties-typed.md) – `undefined#/execute/definitions/Literal/oneOf/2/properties/typed`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-objectvalue-oneof-0.md "A literal value") – `undefined#/execute/definitions/ObjectValue/oneOf/0`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-objectvalue-oneof-1.md "A node to another resource") – `undefined#/execute/definitions/ObjectValue/oneOf/1`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-0.md "Subject match a resource containing the provided node as subject") – `undefined#/execute/definitions/ResourceCriteria/oneOf/0`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-1.md "Property match a resource matching the pair of (predicate, object)") – `undefined#/execute/definitions/ResourceCriteria/oneOf/1`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-1-properties-property.md) – `undefined#/execute/definitions/ResourceCriteria/oneOf/1/properties/property`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-2.md "Referenced match a resource whose subject is referenced in another resource") – `undefined#/execute/definitions/ResourceCriteria/oneOf/2`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-resourcecriteria-oneof-2-properties-referenced.md) – `undefined#/execute/definitions/ResourceCriteria/oneOf/2/properties/referenced`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-valueorjoin-oneof-0.md "A static value") – `undefined#/execute/definitions/ValueOrJoin_for_ObjectValue/oneOf/0`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-valueorjoin-oneof-1.md "A reference to another \[ResourceQuery], identified by its name") – `undefined#/execute/definitions/ValueOrJoin_for_ObjectValue/oneOf/1`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-valueorjoin-1-oneof-0.md "A static value") – `undefined#/execute/definitions/ValueOrJoin_for_String/oneOf/0`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-executemsg-definitions-valueorjoin-1-oneof-1.md "A reference to another \[ResourceQuery], identified by its name") – `undefined#/execute/definitions/ValueOrJoin_for_String/oneOf/1`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-oneof-resources-properties-resources.md) – `undefined#/query/oneOf/0/properties/resources`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-literal-oneof-0.md "A simple string literal value") – `undefined#/query/definitions/Literal/oneOf/0`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-literal-oneof-1.md "An internationalized string value") – `undefined#/query/definitions/Literal/oneOf/1`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-literal-oneof-1-properties-i18_n_value.md) – `undefined#/query/definitions/Literal/oneOf/1/properties/i18_n_value`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-literal-oneof-2.md "A typed value") – `undefined#/query/definitions/Literal/oneOf/2`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-literal-oneof-2-properties-typed.md) – `undefined#/query/definitions/Literal/oneOf/2/properties/typed`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-objectvalue-oneof-0.md "A literal value") – `undefined#/query/definitions/ObjectValue/oneOf/0`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-objectvalue-oneof-1.md "A node to another resource") – `undefined#/query/definitions/ObjectValue/oneOf/1`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-resourcecriteria-oneof-0.md "Subject match a resource containing the provided node as subject") – `undefined#/query/definitions/ResourceCriteria/oneOf/0`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-resourcecriteria-oneof-1.md "Property match a resource matching the pair of (predicate, object)") – `undefined#/query/definitions/ResourceCriteria/oneOf/1`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-resourcecriteria-oneof-1-properties-property.md) – `undefined#/query/definitions/ResourceCriteria/oneOf/1/properties/property`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-resourcecriteria-oneof-2.md "Referenced match a resource whose subject is referenced in another resource") – `undefined#/query/definitions/ResourceCriteria/oneOf/2`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-resourcecriteria-oneof-2-properties-referenced.md) – `undefined#/query/definitions/ResourceCriteria/oneOf/2/properties/referenced`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-valueorjoin-oneof-0.md "A static value") – `undefined#/query/definitions/ValueOrJoin_for_ObjectValue/oneOf/0`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-valueorjoin-oneof-1.md "A reference to another \[ResourceQuery], identified by its name") – `undefined#/query/definitions/ValueOrJoin_for_ObjectValue/oneOf/1`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-valueorjoin-1-oneof-0.md "A static value") – `undefined#/query/definitions/ValueOrJoin_for_String/oneOf/0`

*   [Untitled object in okp4-triplestore](./okp4-triplestore-querymsg-definitions-valueorjoin-1-oneof-1.md "A reference to another \[ResourceQuery], identified by its name") – `undefined#/query/definitions/ValueOrJoin_for_String/oneOf/1`

### Arrays

*   [Untitled array in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-answer-properties-results.md) – `undefined#/responses/ask/definitions/Answer/properties/results`

*   [Untitled array in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-answer-properties-variables.md) – `undefined#/responses/ask/definitions/Answer/properties/variables`

*   [Untitled array in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-result-properties-substitutions.md) – `undefined#/responses/ask/definitions/Result/properties/substitutions`

*   [Untitled array in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-term-properties-arguments.md) – `undefined#/responses/ask/definitions/Term/properties/arguments`

*   [Untitled array in okp4-objectarium](./okp4-objectarium-responses-objectpinsresponse-properties-data.md "The list of addresses that pinned the object") – `undefined#/responses/object_pins/properties/data`

*   [Untitled array in okp4-objectarium](./okp4-objectarium-responses-objectsresponse-properties-data.md "The list of objects in the bucket") – `undefined#/responses/objects/properties/data`

*   [Untitled array in okp4-triplestore](./okp4-triplestore-executemsg-oneof-remove-properties-remove-properties-queries.md "The queries act as the logical disjunction of each single query, a resource shall match at least one query") – `undefined#/execute/oneOf/1/properties/remove/properties/queries`

*   [Untitled array in okp4-triplestore](./okp4-triplestore-executemsg-definitions-resourcequery-properties-criteria.md "The set of criteria a resource must meet to validate the query, it act as the logical conjunction of all the criteria") – `undefined#/execute/definitions/ResourceQuery/properties/criteria`

*   [Untitled array in okp4-triplestore](./okp4-triplestore-querymsg-oneof-resources-properties-resources-properties-queries.md "The queries act as the logical disjunction of each single query, a resource shall match at least one query") – `undefined#/query/oneOf/0/properties/resources/properties/queries`

*   [Untitled array in okp4-triplestore](./okp4-triplestore-querymsg-definitions-resourcequery-properties-criteria.md "The set of criteria a resource must meet to validate the query, it act as the logical conjunction of all the criteria") – `undefined#/query/definitions/ResourceQuery/properties/criteria`

## Version Note

The schemas linked above follow the JSON Schema Spec version: `http://json-schema.org/draft-07/schema#`
