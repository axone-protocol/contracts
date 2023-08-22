use cosmwasm_schema::{cw_serde, QueryResponses};
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
