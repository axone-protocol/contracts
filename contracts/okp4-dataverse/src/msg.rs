use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;
#[allow(unused_imports)]
use okp4_logic_bindings::AskResponse;

/// `InstantiateMsg` is used to initialize a new instance of the dataverse.
#[cw_serde]
pub struct InstantiateMsg {
    /// A name to give to the dataverse instantiated.
    pub name: String,
}

/// `DID` represents a Decentralized Identifier (DID), a globally unique identifier.
/// see https://www.w3.org/TR/did-core/.
type DID = String;

/// `URI` represents a Uniform Resource Identifier (URI), a string of characters that provides a simple way
/// to identify a resource.
/// see https://en.wikipedia.org/wiki/Uniform_Resource_Identifier.
type URI = String;

/// `ExecuteMsg` defines the set of possible actions that can be performed on the dataverse.
///
/// This enum provides variants for registering services, datasets, and other operations related to the dataverse.
#[cw_serde]
pub enum ExecuteMsg {
    /// # RegisterService
    /// Registers a new service within the dataverse.
    /// Service is a generic concept for any kind of service that can be provided through a network (e.g. a REST API, a gRPC service, etc.).
    ///
    /// Each service is identified and located by its unique URI which defines the entry point
    /// of the service.
    ///
    /// #### Examples:
    ///
    /// ```rust
    /// ExecuteMsg::RegisterService {
    ///     identity: "did:key:z6MkrpCPVDHcsqi3aaqnemLC1aBTUwkfPwTyzc8sFWYwm1PA",
    ///     identifier: "urn:uuid:803cd033-2eed-4db7-847b-f46715a42a70",
    ///     registrar: "did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp",
    /// }
    /// ```
    RegisterService {
        /// The decentralized identity of the service.
        identity: DID,
        /// The unique URI that identifies and locates the service.
        ///
        /// The URI serves a dual purpose:
        /// 1. **Identification**: It provides a unique identifier for the service, ensuring that each service can be distinctly recognized within the dataverse.
        /// 2. **Endpoint**: The URI acts as the access point or endpoint for the service. It specifies where the service can be accessed and how interactions with the service should be initiated.
        identifier: URI,
        /// The URI of the entity responsible for registering and managing the service.
        registrar: URI,
    },

    /// # RegisterDataset
    /// Registers a new dataset within the dataverse.
    ///
    /// A `Dataset` represents a collection of related data that is organized and presented in a specific format by the provider. This data can be in various forms, such as CSV files, images, videos, and more. It can also refer to data sources like databases and APIs.
    ///
    /// Each dataset is uniquely identified by its URI, which serves as both the identifier and the access point for the dataset. When accessing a dataset, it's crucial to understand the protocol and methods supported by the dataset's endpoint. For instance, a dataset with an HTTP-based URI might be accessible via GET requests and may require specific headers or parameters for successful retrieval.
    ///
    /// #### Examples:
    ///
    /// ```rust
    /// ExecuteMsg::RegisterDataset {
    ///     identifier: "urn:uuid:3ed871dc-72d0-499f-b8c2-7edcad56a76e",
    ///     provided_by: "urn:uuid:803cd033-2eed-4db7-847b-f46715a42a70",
    ///     registrar: "did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp",
    /// }
    /// ```
    RegisterDataset {
        /// The unique URI that identifies the dataset.
        identifier: URI,

        /// The URI of the service, already registered in the dataverse, that provides the dataset.
        provided_by: URI,

        /// The URI of the entity responsible for registering and managing the dataset.
        registrar: URI,
    },

    /// # FoundZone
    /// Founds a new zone within the dataverse.
    ///
    /// `Zone` is a conceptual framework that is established based on a set of rules, within which recognized digital Resources must conform, considering
    ///  associated consents.
    ///
    /// #### Example
    ///
    /// ```
    /// ExecuteMsg::FoundZone {
    ///     identifier: "urn:uuid:6d1aaad8-9411-4758-a9f9-ed43358af1fd",
    ///     registrar: "did:key:0x04d1f1b8f8a7a28f9a5a254c326a963a22f5a5b5d5f5e5d5c5b5a5958575655",
    /// }
    /// ```
    FoundZone {
        /// The unique URI that identifies the zone.
        identifier: URI,

        /// The URI of the entity responsible for registering and managing the zone.
        registrar: URI,
    },

    /// # AttachMetadata
    /// Attaches metadata to a specified resource registered in the dataverse.
    ///
    /// Metadata provides additional information or details about a resource.
    AttachMetadata {
        /// The RDF identifier of the resource for which the metadata should be attached.
        resource_identifier: URI,

        /// RDF format in which the metadata is represented.
        /// If not provided, the default format is [Turtle](https://www.w3.org/TR/turtle/) format.
        format: Option<RDFFormat>,

        /// The serialized metadata intended for attachment.
        /// This metadata should adhere to the format specified in the `format` field.
        metadata: Binary,
    },

    /// # DetachMetadata
    /// Remove a previously associated metadata (from a specific resource within the dataverse).
    /// Once removed the metadata is no longer accessible.
    DetachMetadata {
        /// The RDF identifier of the metadata to be removed.
        resource_identifier: URI,
    },

    /// # ReviseMetadata
    /// Revises a previously associated metadata in order to update it or amend it.
    ReviseMetadata {
        /// The RDF identifier of the metadata to be revised.
        resource_identifier: URI,

        /// RDF format in which the metadata is represented.
        /// If not provided, the default format is [Turtle](https://www.w3.org/TR/turtle/) format.
        format: Option<RDFFormat>,

        /// The serialized metadata intended for revision.
        /// This metadata should adhere to the format specified in the `format` field.
        metadata: Binary,
    },
}

/// # RDFFormat
/// `RDFFormat` represents the various serialization formats for RDF (Resource Description Framework) data.
#[cw_serde]
#[derive(Default)]
pub enum RDFFormat {
    /// # RDFXml
    /// RDF/XML Format
    ///
    /// RDF/XML is a syntax to express RDF information in XML.
    /// See the [official RDF/XML specification](https://www.w3.org/TR/rdf-syntax-grammar/).
    #[serde(rename = "rdf_xml")]
    RDFXml,

    /// # Turtle
    /// Turtle (Terse RDF Triple Language) Format
    ///
    /// Turtle is a textual format for representing RDF triples in a more compact and human-readable way compared to RDF/XML.
    /// See the [official Turtle specification](https://www.w3.org/TR/turtle/).
    #[serde(rename = "turtle")]
    #[default]
    Turtle,

    /// # NTriples
    /// N-Triples Format
    ///
    /// N-Triples is a line-based, plain text format for encoding an RDF graph. Each line corresponds to a single RDF triple.
    /// See the [official N-Triples specification](https://www.w3.org/TR/n-triples/).
    #[serde(rename = "n_triples")]
    NTriples,

    /// # NQuads
    /// N-Quads Format
    ///
    /// N-Quads is an extension of N-Triples to support RDF datasets by adding an optional fourth element to represent the graph name.
    /// See the [official N-Quads specification](https://www.w3.org/TR/n-quads/).
    #[serde(rename = "n_quads")]
    NQuads,
}

/// `QueryMsg` defines the set of possible queries that can be made to retrieve information about the dataverse.
///
/// This enum provides variants for querying the dataverse's details and other related information.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// # Dataverse
    /// Retrieves information about the current dataverse instance.
    #[returns(DataverseResponse)]
    Dataverse {},
}

/// # DataverseResponse
/// DataverseResponse is the response of the Dataverse query.
#[cw_serde]
pub struct DataverseResponse {
    /// The name of the dataverse.
    pub name: String,
}
