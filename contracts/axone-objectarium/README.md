# Objectarium

A  [CosmWasm](https://cosmwasm.com/) Smart Contract which enables the storage of arbitrary unstructured [Objects](https://en.wikipedia.org/wiki/Object_storage) in any [Cosmos](https://cosmos.network/) blockchains.

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

In a sense, we can consider blockchains built on the [Cosmos L0](https://docs.cosmos.network/main) layer as decentralized databases, and their nature can be shaped and modeled through the smart contracts or modules. Given this, it provides a great opportunity to address the wide range of data management needs. One such important area is the management of unstructured, immutable data, which is written once but accessed frequently — commonly known as object storage. This is the primary focus of `axone-objectarium`: a specialized smart contract designed to offer a versatile and efficient approach to handling *on-chain*, *unstructured*, *immutable* data in a *decentralized* manner.

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
