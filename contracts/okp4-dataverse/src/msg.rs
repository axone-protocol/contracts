use cosmwasm_schema::{cw_serde, QueryResponses};
#[allow(unused_imports)]
use okp4_logic_bindings::AskResponse;

/// Instantiate message
#[cw_serde]
pub struct InstantiateMsg {
    /// A name to give to the dataverse instantiated.
    pub name: String,
}

/// DID is a type alias for a string that represents a Decentralized Identifier (DID).
type DID = String;

/// URI is a type alias for a string that represents a URI.
type URI = String;

/// Execute messages
#[cw_serde]
pub enum ExecuteMsg {
    /// # RegisterService
    /// RegisterService registers a service in the dataverse.
    /// Service is a generic concept for any kind of service that can be provided through a network (e.g. a REST API, a gRPC service, etc.).
    ///
    /// Each service is identified and located by its unique [URI](https://en.wikipedia.org/wiki/Uniform_Resource_Identifier) which defines the entry point
    /// of the service.
    RegisterService {
        /// Identity of the service.
        identity: DID,

        /// Identifier of the service.
        /// This identifier is unique within the scope of the dataverse and is used to reference the service and describe how to interact with it.
        identifier: URI,

        /// References the individual, company, or organization that is responsible for registering and
        /// managing the service.
        registrar: URI,
    },

    /// # RegisterDataset
    /// RegisterDataset registers a dataset in the dataverse.
    ///
    /// `Dataset` represents the group of related data that is organized and presented in a specific format by the provider. This data can take many forms,
    /// including csv files, images, videos, and more, as well as data sources such as databases and APIs.
    ///
    /// Each dataset is identified and located by its unique [URI](https://en.wikipedia.org/wiki/Uniform_Resource_Identifier) which defines the path and
    /// mean to access the data through the provider service.
    RegisterDataset {
        /// Identifier of the dataset.
        identifier: URI,

        /// References the Service, already registered in the dataverse, that provides the dataset.
        provided_by: URI,

        /// References the individual, company, or organization that is responsible for registering and
        /// managing the dataset.
        registrar: URI,
    },
}

/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// # Dataverse
    /// Dataverse returns the information about the dataverse.
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
