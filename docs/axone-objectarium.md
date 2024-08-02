# Objectarium

A [CosmWasm](https://cosmwasm.com/) Smart Contract which enables the storage of arbitrary unstructured [Objects](https://en.wikipedia.org/wiki/Object_storage) in any [Cosmos](https://cosmos.network/) blockchains.

## Purpose

The smart contract serves as a robust storage solution, allowing for the storage of arbitrary `objects` on any blockchain within the [Cosmos blockchains](https://cosmos.network/) network, utilizing the [CosmWasm](https://cosmwasm.com/) framework. The key features of the contract include:

**Versatile Data Storage:**
The contract is designed to accommodate any type of data, be it text, images, or complex data structures. This flexibility makes it an ideal choice for a wide range of decentralized applications (dApps) that require diverse storage needs.

**On-chain Data:**
By design, the contract stores data on the blockchain, ensuring that it is immutable and publicly accessible. This is particularly useful for applications that require a high level of transparency, and also for any other smart contract that needs to store data on the blockchain.

**Pinning and Unpinning:**
One unique feature is the ability to 'pin' and 'unpin' objects associated with a specific sender address. Pinning ensures that the object remains stored and accessible, while unpinning releases it from being permanently stored, offering a level of control over data persistence.

**Object Removal:**
The contract also includes a 'forget' function, allowing for the removal of objects that are no longer pinned. This is particularly useful for managing storage costs and ensuring that only relevant data remains on the blockchain.

**Cost Management:**
Features like pinning, unpinning, and discarding objects offer a strategic way to control storage costs. Additionally, setting limits on contract size — for instance in terms of object count and their individual sizes — serves as a practical tool to regulate storage costs.

## Rationale

In a sense, we can consider blockchains built on the [Cosmos L0](https://docs.cosmos.network/main) layer as decentralized databases, and their nature can be shaped and modeled through the smart contracts or modules. Given this, it provides a great opportunity to address the wide range of data management needs. One such important area is the management of unstructured, immutable data, which is written once but accessed frequently — commonly known as object storage. This is the primary focus of `axone-objectarium`: a specialized smart contract designed to offer a versatile and efficient approach to handling _on-chain_, _unstructured_, _immutable_ data in a _decentralized_ manner.

## Terminology

### Object

In the context of the `axone-objectarium` smart contract, an `object` refers to a piece of data stored on the blockchain. It can represent various types of information, such as documents, binary files, or any other digital content. Objects are immutable once stored and are identified by their cryptographic hash, which can be generated using algorithms like MD5 or SHA256. This ensures the integrity and security of the stored data, as any modification to the object would result in a different hash value.

### Bucket

The smart contract is organized around buckets. A bucket represents a logical container within the `axone-objectarium` smart contract instance that groups related Objects together. It acts as a storage unit for Objects and provides a context for managing and organizing them. Each bucket has a unique name and is associated with a set of configurations and limits that define its behaviour and characteristics.

### Pin

Pin refers to a mechanism that allows users to mark or "pin" specific objects within a bucket. Pinning an object serves as a way to ensure that the object remains in storage and cannot be removed (this is called "forgotten"). It provides protection and guarantees that the pinned object will persist in the protocol. When an object is pinned, it is associated with the identity (or sender) that performed the pinning action.

## Usage

The unstructured nature of the data stored in the chain opens up a plethora of possibilities for decentralized applications that require this type of versatile storage.

### In the AXONE protocol

The primary function of this smart contract within the AXONE protocol is to enable the persistence of governance rules, which are encoded in Prolog. These programs are stored in an immutable format within the protocol and can be referenced by their unique identifiers in situations where there is a need to refer to these rules.

### In the wild world

A plethora of possibilities opens up for decentralized applications (dApps) that require this kind of versatile storage. However, it's important to consider the following constraints: the data is immutable, the cost of recording the data is proportional to its size, and the data is publicly accessible.

## Play

### Instantiation

The `axone-objectarium` can be instantiated as follows, refer to the schema for more information on configuration, limits and pagination configuration:

```bash
axoned tx wasm instantiate $CODE_ID \
    --label "my-storage" \
    --from $ADDR \
    --admin $ADMIN_ADDR \
    --gas 1000000 \
    '{"bucket":"my-bucket"}'
```

### Execution

We can store an object by providing its data in base64 encoded, we can pin the stored object to prevent it from being removed:

```bash
axoned tx wasm execute $CONTRACT_ADDR \
    --from $ADDR \
    --gas 1000000 \
    "{\"store_object\":{\"data\": \"$(cat my-data | base64)\",\"pin\":true}}"
```

The object id is stable as it is a hash, we can't store an object twice.

With the following commands we can pin and unpin existing objects:

```bash
axoned tx wasm execute $CONTRACT_ADDR \
    --from $ADDR \
    --gas 1000000 \
    "{\"pin_object\":{\"id\": \"$OBJECT_ID\"}}"

axoned tx wasm execute $CONTRACT_ADDR \
    --from $ADDR \
    --gas 1000000 \
    "{\"unpin_object\":{\"id\": \"$OBJECT_ID\"}}"
```

And if an object is not pinned, or pinned by the sender of transaction, we can remove it:

```bash
axoned tx wasm execute $CONTRACT_ADDR \
    --from $ADDR \
    --gas 1000000 \
    "{\"forget_object\":{\"id\": \"$OBJECT_ID\"}}"
```

### Querying

Query an object by its id:

```bash
axoned query wasm contract-state smart $CONTRACT_ADDR \
    "{\"object\": {\"id\": \"$OBJECT_ID\"}}"
```

Or its data:

```bash
axoned query wasm contract-state smart $CONTRACT_ADDR \
    "{\"object_data\": {\"id\": \"$OBJECT_ID\"}}"
```

We can also list the objects, eventually filtering on the object owner:

```bash
axoned query wasm contract-state smart $CONTRACT_ADDR \
    "{\"objects\": {\"address\": \"axone1p8u47en82gmzfm259y6z93r9qe63l25d858vqu\"}}"
```

And navigate in a cursor based pagination:

```bash
axoned query wasm contract-state smart $CONTRACT_ADDR \
    "{\"objects\": {\"first\": 5, \"after\": \"23Y5t5DBe7DkPwfJo3Sd26Y8Z9epmtpA1FTpdG7DiG6MD8vPRTzzbQ9TccmyoBcePkPK6atUiqcAzJVo3TfYNBGY\"}}"
```

We can also query object pins with the same cursor based pagination:

```bash
axoned query wasm contract-state smart $CONTRACT_ADDR \
    "{\"object_pins\": {\"id\": \"$OBJECT_ID\", \"first\": 5, \"after\": \"23Y5t5DBe7DkPwfJo3Sd26Y8Z9epmtpA1FTpdG7DiG6MD8vPRTzzbQ9TccmyoBcePkPK6atUiqcAzJVo3TfYNBGY\"}}"
```

## InstantiateMsg

Instantiate messages

| parameter                                | description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ---------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bucket`                                 | _(Required.) _ **string**. The name of the bucket. The name could not be empty or contains whitespaces. If name contains whitespace, they will be removed.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `config`                                 | **[BucketConfig](#bucketconfig)**. The configuration of the bucket.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `config.accepted_compression_algorithms` | **Array&lt;[CompressionAlgorithm](#compressionalgorithm)&gt;**. The acceptable compression algorithms for the objects in the bucket. If this parameter is not set, then all compression algorithms are accepted. If this parameter is set, then only the compression algorithms in the array are accepted.<br /><br />When an object is stored in the bucket without a specified compression algorithm, the first algorithm in the array is used. Therefore, the order of the algorithms in the array is significant. Typically, the most efficient compression algorithm, such as the NoCompression algorithm, should be placed first in the array.<br /><br />Any attempt to store an object using a different compression algorithm than the ones specified here will fail.<br />**Default:** `["passthrough","snappy","lzma"]` |
| `config.hash_algorithm`                  | **[HashAlgorithm](#hashalgorithm)**. The algorithm used to hash the content of the objects to generate the id of the objects. The algorithm is optional and if not set, the default algorithm is used.<br /><br />The default algorithm is Sha256 if not set.<br />**Default:** `"sha256"`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `limits`                                 | **[BucketLimits](#bucketlimits)**. The limits of the bucket.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `limits.max_object_pins`                 | **[Uint128](#uint128)\|null**. The maximum number of pins in the bucket for an object.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `limits.max_object_size`                 | **[Uint128](#uint128)\|null**. The maximum size of the objects in the bucket.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `limits.max_objects`                     | **[Uint128](#uint128)\|null**. The maximum number of objects in the bucket.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `limits.max_total_size`                  | **[Uint128](#uint128)\|null**. The maximum total size of the objects in the bucket.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `pagination`                             | **[PaginationConfig](#paginationconfig)**. The configuration for paginated query.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `pagination.default_page_size`           | **integer**. The default number of elements in a page.<br /><br />Shall be less or equal than `max_page_size`. Default to '10' if not set.<br />**Default:** `10`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `pagination.max_page_size`               | **integer**. The maximum elements a page can contain.<br /><br />Shall be less than `u32::MAX - 1`. Default to '30' if not set.<br />**Default:** `30`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

## ExecuteMsg

Execute messages

### ExecuteMsg::StoreObject

StoreObject store an object to the bucket and make the sender the owner of the object. The object is referenced by the hash of its content and this value is returned. If the object is already stored, it is a no-op. It may be pinned though.

The "pin" parameter specifies whether the object should be pinned for the sender. Pinning an object ensures it is protected from being removed from storage, making it persistent and guaranteeing its indefinite accessibility. It’s important to note that pinning is optional; objects can be stored without pinning. However, be aware that non-pinned objects can be removed from the storage by anyone at any time, making them no longer accessible.

The "compression_algorithm" parameter specifies the algorithm for compressing the object before storing it in the storage, which is optional. If no algorithm is specified, the algorithm used is the first algorithm of the bucket configuration limits. Note that the chosen algorithm can save storage space, but it will increase CPU usage. Depending on the chosen compression algorithm and the achieved compression ratio, the gas cost of the operation will vary, either increasing or decreasing.

| parameter                            | description                                                                                                                                                                                                                                                                                     |
| ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `store_object`                       | _(Required.) _ **object**.                                                                                                                                                                                                                                                                      |
| `store_object.compression_algorithm` | **[CompressionAlgorithm](#compressionalgorithm)\|null**. Specifies the compression algorithm to use when storing the object. If None, the first algorithm specified in the list of accepted compression algorithms of the bucket is used (see [BucketLimits::accepted_compression_algorithms]). |
| `store_object.data`                  | _(Required.) _ **[Binary](#binary)**. The content of the object to store.                                                                                                                                                                                                                       |
| `store_object.pin`                   | _(Required.) _ **boolean**. Specifies whether the object should be pinned for the sender. Pinning ensures the object remains persistent and cannot be removed from storage by anyone.                                                                                                           |

### ExecuteMsg::ForgetObject

ForgetObject first unpins the object from the bucket for the sender, then removes it from storage if it is no longer pinned by anyone. If the object is still pinned by other senders, it is not removed from storage and an error is returned. If the object is not pinned for the sender, this operation is a no-op.

| parameter          | description                |
| ------------------ | -------------------------- |
| `forget_object`    | _(Required.) _ **object**. |
| `forget_object.id` | _(Required.) _ **string**. |

### ExecuteMsg::PinObject

PinObject pins the object in the bucket for the sender. If the object is already pinned for the sender, this operation is a no-op. While an object is pinned, it cannot be removed from storage.

| parameter       | description                |
| --------------- | -------------------------- |
| `pin_object`    | _(Required.) _ **object**. |
| `pin_object.id` | _(Required.) _ **string**. |

### ExecuteMsg::UnpinObject

UnpinObject unpins the object in the bucket for the sender. If the object is not pinned for the sender, this operation is a no-op. The object can be removed from storage if it is no longer pinned by anyone.

| parameter         | description                |
| ----------------- | -------------------------- |
| `unpin_object`    | _(Required.) _ **object**. |
| `unpin_object.id` | _(Required.) _ **string**. |

## QueryMsg

Query messages

### QueryMsg::Bucket

Bucket returns the bucket information.

| parameter | description                |
| --------- | -------------------------- |
| `bucket`  | _(Required.) _ **object**. |

### QueryMsg::Object

Object returns the object information with the given id.

| parameter   | description                                             |
| ----------- | ------------------------------------------------------- |
| `object`    | _(Required.) _ **object**.                              |
| `object.id` | _(Required.) _ **string**. The id of the object to get. |

### QueryMsg::Objects

Objects returns the list of objects in the bucket with support for pagination.

| parameter         | description                                                             |
| ----------------- | ----------------------------------------------------------------------- |
| `objects`         | _(Required.) _ **object**.                                              |
| `objects.address` | **string\|null**. The owner of the objects to get.                      |
| `objects.after`   | **string\|null**. The point in the sequence to start returning objects. |
| `objects.first`   | **integer\|null**. The number of objects to return.                     |

### QueryMsg::ObjectData

ObjectData returns the content of the object with the given id.

| parameter        | description                                             |
| ---------------- | ------------------------------------------------------- |
| `object_data`    | _(Required.) _ **object**.                              |
| `object_data.id` | _(Required.) _ **string**. The id of the object to get. |

### QueryMsg::ObjectPins

ObjectPins returns the list of addresses that pinned the object with the given id with support for pagination.

| parameter           | description                                                          |
| ------------------- | -------------------------------------------------------------------- |
| `object_pins`       | _(Required.) _ **object**.                                           |
| `object_pins.after` | **string\|null**. The point in the sequence to start returning pins. |
| `object_pins.first` | **integer\|null**. The number of pins to return.                     |
| `object_pins.id`    | _(Required.) _ **string**. The id of the object to get the pins for. |

## Responses

### bucket

BucketResponse is the response of the Bucket query.

| property                                 | description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ---------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `config`                                 | _(Required.) _ **[BucketConfig](#bucketconfig)**. The configuration of the bucket.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `config.accepted_compression_algorithms` | **Array&lt;[CompressionAlgorithm](#compressionalgorithm)&gt;**. The acceptable compression algorithms for the objects in the bucket. If this parameter is not set, then all compression algorithms are accepted. If this parameter is set, then only the compression algorithms in the array are accepted.<br /><br />When an object is stored in the bucket without a specified compression algorithm, the first algorithm in the array is used. Therefore, the order of the algorithms in the array is significant. Typically, the most efficient compression algorithm, such as the NoCompression algorithm, should be placed first in the array.<br /><br />Any attempt to store an object using a different compression algorithm than the ones specified here will fail.<br />**Default:** `["passthrough","snappy","lzma"]` |
| `config.hash_algorithm`                  | **[HashAlgorithm](#hashalgorithm)**. The algorithm used to hash the content of the objects to generate the id of the objects. The algorithm is optional and if not set, the default algorithm is used.<br /><br />The default algorithm is Sha256 if not set.<br />**Default:** `"sha256"`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `limits`                                 | _(Required.) _ **[BucketLimits](#bucketlimits)**. The limits of the bucket.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `limits.max_object_pins`                 | **[Uint128](#uint128)\|null**. The maximum number of pins in the bucket for an object.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `limits.max_object_size`                 | **[Uint128](#uint128)\|null**. The maximum size of the objects in the bucket.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `limits.max_objects`                     | **[Uint128](#uint128)\|null**. The maximum number of objects in the bucket.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `limits.max_total_size`                  | **[Uint128](#uint128)\|null**. The maximum total size of the objects in the bucket.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `name`                                   | _(Required.) _ **string**. The name of the bucket.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `pagination`                             | _(Required.) _ **[PaginationConfig](#paginationconfig)**. The configuration for paginated query.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `pagination.default_page_size`           | **integer**. The default number of elements in a page.<br /><br />Shall be less or equal than `max_page_size`. Default to '10' if not set.<br />**Default:** `10`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `pagination.max_page_size`               | **integer**. The maximum elements a page can contain.<br /><br />Shall be less than `u32::MAX - 1`. Default to '30' if not set.<br />**Default:** `30`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `stat`                                   | _(Required.) _ **[BucketStat](#bucketstat)**. The statistics of the bucket.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `stat.compressed_size`                   | **[Uint128](#uint128)**. The total size of the objects contained in the bucket after compression.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `stat.object_count`                      | **[Uint128](#uint128)**. The number of objects in the bucket.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `stat.size`                              | **[Uint128](#uint128)**. The total size of the objects contained in the bucket.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |

### object

ObjectResponse is the response of the Object query.

| property                | description                                                                                                                                       |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `compressed_size`       | _(Required.) _ **[Uint128](#uint128)**. The size of the object when compressed. If the object is not compressed, the value is the same as `size`. |
| `compression_algorithm` | _(Required.) _ **[CompressionAlgorithm](#compressionalgorithm)**. The compression algorithm used to compress the content of the object.           |
| `id`                    | _(Required.) _ **string**. The id of the object.                                                                                                  |
| `is_pinned`             | _(Required.) _ **boolean**. Tells if the object is pinned by at least one address.                                                                |
| `owner`                 | _(Required.) _ **string**. The owner of the object.                                                                                               |
| `size`                  | _(Required.) _ **[Uint128](#uint128)**. The size of the object.                                                                                   |

### object_data

Binary is a wrapper around Vec&lt;u8&gt; to add base64 de/serialization with serde. It also adds some helper methods to help encode inline.

This is only needed as serde-json-\{core,wasm\} has a horrible encoding for Vec&lt;u8&gt;. See also &lt;https://github.com/CosmWasm/cosmwasm/blob/main/docs/MESSAGE_TYPES.md&gt;.

| type        |
| ----------- |
| **string**. |

### object_pins

ObjectPinsResponse is the response of the GetObjectPins query.

| property                  | description                                                                           |
| ------------------------- | ------------------------------------------------------------------------------------- |
| `data`                    | _(Required.) _ **Array&lt;string&gt;**. The list of addresses that pinned the object. |
| `page_info`               | _(Required.) _ **[PageInfo](#pageinfo)**. The page information.                       |
| `page_info.cursor`        | **string**. The cursor to the next page.                                              |
| `page_info.has_next_page` | **boolean**. Tells if there is a next page.                                           |

### objects

ObjectsResponse is the response of the Objects query.

| property                  | description                                                                                           |
| ------------------------- | ----------------------------------------------------------------------------------------------------- |
| `data`                    | _(Required.) _ **Array&lt;[ObjectResponse](#objectresponse)&gt;**. The list of objects in the bucket. |
| `page_info`               | _(Required.) _ **[PageInfo](#pageinfo)**. The page information.                                       |
| `page_info.cursor`        | **string**. The cursor to the next page.                                                              |
| `page_info.has_next_page` | **boolean**. Tells if there is a next page.                                                           |

## Definitions

### Binary

A string containing Base64-encoded data.

| type        |
| ----------- |
| **string**. |

### BucketConfig

BucketConfig is the type of the configuration of a bucket.

The configuration is set at the instantiation of the bucket, and is immutable and cannot be changed. The configuration is optional and if not set, the default configuration is used.

| property                          | description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `accepted_compression_algorithms` | **Array&lt;[CompressionAlgorithm](#compressionalgorithm)&gt;**. The acceptable compression algorithms for the objects in the bucket. If this parameter is not set, then all compression algorithms are accepted. If this parameter is set, then only the compression algorithms in the array are accepted.<br /><br />When an object is stored in the bucket without a specified compression algorithm, the first algorithm in the array is used. Therefore, the order of the algorithms in the array is significant. Typically, the most efficient compression algorithm, such as the NoCompression algorithm, should be placed first in the array.<br /><br />Any attempt to store an object using a different compression algorithm than the ones specified here will fail. |
| `hash_algorithm`                  | **[HashAlgorithm](#hashalgorithm)**. The algorithm used to hash the content of the objects to generate the id of the objects. The algorithm is optional and if not set, the default algorithm is used.<br /><br />The default algorithm is Sha256 if not set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

### BucketLimits

BucketLimits is the type of the limits of a bucket.

The limits are optional and if not set, there is no limit.

| property          | description                                                                            |
| ----------------- | -------------------------------------------------------------------------------------- |
| `max_object_pins` | **[Uint128](#uint128)\|null**. The maximum number of pins in the bucket for an object. |
| `max_object_size` | **[Uint128](#uint128)\|null**. The maximum size of the objects in the bucket.          |
| `max_objects`     | **[Uint128](#uint128)\|null**. The maximum number of objects in the bucket.            |
| `max_total_size`  | **[Uint128](#uint128)\|null**. The maximum total size of the objects in the bucket.    |

### BucketStat

BucketStat is the type of the statistics of a bucket.

| property          | description                                                                                                      |
| ----------------- | ---------------------------------------------------------------------------------------------------------------- |
| `compressed_size` | _(Required.) _ **[Uint128](#uint128)**. The total size of the objects contained in the bucket after compression. |
| `object_count`    | _(Required.) _ **[Uint128](#uint128)**. The number of objects in the bucket.                                     |
| `size`            | _(Required.) _ **[Uint128](#uint128)**. The total size of the objects contained in the bucket.                   |

### CompressionAlgorithm

CompressionAlgorithm is an enumeration that defines the different compression algorithms supported for compressing the content of objects. The compression algorithm specified here are relevant algorithms for compressing data on-chain, which means that they are fast to compress and decompress, and have a low computational cost.

The order of the compression algorithms is based on their estimated computational cost (quite opinionated) during both compression and decompression, ranging from the lowest to the highest. This particular order is utilized to establish the default compression algorithm for storing an object.

| variant                     | description                                                                                                                                                                                                                                                                                                                                             |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Passthrough](#passthrough) | **string**: `passthrough`. Represents no compression algorithm. The object is stored as is without any compression.                                                                                                                                                                                                                                     |
| [Snappy](#snappy)           | **string**: `snappy`. Represents the Snappy algorithm. Snappy is a compression/decompression algorithm that does not aim for maximum compression. Instead, it aims for very high speeds and reasonable compression.<br /><br />See [the snappy web page](https://google.github.io/snappy/) for more information.                                        |
| [Lzma](#lzma)               | **string**: `lzma`. Represents the LZMA algorithm. LZMA is a lossless data compression/decompression algorithm that features a high compression ratio and a variable compression-dictionary size up to 4 GB.<br /><br />See [the LZMA wiki page](https://en.wikipedia.org/wiki/Lempel%E2%80%93Ziv%E2%80%93Markov_chain_algorithm) for more information. |

### HashAlgorithm

HashAlgorithm is an enumeration that defines the different hash algorithms supported for hashing the content of objects.

| variant           | description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [MD5](#md5)       | **string**: `m_d5`. Represents the MD5 algorithm. MD5 is a widely used cryptographic hash function that produces a 128-bit hash value. The computational cost of MD5 is relatively low compared to other hash functions, but its short hash length makes it easier to find hash collisions. It is now considered insecure for cryptographic purposes, but can still used in non-security contexts.<br /><br />MD5 hashes are stored on-chain as 32 hexadecimal characters.<br /><br />See [the MD5 Wikipedia page](https://en.wikipedia.org/wiki/MD5) for more information. |
| [SHA1](#sha1)     | **string**: `sha224`. Represents the SHA-224 algorithm. SHA-224 is a variant of the SHA-2 family of hash functions that produces a 224-bit hash value. It is similar to SHA-256, but with a shorter output size. The computational cost of SHA-224 is moderate, and its relatively short hash length makes it easier to store and transmit.<br /><br />SHA-224 hashes are stored on-chain as 56 hexadecimal characters.<br /><br />See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.                                                |
| [SHA256](#sha256) | **string**: `sha256`. Represents the SHA-256 algorithm. SHA-256 is a member of the SHA-2 family of hash functions that produces a 256-bit hash value. It is widely used in cryptography and other security-related applications. The computational cost of SHA-256 is moderate, and its hash length strikes a good balance between security and convenience.<br /><br />SHA-256 hashes are stored on-chain as 64 hexadecimal characters.<br /><br />See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.                               |
| [SHA384](#sha384) | **string**: `sha384`. Represents the SHA-384 algorithm. SHA-384 is a variant of the SHA-2 family of hash functions that produces a 384-bit hash value. It is similar to SHA-512, but with a shorter output size. The computational cost of SHA-384 is relatively high, but its longer hash length provides better security against hash collisions.<br /><br />SHA-384 hashes are stored on-chain as 96 hexadecimal characters.<br /><br />See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.                                        |
| [SHA512](#sha512) | **string**: `sha512`. Represents the SHA-512 algorithm. SHA-512 is a member of the SHA-2 family of hash functions that produces a 512-bit hash value. It is widely used in cryptography and other security-related applications. The computational cost of SHA-512 is relatively high, but its longer hash length provides better security against hash collisions.<br /><br />SHA-512 hashes are stored on-chain as 128 hexadecimal characters.<br /><br />See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.                       |

### Lzma

Represents the LZMA algorithm. LZMA is a lossless data compression/decompression algorithm that features a high compression ratio and a variable compression-dictionary size up to 4 GB.

See [the LZMA wiki page](https://en.wikipedia.org/wiki/Lempel%E2%80%93Ziv%E2%80%93Markov_chain_algorithm) for more information.

| literal  |
| -------- |
| `"lzma"` |

### MD5

Represents the MD5 algorithm. MD5 is a widely used cryptographic hash function that produces a 128-bit hash value. The computational cost of MD5 is relatively low compared to other hash functions, but its short hash length makes it easier to find hash collisions. It is now considered insecure for cryptographic purposes, but can still used in non-security contexts.

MD5 hashes are stored on-chain as 32 hexadecimal characters.

See [the MD5 Wikipedia page](https://en.wikipedia.org/wiki/MD5) for more information.

| literal  |
| -------- |
| `"m_d5"` |

### ObjectResponse

ObjectResponse is the response of the Object query.

| property                | description                                                                                                                                       |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `compressed_size`       | _(Required.) _ **[Uint128](#uint128)**. The size of the object when compressed. If the object is not compressed, the value is the same as `size`. |
| `compression_algorithm` | _(Required.) _ **[CompressionAlgorithm](#compressionalgorithm)**. The compression algorithm used to compress the content of the object.           |
| `id`                    | _(Required.) _ **string**. The id of the object.                                                                                                  |
| `is_pinned`             | _(Required.) _ **boolean**. Tells if the object is pinned by at least one address.                                                                |
| `owner`                 | _(Required.) _ **string**. The owner of the object.                                                                                               |
| `size`                  | _(Required.) _ **[Uint128](#uint128)**. The size of the object.                                                                                   |

### PageInfo

PageInfo is the page information returned for paginated queries.

| property        | description                                                |
| --------------- | ---------------------------------------------------------- |
| `cursor`        | _(Required.) _ **string**. The cursor to the next page.    |
| `has_next_page` | _(Required.) _ **boolean**. Tells if there is a next page. |

### PaginationConfig

PaginationConfig is the type carrying configuration for paginated queries.

The fields are optional and if not set, there is a default configuration.

| property            | description                                                                                                                                |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `default_page_size` | **integer**. The default number of elements in a page.<br /><br />Shall be less or equal than `max_page_size`. Default to '10' if not set. |
| `max_page_size`     | **integer**. The maximum elements a page can contain.<br /><br />Shall be less than `u32::MAX - 1`. Default to '30' if not set.            |

### Passthrough

Represents no compression algorithm. The object is stored as is without any compression.

| literal         |
| --------------- |
| `"passthrough"` |

### SHA1

Represents the SHA-224 algorithm. SHA-224 is a variant of the SHA-2 family of hash functions that produces a 224-bit hash value. It is similar to SHA-256, but with a shorter output size. The computational cost of SHA-224 is moderate, and its relatively short hash length makes it easier to store and transmit.

SHA-224 hashes are stored on-chain as 56 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| literal    |
| ---------- |
| `"sha224"` |

### SHA256

Represents the SHA-256 algorithm. SHA-256 is a member of the SHA-2 family of hash functions that produces a 256-bit hash value. It is widely used in cryptography and other security-related applications. The computational cost of SHA-256 is moderate, and its hash length strikes a good balance between security and convenience.

SHA-256 hashes are stored on-chain as 64 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| literal    |
| ---------- |
| `"sha256"` |

### SHA384

Represents the SHA-384 algorithm. SHA-384 is a variant of the SHA-2 family of hash functions that produces a 384-bit hash value. It is similar to SHA-512, but with a shorter output size. The computational cost of SHA-384 is relatively high, but its longer hash length provides better security against hash collisions.

SHA-384 hashes are stored on-chain as 96 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| literal    |
| ---------- |
| `"sha384"` |

### SHA512

Represents the SHA-512 algorithm. SHA-512 is a member of the SHA-2 family of hash functions that produces a 512-bit hash value. It is widely used in cryptography and other security-related applications. The computational cost of SHA-512 is relatively high, but its longer hash length provides better security against hash collisions.

SHA-512 hashes are stored on-chain as 128 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| literal    |
| ---------- |
| `"sha512"` |

### Snappy

Represents the Snappy algorithm. Snappy is a compression/decompression algorithm that does not aim for maximum compression. Instead, it aims for very high speeds and reasonable compression.

See [the snappy web page](https://google.github.io/snappy/) for more information.

| literal    |
| ---------- |
| `"snappy"` |

### Uint128

A string containing a 128-bit integer in decimal representation.

| type        |
| ----------- |
| **string**. |

---

_Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `axone-objectarium.json` (`3a53bb06f9b58538`)_
