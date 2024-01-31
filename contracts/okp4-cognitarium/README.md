# Cognitarium

A [CosmWasm](https://cosmwasm.com/) Smart Contract which enables the storage and querying of Semantic data using [RDF (Resource Description Framework)](https://en.wikipedia.org/wiki/Resource_Description_Framework), which represents information as semantic triples.

## Purpose

The Smart Contract operates as a [semantic database](https://en.wikipedia.org/wiki/Triplestore), adept at storing and fetching [RDF triples](https://en.wikipedia.org/wiki/Semantic_triple) via semantic queries. It can be deployed on any blockchain within the [Cosmos blockchains](https://cosmos.network/) network, utilizing the [CosmWasm](https://cosmwasm.com/) framework.

The key features of the contract include:

**Insertion of RDF Triples:**
This functionality enables the insertion of new data in the form of [RDF triples](https://en.wikipedia.org/wiki/Semantic_triple) onto the blockchain, ensuring secure and tamper-proof storage. The Smart Contract supports inserting these triples in various serialization formats including [RDF/XML](https://en.wikipedia.org/wiki/RDF/XML), [Turtle](https://www.w3.org/TR/turtle/), [N-Triples](https://www.w3.org/TR/n-triples/) and [N-Quads](https://www.w3.org/TR/n-quads/).

**Removal of RDF Triples:**
This functionality enables the selective deletion of RDF triples from the on-chain store. Users can specify patterns or criteria that identify the triples to be removed, ensuring precise and targeted removal of data.

**Querying of RDF Triples:**
The Smart Contract provides powerful on-chain querying capabilities, allowing users to retrieve specific RDF triples stored on the blockchain. This is done using a variation of [SPARQL](https://www.w3.org/TR/sparql11-query/), a specialized query language designed for retrieving and manipulating data stored in RDF format. Users can specify their search criteria in the query, and the Smart Contract will return the matching RDF triples, directly accessing the on-chain data. This feature supports various serialization formats for the output, such as Turtle, N-Triples, N-Quads, and RDF/XML, offering flexibility in how the retrieved data is presented and used.

**Policies of the Store:**
The Smart Contract includes a straightforward yet effective policies functionality to manage the capacity and usage of the on-chain storage effectively. These policies ensure efficient operation and prevent misuse or overuse of the Smart Contract. For instance:

- Maximum Triples: Caps the total number of RDF triples the store can hold, preventing database overload.
- Storage Size Limit: Sets an upper bound on the store's data size in bytes, managing blockchain resource use.
- Query Size Limit: Restricts the size or complexity of queries to maintain fast and reliable data retrieval.
- Insert Data Limit: Limits the size of data (in bytes) that can be added in a single transaction, ensuring smooth and efficient data insertion.

## Rationale

The data preserved in the blockchain holds significant value due to its derivation from a distributed consensus, rendering it a reliable source for decision-making, applicable to both on-chain and off-chain scenarios.

To effectively utilize this data, it's essential to adopt representation models that cater to diverse requirements. The Smart Contract Cognitarium provides such a model, facilitating the depiction of intricate and evolving semantic connections within a highly interconnected dataset. This approach transforms the data into a Knowledge Graph, enabling an accurate portrayal of existing facts and fostering the generation of new insights.
