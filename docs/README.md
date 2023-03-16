# README

## Top-level Schemas

*   [cw-law-stone](./cw-law-stone.md "CW Law StoneOverviewThe cw-law-stone smart contract aims to provide GaaS (i") – `-`

*   [cw-logic-sample](./cw-logic-sample.md "CW Logic SampleSample contract to query the OKP4 logic module") – `-`

*   [cw-storage](./cw-storage.md "CW StorageOverviewThe cw-storage smart contract enables the storage of arbitrary objects in any Cosmos blockchains using the CosmWasm framework") – `-`

*   [cw-template](./cw-template.md "CW TemplateBase smart contract to start coding into the blockchain 🚀") – `-`

## Other Schemas

### Objects

*   [Ask](./cw-law-stone-querymsg-oneof-ask.md "If not broken, ask the logic module the provided query with the law program loaded") – `undefined#/query/oneOf/0`

*   [Ask](./cw-logic-sample-querymsg-oneof-ask.md "Ask returns the evaluation of the query using the program context through the logic module") – `undefined#/query/oneOf/0`

*   [AskResponse](./cw-law-stone-responses-askresponse.md) – `undefined#/responses/ask`

*   [AskResponse](./cw-logic-sample-responses-askresponse.md) – `undefined#/responses/ask`

*   [Bucket](./cw-storage-querymsg-oneof-bucket.md "Bucket returns the bucket information") – `undefined#/query/oneOf/0`

*   [BucketResponse](./cw-storage-responses-bucketresponse.md "BucketResponse is the response of the Bucket query") – `undefined#/responses/bucket`

*   [ForgetObject](./cw-storage-executemsg-oneof-forgetobject.md "ForgetObject first unpin the object from the bucket for the considered sender, then remove it from the storage if it is not pinned anymore") – `undefined#/execute/oneOf/1`

*   [GetCount](./cw-template-querymsg-oneof-getcount.md "GetCount returns the current count as a json-encoded number") – `undefined#/query/oneOf/0`

*   [GetCountResponse](./cw-template-responses-getcountresponse.md "We define a custom struct for each query response") – `undefined#/responses/get_count`

*   [Increment](./cw-template-executemsg-oneof-increment.md "Execute an increment message") – `undefined#/execute/oneOf/0`

*   [InstantiateMsg](./cw-law-stone-instantiatemsg.md "Instantiate message") – `undefined#/instantiate`

*   [InstantiateMsg](./cw-logic-sample-instantiatemsg.md "Instantiate messages") – `undefined#/instantiate`

*   [InstantiateMsg](./cw-storage-instantiatemsg.md "Instantiate messages") – `undefined#/instantiate`

*   [InstantiateMsg](./cw-template-instantiatemsg.md "Instantiate messages") – `undefined#/instantiate`

*   [Object](./cw-storage-querymsg-oneof-object.md "Object returns the object information with the given id") – `undefined#/query/oneOf/1`

*   [ObjectData](./cw-storage-querymsg-oneof-objectdata.md "ObjectData returns the content of the object with the given id") – `undefined#/query/oneOf/3`

*   [ObjectPins](./cw-storage-querymsg-oneof-objectpins.md "ObjectPins returns the list of addresses that pinned the object with the given id with support for pagination") – `undefined#/query/oneOf/4`

*   [ObjectPinsResponse](./cw-storage-responses-objectpinsresponse.md "ObjectPinsResponse is the response of the GetObjectPins query") – `undefined#/responses/object_pins`

*   [ObjectResponse](./cw-storage-responses-objectresponse.md "ObjectResponse is the response of the Object query") – `undefined#/responses/object`

*   [ObjectResponse](./cw-storage-responses-objectsresponse-definitions-objectresponse.md "ObjectResponse is the response of the Object query") – `undefined#/responses/objects/definitions/ObjectResponse`

*   [Objects](./cw-storage-querymsg-oneof-objects.md "Objects returns the list of objects in the bucket with support for pagination") – `undefined#/query/oneOf/2`

*   [ObjectsResponse](./cw-storage-responses-objectsresponse.md "ObjectsResponse is the response of the Objects query") – `undefined#/responses/objects`

*   [PageInfo](./cw-storage-responses-objectpinsresponse-definitions-pageinfo.md "PageInfo is the page information returned for paginated queries") – `undefined#/responses/object_pins/definitions/PageInfo`

*   [PageInfo](./cw-storage-responses-objectsresponse-definitions-pageinfo.md "PageInfo is the page information returned for paginated queries") – `undefined#/responses/objects/definitions/PageInfo`

*   [PinObject](./cw-storage-executemsg-oneof-pinobject.md "PinObject pins the object in the bucket for the considered sender") – `undefined#/execute/oneOf/2`

*   [ProgramResponse](./cw-law-stone-responses-programresponse.md "ProgramResponse carry elements to locate the program in a cw-storage contract") – `undefined#/responses/program`

*   [Reset](./cw-template-executemsg-oneof-reset.md "Reset counter to the specified value") – `undefined#/execute/oneOf/1`

*   [StoreObject](./cw-storage-executemsg-oneof-storeobject.md "StoreObject store an object to the bucket and make the sender the owner of the object") – `undefined#/execute/oneOf/0`

*   [UnpinObject](./cw-storage-executemsg-oneof-unpinobject.md "UnpinObject unpins the object in the bucket for the considered sender") – `undefined#/execute/oneOf/3`

*   [Untitled object in cw-law-stone](./cw-law-stone-querymsg-oneof-ask-properties-ask.md) – `undefined#/query/oneOf/0/properties/ask`

*   [Untitled object in cw-law-stone](./cw-law-stone-responses-askresponse-definitions-answer.md) – `undefined#/responses/ask/definitions/Answer`

*   [Untitled object in cw-law-stone](./cw-law-stone-responses-askresponse-definitions-result.md) – `undefined#/responses/ask/definitions/Result`

*   [Untitled object in cw-law-stone](./cw-law-stone-responses-askresponse-definitions-substitution.md) – `undefined#/responses/ask/definitions/Substitution`

*   [Untitled object in cw-law-stone](./cw-law-stone-responses-askresponse-definitions-term.md) – `undefined#/responses/ask/definitions/Term`

*   [Untitled object in cw-logic-sample](./cw-logic-sample-querymsg-oneof-ask-properties-ask.md) – `undefined#/query/oneOf/0/properties/ask`

*   [Untitled object in cw-logic-sample](./cw-logic-sample-responses-askresponse-definitions-answer.md) – `undefined#/responses/ask/definitions/Answer`

*   [Untitled object in cw-logic-sample](./cw-logic-sample-responses-askresponse-definitions-result.md) – `undefined#/responses/ask/definitions/Result`

*   [Untitled object in cw-logic-sample](./cw-logic-sample-responses-askresponse-definitions-substitution.md) – `undefined#/responses/ask/definitions/Substitution`

*   [Untitled object in cw-logic-sample](./cw-logic-sample-responses-askresponse-definitions-term.md) – `undefined#/responses/ask/definitions/Term`

*   [Untitled object in cw-storage](./cw-storage-instantiatemsg-definitions-bucketlimits.md "BucketLimits is the type of the limits of a bucket") – `undefined#/instantiate/definitions/BucketLimits`

*   [Untitled object in cw-storage](./cw-storage-executemsg-oneof-storeobject-properties-store_object.md) – `undefined#/execute/oneOf/0/properties/store_object`

*   [Untitled object in cw-storage](./cw-storage-executemsg-oneof-forgetobject-properties-forget_object.md) – `undefined#/execute/oneOf/1/properties/forget_object`

*   [Untitled object in cw-storage](./cw-storage-executemsg-oneof-pinobject-properties-pin_object.md) – `undefined#/execute/oneOf/2/properties/pin_object`

*   [Untitled object in cw-storage](./cw-storage-executemsg-oneof-unpinobject-properties-unpin_object.md) – `undefined#/execute/oneOf/3/properties/unpin_object`

*   [Untitled object in cw-storage](./cw-storage-querymsg-oneof-bucket-properties-bucket.md) – `undefined#/query/oneOf/0/properties/bucket`

*   [Untitled object in cw-storage](./cw-storage-querymsg-oneof-object-properties-object.md) – `undefined#/query/oneOf/1/properties/object`

*   [Untitled object in cw-storage](./cw-storage-querymsg-oneof-objects-properties-objects.md) – `undefined#/query/oneOf/2/properties/objects`

*   [Untitled object in cw-storage](./cw-storage-querymsg-oneof-objectdata-properties-object_data.md) – `undefined#/query/oneOf/3/properties/object_data`

*   [Untitled object in cw-storage](./cw-storage-querymsg-oneof-objectpins-properties-object_pins.md) – `undefined#/query/oneOf/4/properties/object_pins`

*   [Untitled object in cw-storage](./cw-storage-responses-bucketresponse-definitions-bucketlimits.md "BucketLimits is the type of the limits of a bucket") – `undefined#/responses/bucket/definitions/BucketLimits`

*   [Untitled object in cw-template](./cw-template-executemsg-oneof-increment-properties-increment.md) – `undefined#/execute/oneOf/0/properties/increment`

*   [Untitled object in cw-template](./cw-template-executemsg-oneof-reset-properties-reset.md) – `undefined#/execute/oneOf/1/properties/reset`

*   [Untitled object in cw-template](./cw-template-querymsg-oneof-getcount-properties-get_count.md) – `undefined#/query/oneOf/0/properties/get_count`

### Arrays

*   [Untitled array in cw-law-stone](./cw-law-stone-responses-askresponse-definitions-answer-properties-results.md) – `undefined#/responses/ask/definitions/Answer/properties/results`

*   [Untitled array in cw-law-stone](./cw-law-stone-responses-askresponse-definitions-answer-properties-variables.md) – `undefined#/responses/ask/definitions/Answer/properties/variables`

*   [Untitled array in cw-law-stone](./cw-law-stone-responses-askresponse-definitions-result-properties-substitutions.md) – `undefined#/responses/ask/definitions/Result/properties/substitutions`

*   [Untitled array in cw-law-stone](./cw-law-stone-responses-askresponse-definitions-term-properties-arguments.md) – `undefined#/responses/ask/definitions/Term/properties/arguments`

*   [Untitled array in cw-logic-sample](./cw-logic-sample-responses-askresponse-definitions-answer-properties-results.md) – `undefined#/responses/ask/definitions/Answer/properties/results`

*   [Untitled array in cw-logic-sample](./cw-logic-sample-responses-askresponse-definitions-answer-properties-variables.md) – `undefined#/responses/ask/definitions/Answer/properties/variables`

*   [Untitled array in cw-logic-sample](./cw-logic-sample-responses-askresponse-definitions-result-properties-substitutions.md) – `undefined#/responses/ask/definitions/Result/properties/substitutions`

*   [Untitled array in cw-logic-sample](./cw-logic-sample-responses-askresponse-definitions-term-properties-arguments.md) – `undefined#/responses/ask/definitions/Term/properties/arguments`

*   [Untitled array in cw-storage](./cw-storage-responses-objectpinsresponse-properties-data.md "The list of addresses that pinned the object") – `undefined#/responses/object_pins/properties/data`

*   [Untitled array in cw-storage](./cw-storage-responses-objectsresponse-properties-data.md "The list of objects in the bucket") – `undefined#/responses/objects/properties/data`

## Version Note

The schemas linked above follow the JSON Schema Spec version: `http://json-schema.org/draft-07/schema#`
