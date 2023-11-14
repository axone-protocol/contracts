# Dataverse

## Overview

The `dataverse` smart contract is responsible for overseeing and managing the Dataverse.

## Dataverse

The Dataverse is an ever-expanding universe that encompasses a wide range of digital resources. These include datasets, data processing algorithms, ML algorithm, storage resources, computational resources, identity management solutions, orchestration engines, oracles, and many other resources recorded on the blockchain.

When the smart contract is instantiated, it creates a Dataverse instance. This instance is separated and isolated from any pre-existing ones, and as many dataverse instances as required can be created.

## Zones

Zones within the Dataverse represent distinct areas or domains where specific governance rules and policies are applied. These Zones are conceptual frameworks created to manage and organize resources under a unified set of regulations and permissions.

Each Zone is defined by its unique identity and set of governing rules, which dictate how resources within it can be accessed, used, and shared. This approach allows for granular control over different segments of the Dataverse, catering to various requirements and use cases. By managing these Zones, the dataverse smart contract ensures that resources are utilized in compliance with the defined policies and consents, thereby maintaining order and integrity within the Dataverse.

## Resources

In the context of the Dataverse, Resources refer to a broad category of digital entities, which include Services and Digital Resources.

- **Digital Resources**: This category extends to various digital entities such as datasets, algorithms, machine learning models, and other digital assets. Like Services, Digital Resources are identified by a URI in conjunction with the Service responsible for their provision.

- **Services**: These are network-accessible functionalities like REST APIs, gRPC services, and other similar offerings. Each Service in the Dataverse is uniquely identified by its Uniform Resource Identifier (URI) and is associated with a specific Registrar responsible for its registration and management.

## Decentralized Identifiers (DID)

Decentralized Identifiers (DID) are a foundational element in the Dataverse, serving as unique, persistent, and globally resolvable identifiers that are fully under the control of the DID subject, which could be an individual, organization, or a any kind of resource (dataset,
algorithm, nft, ML algorithm).

DIDs play a crucial role in the Dataverse by facilitating a trustable and interoperable identity mechanism. They enable the establishment of a verifiable and self-sovereign identity for resources, services, and entities within the ecosystem.

## Claims

Claims in the Dataverse context are assertions or statements made about a Resource identified by a DID.

Claims play a pivotal role in the governance framework of the Dataverse. By leveraging knowledge derived from verifiable credentials, the governances established by Zones can evaluate the fulfilment of specific rules and compliance. This evaluation is critical in ensuring that the resources within the Dataverse adhere to the established norms, policies, and requirements.

Claims are submitted in the form of [Verifiable Presentations (VPs)](https://www.w3.org/TR/vc-data-model/#presentations), which are aggregations of one or more [Verifiable Credentials (VCs)](https://www.w3.org/TR/vc-data-model/#what-is-a-verifiable-credential).

## Dependencies

Given its role and status, this smart contract serves as the primary access point for the OKP4 protocol to manage all on-chain stored resources. To fulfill its tasks, the smart contract relies on other smart contracts within the OKP4 ecosystem. Notably, it uses the `Cognitarium` smart contract for persisting the Dataverse representation in an ontological form and the `Law Stone` smart contract to establish governance rules.

## InstantiateMsg

`InstantiateMsg` is used to initialize a new instance of the dataverse.

|parameter|description|
|----------|-----------|
|`name`|*(Required.) * **string**. A unique name to identify the dataverse instance.|

## ExecuteMsg

`ExecuteMsg` defines the set of possible actions that can be performed on the dataverse.

This enum provides variants for registering services, datasets, and other operations related to the dataverse.

### ExecuteMsg::RegisterService

Registers a new Service within the dataverse.

The term 'Service' in this context is employed to denote any form of service that is accessible over a network. This encompasses, but is not limited to, services such as REST APIs, gRPC services, and similar network-based services.

A fundamental characteristic of each service is its unique Uniform Resource Identifier (URI), which serves as the definitive entry point for accessing the service. This URI is pivotal in the identification and location of the service within the network.

|parameter|description|
|----------|-----------|
|`register_service`|*(Required.) * **object**. |
|`register_service.identifier`|*(Required.) * **string**. The URI that identifies and locates the service.<br /><br />The URI serves a dual purpose: 1. **Identification**: It provides a unique identifier for the service, ensuring that each service can be distinctly recognized within the dataverse. 2. **Endpoint**: The URI acts as the access point or endpoint for the service. It specifies where the service can be accessed and how interactions with the service should be initiated.|
|`register_service.identity`|*(Required.) * **string**. The decentralized identity (DID) of the service.<br /><br />Preconditions: - The identity must be unique within the dataverse.|
|`register_service.registrar`|**string\|null**. The URI of the entity responsible for registering and managing the service in the dataverse (i.e. on the blockchain). It's an optional field, if not provided the service is registered by the entity that invokes the transaction.|

### ExecuteMsg::RegisterDigitalResource

Registers a new digital resource within the dataverse.

A Digital Resource represents a broad category encompassing various digital entities registerable in the dataverse. This category includes, but is not limited to, datasets, algorithms, machine learning models, and other digital assets.

The unique identification of each Digital Resource is achieved through a combination of its Uniform Resource Identifier (URI) and the specific service responsible for its provision. This dual-component identification mechanism guarantees the distinct recognition and operationalization of each Digital Resource within the dataverse environment.

|parameter|description|
|----------|-----------|
|`register_digital_resource`|*(Required.) * **object**. |
|`register_digital_resource.identifier`|*(Required.) * **string**. The URI that identifies the dataset. This URI makes sense only in the context of the service that provides the dataset.<br /><br />Preconditions: - The URI must be unique within the dataverse.|
|`register_digital_resource.identity`|*(Required.) * **string**. The decentralized identity (DID) of the Digital Resource.<br /><br />Preconditions: - The identity must be unique within the dataverse.|
|`register_digital_resource.provided_by`|*(Required.) * **string**. The URI of the service, already registered in the dataverse, that provides the dataset.<br /><br />Preconditions: - The Service must be registered in the dataverse before the dataset can be registered.|
|`register_digital_resource.registrar`|**string\|null**. The URI of the entity responsible for registering and managing the dataset in the dataverse (i.e. on the blockchain). It's an optional field, if not provided the dataset is registered by the entity that invokes the transaction.|

### ExecuteMsg::FoundZone

Founds a new zone within the dataverse.

`Zone` is a conceptual framework that is established based on a set of rules, within which recognized Resources must conform, considering associated consents.

|parameter|description|
|----------|-----------|
|`found_zone`|*(Required.) * **object**. |
|`found_zone.identity`|*(Required.) * **string**. The decentralized identity (DID) of the Zone. This identity must be unique within the dataverse.|
|`found_zone.registrar`|**string\|null**. The URI of the entity responsible for registering and managing the zone in the dataverse (i.e. on the blockchain). It's an optional field, if not provided the zone is registered by the entity that invokes the transaction.|

### ExecuteMsg::SubmitClaims

Submits new claims about a resource to the dataverse.

A claim is a statement made by an entity, the issuer (e.g. a person, an organization, or a machine) about a resource (e.g. an entity, a service, or a zone) that the issuer asserts to be true.

The claims are submitted to the dataverse in the form of Verifiable Presentations (VPs), which combine and present credentials. The data in the presentation concerns usually the same subject, but there is no limit to the number of subjects or issuers in the data.

Preconditions: - The claims must be submitted in the form of Verifiable Presentations (VPs). - The subjects of the Verifiable Credentials must exist in the dataverse before the claims can be submitted. - The identifiers of the Veriable Credentials must be unique within the dataverse. - The claims must be signed by the issuer and the signature must be verifiable.

|parameter|description|
|----------|-----------|
|`submit_claims`|*(Required.) * **object**. |
|`submit_claims.format`|**[RdfFormat](#rdfformat)\|null**. RDF format in which the metadata is represented. If not provided, the default format is [Turtle](https://www.w3.org/TR/turtle/) format.|
|`submit_claims.metadata`|*(Required.) * **[Binary](#binary)**. The serialized metadata intended for attachment. This metadata should adhere to the format specified in the `format` field.|

### ExecuteMsg::RevokeClaims

Revoke or withdraw a previously submitted claims.

Preconditions: - The identifier of the claims must exist in the dataverse.

|parameter|description|
|----------|-----------|
|`revoke_claims`|*(Required.) * **object**. |
|`revoke_claims.identifier`|*(Required.) * **string**. The unique identifier of the claims to be revoked.|

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

*Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `okp4-dataverse.json` (`dcbd7d6ba7b75fcf`)*