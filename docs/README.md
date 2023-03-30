# README

## Top-level Schemas

*   [okp4-law-stone](./okp4-law-stone.md "Law StoneOverviewThe okp4-law-stone smart contract aims to provide GaaS (i") – `-`

*   [okp4-objectarium](./okp4-objectarium.md "ObjectariumOverviewThe okp4-objectarium smart contract enables the storage of arbitrary objects in any Cosmos blockchains using the CosmWasm framework") – `-`

## Other Schemas

### Objects

*   [Ask](./okp4-law-stone-querymsg-oneof-ask.md "If not broken, ask the logic module the provided query with the law program loaded") – `undefined#/query/oneOf/0`

*   [AskResponse](./okp4-law-stone-responses-askresponse.md) – `undefined#/responses/ask`

*   [Bucket](./okp4-objectarium-querymsg-oneof-bucket.md "Bucket returns the bucket information") – `undefined#/query/oneOf/0`

*   [BucketResponse](./okp4-objectarium-responses-bucketresponse.md "BucketResponse is the response of the Bucket query") – `undefined#/responses/bucket`

*   [ForgetObject](./okp4-objectarium-executemsg-oneof-forgetobject.md "ForgetObject first unpin the object from the bucket for the considered sender, then remove it from the storage if it is not pinned anymore") – `undefined#/execute/oneOf/1`

*   [InstantiateMsg](./okp4-law-stone-instantiatemsg.md "Instantiate message") – `undefined#/instantiate`

*   [InstantiateMsg](./okp4-objectarium-instantiatemsg.md "Instantiate messages") – `undefined#/instantiate`

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

*   [StoreObject](./okp4-objectarium-executemsg-oneof-storeobject.md "StoreObject store an object to the bucket and make the sender the owner of the object") – `undefined#/execute/oneOf/0`

*   [UnpinObject](./okp4-objectarium-executemsg-oneof-unpinobject.md "UnpinObject unpins the object in the bucket for the considered sender") – `undefined#/execute/oneOf/3`

*   [Untitled object in okp4-law-stone](./okp4-law-stone-querymsg-oneof-ask-properties-ask.md) – `undefined#/query/oneOf/0/properties/ask`

*   [Untitled object in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-answer.md) – `undefined#/responses/ask/definitions/Answer`

*   [Untitled object in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-result.md) – `undefined#/responses/ask/definitions/Result`

*   [Untitled object in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-substitution.md) – `undefined#/responses/ask/definitions/Substitution`

*   [Untitled object in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-term.md) – `undefined#/responses/ask/definitions/Term`

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

*   [Untitled object in okp4-objectarium](./okp4-objectarium-responses-bucketresponse-definitions-bucketlimits.md "BucketLimits is the type of the limits of a bucket") – `undefined#/responses/bucket/definitions/BucketLimits`

*   [Untitled object in okp4-objectarium](./okp4-objectarium-responses-bucketresponse-definitions-paginationconfig.md "PaginationConfig is the type carrying configuration for paginated queries") – `undefined#/responses/bucket/definitions/PaginationConfig`

### Arrays

*   [Untitled array in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-answer-properties-results.md) – `undefined#/responses/ask/definitions/Answer/properties/results`

*   [Untitled array in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-answer-properties-variables.md) – `undefined#/responses/ask/definitions/Answer/properties/variables`

*   [Untitled array in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-result-properties-substitutions.md) – `undefined#/responses/ask/definitions/Result/properties/substitutions`

*   [Untitled array in okp4-law-stone](./okp4-law-stone-responses-askresponse-definitions-term-properties-arguments.md) – `undefined#/responses/ask/definitions/Term/properties/arguments`

*   [Untitled array in okp4-objectarium](./okp4-objectarium-responses-objectpinsresponse-properties-data.md "The list of addresses that pinned the object") – `undefined#/responses/object_pins/properties/data`

*   [Untitled array in okp4-objectarium](./okp4-objectarium-responses-objectsresponse-properties-data.md "The list of objects in the bucket") – `undefined#/responses/objects/properties/data`

## Version Note

The schemas linked above follow the JSON Schema Spec version: `http://json-schema.org/draft-07/schema#`
