# Dataverse

## Overview

The `dataverse smart contract is responsible for overseeing and managing the dataverse. The Dataverse is an ever-expanding universe that encompasses a wide range of digital resources. These include datasets, data processing algorithms, storage resources, computational resources, identity management solutions, orchestration engines, oracles, and many other resources recorded on the blockchain. Within the Dataverse, there are defined Zones where specific rules apply. Digital resources recognized within these Zones are the ones compatible with these rules, considering the associated consents. Hence the smart contract also provides mechanisms to manage these Zones, ensuring the implementation of precise governance rules.

## Instances

When the smart contract is instantiated, it creates a Dataverse instance. This instance is separate and isolated from any pre-existing ones, and as many dataverse instances as required can be created.

## Dependencies

Given its role and status, this smart contract serves as the primary access point for the OKP4 protocol to manage all on-chain stored resources. To fulfill its tasks, the smart contract relies on other smart contracts within the OKP4 ecosystem. Notably, it uses the `Cognitarium` smart contract for persisting the Dataverse representation in an ontological form and the `Law Stone`` smart contract to establish governance rules.
