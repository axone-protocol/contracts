# Dataverse

## Overview

The `dataverse` smart contract is responsible for overseeing and managing the dataverse. The Dataverse is an ever-expanding universe that encompasses a wide range of digital resources. These include datasets, data processing algorithms, ML algorithm, storage resources, computational resources, identity management solutions, orchestration engines, oracles, and many other resources recorded on the blockchain.

Within the Dataverse, there are defined Zones where specific rules apply. Digital resources recognized within these Zones are the ones compatible with these rules, considering the associated consents. Hence the smart contract also provides mechanisms to manage these Zones, ensuring the implementation of precise governance rules.

## Instances

When the smart contract is instantiated, it creates a Dataverse instance. This instance is separated and isolated from any pre-existing ones, and as many dataverse instances as required can be created.

## Dependencies

Given its role and status, this smart contract serves as the primary access point for the OKP4 protocol to manage all on-chain stored resources. To fulfill its tasks, the smart contract relies on other smart contracts within the OKP4 ecosystem. Notably, it uses the `Cognitarium` smart contract for persisting the Dataverse representation in an ontological form and the `Law Stone` smart contract to establish governance rules.

## InstantiateMsg

`InstantiateMsg` is used to initialize a new instance of the dataverse.

|parameter|description|
|----------|-----------|
|`name`|*(Required.) * **string**. A name to give to the dataverse instantiated.|

## ExecuteMsg

`ExecuteMsg` defines the set of possible actions that can be performed on the dataverse.

This enum provides variants for registering services, datasets, and other operations related to the dataverse.

### ExecuteMsg::RegisterService

Registers a new service within the dataverse. Service is a generic concept for any kind of service that can be provided through a network (e.g. a REST API, a gRPC service, etc.).

Each service is identified and located by its unique URI which defines the entry point of the service.

#### Examples:

```rust ExecuteMsg::RegisterService { subject:    "https://ontology.okp4.space/dataverse/service/metadata/52549532-887d-409b-a9c0-fb68f9e521d2", identity:   "did:key:z6MkrpCPVDHcsqi3aaqnemLC1aBTUwkfPwTyzc8sFWYwm1PA", identifier: "urn:uuid:803cd033-2eed-4db7-847b-f46715a42a70" } ```

|parameter|description|
|----------|-----------|
|`register_service`|*(Required.) * **object**. |
|`register_service.identifier`|*(Required.) * **string**. The unique URI that identifies and locates the service.<br /><br />The URI serves a dual purpose: 1. **Identification**: It provides a unique identifier for the service, ensuring that each service can be distinctly recognized within the dataverse. 2. **Endpoint**: The URI acts as the access point or endpoint for the service. It specifies where the service can be accessed and how interactions with the service should be initiated.|
|`register_service.identity`|*(Required.) * **string**. The decentralized identity of the service.|
|`register_service.registrar`|**string\|null**. The URI of the entity responsible for registering and managing the service in the dataverse (i.e. on the blockchain). It's an optional field, if not provided the service is registered by the entity that invokes the transaction.|
|`register_service.subject`|*(Required.) * **string**. The unique RDF identifier for the resource representation of the service within the dataverse.|

### ExecuteMsg::RegisterDataset

Registers a new dataset within the dataverse.

A `Dataset` represents a collection of related data that is organized and presented in a specific format by the provider. This data can be in various forms, such as CSV files, images, videos, and more. It can also refer to data sources like databases and APIs.

Each dataset is uniquely identified by its URI, which serves as both the identifier and the access point for the dataset. When accessing a dataset, it's crucial to understand the protocol and methods supported by the dataset's endpoint. For instance, a dataset with an HTTP-based URI might be accessible via GET requests and may require specific headers or parameters for successful retrieval.

#### Examples:

```rust ExecuteMsg::RegisterDataset { subject:     "https://ontology.okp4.space/dataverse/dataset/96a562a9-5feb-4a41-bcf2-cc8610af9f78", identifier:  "ipfs://bafybeicn7i3soqdgr7dwnrwytgq4zxy7a5jpkizrvhm5mv6bgjd32wm3q4", provided_by: "urn:uuid:803cd033-2eed-4db7-847b-f46715a42a70" } ```

|parameter|description|
|----------|-----------|
|`register_dataset`|*(Required.) * **object**. |
|`register_dataset.identifier`|*(Required.) * **string**. The unique URI that identifies the dataset.|
|`register_dataset.provided_by`|*(Required.) * **string**. The URI of the service, already registered in the dataverse, that provides the dataset.|
|`register_dataset.registrar`|**string\|null**. The URI of the entity responsible for registering and managing the dataset in the dataverse (i.e. on the blockchain). It's an optional field, if not provided the dataset is registered by the entity that invokes the transaction.|
|`register_dataset.subject`|*(Required.) * **string**. The unique RDF identifier for the resource representation of the dataset within the dataverse.|

### ExecuteMsg::FoundZone

Founds a new zone within the dataverse.

`Zone` is a conceptual framework that is established based on a set of rules, within which recognized digital Resources must conform, considering associated consents.

#### Example

``` ExecuteMsg::FoundZone { subject:    "https://ontology.okp4.space/dataverse/zone/ef347285-e52a-430d-9679-dcb76b962ce7", identifier: "urn:uuid:6d1aaad8-9411-4758-a9f9-ed43358af1fd" } ```

|parameter|description|
|----------|-----------|
|`found_zone`|*(Required.) * **object**. |
|`found_zone.identifier`|*(Required.) * **string**. The unique URI that identifies the zone.|
|`found_zone.registrar`|**string\|null**. The URI of the entity responsible for registering and managing the zone in the dataverse (i.e. on the blockchain). It's an optional field, if not provided the zone is registered by the entity that invokes the transaction.|
|`found_zone.subject`|*(Required.) * **string**. The unique RDF identifier for the resource representation of the zone within the dataverse.|

### ExecuteMsg::AttachMetadata

Attaches metadata to a specified resource registered in the dataverse.

Metadata provides additional information or details about a resource.

|parameter|description|
|----------|-----------|
|`attach_metadata`|*(Required.) * **object**. |
|`attach_metadata.format`|**[RdfFormat](#rdfformat)\|null**. RDF format in which the metadata is represented. If not provided, the default format is [Turtle](https://www.w3.org/TR/turtle/) format.|
|`attach_metadata.metadata`|*(Required.) * **[Binary](#binary)**. The serialized metadata intended for attachment. This metadata should adhere to the format specified in the `format` field.|
|`attach_metadata.subject`|*(Required.) * **string**. The unique RDF identifier of the resource for which the metadata should be attached.|

### ExecuteMsg::DetachMetadata

Remove a previously associated metadata (from a specific resource within the dataverse). Once removed the metadata is no longer accessible.

|parameter|description|
|----------|-----------|
|`detach_metadata`|*(Required.) * **object**. |
|`detach_metadata.subject`|*(Required.) * **string**. The RDF identifier of the metadata to be removed.|

### ExecuteMsg::ReviseMetadata

Revises a previously associated metadata in order to update it or amend it.

|parameter|description|
|----------|-----------|
|`revise_metadata`|*(Required.) * **object**. |
|`revise_metadata.format`|**[RdfFormat](#rdfformat)\|null**. RDF format in which the metadata is represented. If not provided, the default format is [Turtle](https://www.w3.org/TR/turtle/) format.|
|`revise_metadata.metadata`|*(Required.) * **[Binary](#binary)**. The serialized metadata intended for revision. This metadata should adhere to the format specified in the `format` field.|
|`revise_metadata.subject`|*(Required.) * **string**. The RDF identifier of the metadata to be revised.|

## QueryMsg

`QueryMsg` defines the set of possible queries that can be made to retrieve information about the dataverse.

This enum provides variants for querying the dataverse's details and other related information.

### QueryMsg::Dataverse

Retrieves information about the current dataverse instance.

|parameter|description|
|----------|-----------|
|`dataverse`|*(Required.) * **object**. |

## Responses

### dataverse

DataverseResponse is the response of the Dataverse query.

|property|description|
|----------|-----------|
|`name`|*(Required.) * **string**. The name of the dataverse.|

## Definitions

### Binary

A string containing Base64-encoded data.

|type|
|----|
|**string**.|

### NQuads

N-Quads Format

N-Quads is an extension of N-Triples to support RDF datasets by adding an optional fourth element to represent the graph name. See the [official N-Quads specification](https://www.w3.org/TR/n-quads/).

|literal|
|-------|
|`"n_quads"`|

### NTriples

N-Triples Format

N-Triples is a line-based, plain text format for encoding an RDF graph. Each line corresponds to a single RDF triple. See the [official N-Triples specification](https://www.w3.org/TR/n-triples/).

|literal|
|-------|
|`"n_triples"`|

### RdfFormat

`RdfFormat` represents the various serialization formats for RDF (Resource Description Framework) data.

|variant|description|
|-------|-----------|
|[RdfXml](#rdfxml)|**string**: `rdf_xml`. RDF/XML Format<br /><br />RDF/XML is a syntax to express RDF information in XML. See the [official RDF/XML specification](https://www.w3.org/TR/rdf-syntax-grammar/).|
|[Turtle](#turtle)|**string**: `turtle`. Turtle (Terse RDF Triple Language) Format<br /><br />Turtle is a textual format for representing RDF triples in a more compact and human-readable way compared to RDF/XML. See the [official Turtle specification](https://www.w3.org/TR/turtle/).|
|[NTriples](#ntriples)|**string**: `n_triples`. N-Triples Format<br /><br />N-Triples is a line-based, plain text format for encoding an RDF graph. Each line corresponds to a single RDF triple. See the [official N-Triples specification](https://www.w3.org/TR/n-triples/).|
|[NQuads](#nquads)|**string**: `n_quads`. N-Quads Format<br /><br />N-Quads is an extension of N-Triples to support RDF datasets by adding an optional fourth element to represent the graph name. See the [official N-Quads specification](https://www.w3.org/TR/n-quads/).|

### RdfXml

RDF/XML Format

RDF/XML is a syntax to express RDF information in XML. See the [official RDF/XML specification](https://www.w3.org/TR/rdf-syntax-grammar/).

|literal|
|-------|
|`"rdf_xml"`|

### Turtle

Turtle (Terse RDF Triple Language) Format

Turtle is a textual format for representing RDF triples in a more compact and human-readable way compared to RDF/XML. See the [official Turtle specification](https://www.w3.org/TR/turtle/).

|literal|
|-------|
|`"turtle"`|

---

*Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `okp4-dataverse.json` (`97457018a767e898`)*