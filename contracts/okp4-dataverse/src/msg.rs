use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

/// `InstantiateMsg` is used to initialize a new instance of the dataverse.
#[cw_serde]
pub struct InstantiateMsg {
    /// A unique name to identify the dataverse instance.
    pub name: String,
}

/// `ExecuteMsg` defines the set of possible actions that can be performed on the dataverse.
///
/// This enum provides variants for registering services, datasets, and other operations related to the dataverse.
#[cw_serde]
pub enum ExecuteMsg {
    /// # RegisterService
    /// Registers a new Service within the dataverse.
    ///
    /// The term 'Service' in this context is employed to denote any form of service that is accessible over a network.
    /// This encompasses, but is not limited to, services such as REST APIs, gRPC services, and similar network-based
    /// services.
    ///
    /// A fundamental characteristic of each service is its unique Uniform Resource Identifier (URI), which serves as
    /// the definitive entry point for accessing the service. This URI is pivotal in the identification and location of
    /// the service within the network.
    RegisterService {
        /// The decentralized identity (DID) of the service.
        ///
        /// Preconditions:
        /// - The identity must be unique within the dataverse.
        identity: Did,
        /// The URI that identifies and locates the service.
        ///
        /// The URI serves a dual purpose:
        /// 1. **Identification**: It provides a unique identifier for the service, ensuring that each service can be distinctly recognized within the dataverse.
        /// 2. **Endpoint**: The URI acts as the access point or endpoint for the service. It specifies where the service can be accessed and how interactions with the service should be initiated.
        identifier: Uri,
        /// The URI of the entity responsible for registering and managing the service in the dataverse (i.e. on the blockchain).
        /// It's an optional field, if not provided the service is registered by the entity that invokes the transaction.
        registrar: Option<Did>,
    },

    /// # RegisterDigitalResource
    /// Registers a new digital resource within the dataverse.
    ///
    /// A Digital Resource represents a broad category encompassing various digital entities registerable in the dataverse.
    /// This category includes, but is not limited to, datasets, algorithms, machine learning models, and other digital assets.
    ///
    /// The unique identification of each Digital Resource is achieved through a combination of its Uniform Resource Identifier (URI)
    /// and the specific service responsible for its provision. This dual-component identification mechanism guarantees the distinct
    /// recognition and operationalization of each Digital Resource within the dataverse environment.
    RegisterDigitalResource {
        /// The decentralized identity (DID) of the Digital Resource.
        ///
        /// Preconditions:
        /// - The identity must be unique within the dataverse.
        identity: Did,
        /// The URI that identifies the dataset.
        /// This URI makes sense only in the context of the service that provides the resource.
        ///
        /// Preconditions:
        /// - The URI must be unique within the dataverse.
        identifier: Uri,
        /// The URI of the service, already registered in the dataverse, that provides the dataset.
        ///
        /// Preconditions:
        /// - The Service must be registered in the dataverse before the resource can be registered.
        provided_by: Uri,
        /// The URI of the entity responsible for registering and managing the resource in the dataverse (i.e. on the blockchain).
        /// It's an optional field, if not provided the dataset is registered by the entity that invokes the transaction.
        registrar: Option<Did>,
    },

    /// # FoundZone
    /// Founds a new zone within the dataverse.
    ///
    /// `Zone` is a conceptual framework that is established based on a set of rules, within which
    /// recognized Resources must conform, considering associated consents.
    FoundZone {
        /// The decentralized identity (DID) of the Zone.
        /// This identity must be unique within the dataverse.
        identity: Did,
        /// The URI of the entity responsible for registering and managing the zone in the dataverse (i.e. on the blockchain).
        /// It's an optional field, if not provided the zone is registered by the entity that invokes the transaction.
        registrar: Option<Did>,
    },

    /// # SubmitClaims
    /// Submits new claims about a resource to the dataverse.
    ///
    /// A claim is a statement made by an entity, the issuer (e.g. a person, an organization, or a machine) about a resource
    /// (e.g. an entity, a service, or a zone) that the issuer asserts to be true.
    ///
    /// The claims are submitted to the dataverse in the form of Verifiable Presentations (VPs), which combine and present credentials.
    /// The data in the presentation concerns usually the same subject, but there is no limit to the number of subjects or
    /// issuers in the data.
    ///
    /// Preconditions:
    /// - The claims must be submitted in the form of Verifiable Presentations (VPs).
    /// - The subjects of the Verifiable Credentials must exist in the dataverse before the claims can be submitted.
    /// - The identifiers of the Veriable Credentials must be unique within the dataverse.
    /// - The claims must be signed by the issuer and the signature must be verifiable.
    SubmitClaims {
        /// The serialized metadata intended for attachment.
        /// This metadata should adhere to the format specified in the `format` field.
        metadata: Binary,
        /// RDF format in which the metadata is represented.
        /// If not provided, the default format is [Turtle](https://www.w3.org/TR/turtle/) format.
        format: Option<RdfFormat>,
    },

    /// # RevokeClaims
    /// Revoke or withdraw a previously submitted claims.
    ///
    /// Preconditions:
    /// - The identifier of the claims must exist in the dataverse.
    RevokeClaims {
        /// The unique identifier of the claims to be revoked.
        identifier: Uri,
    },
}

/// # RdfFormat
/// `RdfFormat` represents the various serialization formats for RDF (Resource Description Framework) data.
#[cw_serde]
#[derive(Default)]
pub enum RdfFormat {
    /// # RdfXml
    /// RDF/XML Format
    ///
    /// RDF/XML is a syntax to express RDF information in XML.
    /// See the [official RDF/XML specification](https://www.w3.org/TR/rdf-syntax-grammar/).
    #[serde(rename = "rdf_xml")]
    RdfXml,

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

/// # Uri
/// `Uri` represents a Uniform Resource Identifier (URI), a string of characters that provides a simple way
/// to identify a resource.
/// see https://en.wikipedia.org/wiki/Uniform_Resource_Identifier.
type Uri = String;

/// # Did
/// `Did` represents a Decentralized Identifier (DID), a globally unique identifier.
/// see https://www.w3.org/TR/did-core/.
type Did = Uri;

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
