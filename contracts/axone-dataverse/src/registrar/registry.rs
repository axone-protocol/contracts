use crate::registrar::credential::DataverseCredential;
use crate::state::DATAVERSE;
use crate::ContractError;
use axone_cognitarium::msg::DataFormat;
use axone_cognitarium::parser::{
    Node, SelectItem, SelectQuery, TriplePattern, VarOrNamedNode, VarOrNode, VarOrNodeOrLiteral,
    WhereClause, IRI,
};
use axone_cognitarium_client::CognitariumClient;
use cosmwasm_std::{ensure, DepsMut, StdResult, Storage, WasmMsg};

/// ClaimRegistrar is the entity responsible to manage claims (i.e. submission and revocation) into
/// the Dataverse, ensuring that any pre-condition criteria to an action is met, and any attached
/// logic is properly executed.
pub struct ClaimRegistrar {
    triplestore: CognitariumClient,
}

impl ClaimRegistrar {
    const RDF_DATA_FORMAT: DataFormat = DataFormat::NTriples;

    pub fn try_new(storage: &dyn Storage) -> StdResult<Self> {
        DATAVERSE.load(storage).map(|dataverse| Self {
            triplestore: CognitariumClient::new(dataverse.triplestore_address),
        })
    }

    /// Checks if a credential exists in the triplestore by ID.
    /// Returns `true` if at least one triple is found, `false` otherwise.
    pub fn exists(&self, deps: &DepsMut<'_>, credential_id: &str) -> Result<bool, ContractError> {
        let query = SelectQuery {
            prefixes: Vec::new(),
            limit: Some(1),
            select: vec![SelectItem::Variable("p".into())],
            r#where: WhereClause::Bgp {
                patterns: vec![TriplePattern {
                    subject: VarOrNode::Node(Node::NamedNode(IRI::Full(credential_id.into()))),
                    predicate: VarOrNamedNode::Variable("p".into()),
                    object: VarOrNodeOrLiteral::Variable("o".into()),
                }],
            },
        };

        let response = self.triplestore.select(deps.querier, query)?;
        Ok(!response.results.bindings.is_empty())
    }

    pub fn submit_claim(
        &self,
        deps: &DepsMut<'_>,
        credential: &DataverseCredential<'_>,
    ) -> Result<WasmMsg, ContractError> {
        ensure!(
            !self.exists(deps, &credential.id)?,
            ContractError::CredentialAlreadyExists(credential.id.to_string())
        );

        self.triplestore
            .insert_data(
                Some(Self::RDF_DATA_FORMAT),
                credential.serialize((&Self::RDF_DATA_FORMAT).into())?,
            )
            .map_err(ContractError::from)
    }
}
