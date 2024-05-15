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

Given its role and status, this smart contract serves as the primary access point for the AXONE protocol to manage all on-chain stored resources. To fulfill its tasks, the smart contract relies on other smart contracts within the AXONE ecosystem. Notably, it uses the `Cognitarium` smart contract for persisting the Dataverse representation in an ontological form and the `Law Stone` smart contract to establish governance rules.
