use crate::registrar::credential::DataverseCredential;
use crate::state::DATAVERSE;
use crate::ContractError;
use axone_cognitarium::msg::{
    DataFormat, Node, SelectItem, SelectQuery, TriplePattern, VarOrNamedNode, VarOrNode,
    VarOrNodeOrLiteral, WhereClause, IRI,
};
use axone_cognitarium_client::CognitariumClient;
use cosmwasm_std::{DepsMut, StdResult, Storage, WasmMsg};

/// ClaimRegistrar is the entity responsible to manage claims (i.e. submission and revocation) into
/// the Dataverse, ensuring that any pre-condition criteria to an action is met, and any attached
/// logic is properly executed.
pub struct ClaimRegistrar {
    triplestore: CognitariumClient,
}

impl ClaimRegistrar {
    const RDF_DATA_FORMAT: DataFormat = DataFormat::NTriples;

    pub fn try_new(storage: &dyn Storage) -> StdResult<Self> {
        let dataverse = DATAVERSE.load(storage)?;
        Ok(Self {
            triplestore: CognitariumClient::new(dataverse.triplestore_address),
        })
    }

    pub fn submit_claim(
        &self,
        deps: &DepsMut<'_>,
        credential: &DataverseCredential<'_>,
    ) -> Result<WasmMsg, ContractError> {
        let resp = self.triplestore.select(
            deps.querier,
            SelectQuery {
                prefixes: vec![],
                limit: Some(1u32),
                select: vec![SelectItem::Variable("p".to_string())],
                r#where: WhereClause::Bgp {
                    patterns: vec![TriplePattern {
                        subject: VarOrNode::Node(Node::NamedNode(IRI::Full(
                            credential.id.to_string(),
                        ))),
                        predicate: VarOrNamedNode::Variable("p".to_string()),
                        object: VarOrNodeOrLiteral::Variable("o".to_string()),
                    }],
                },
            },
        )?;

        if !resp.results.bindings.is_empty() {
            Err(ContractError::CredentialAlreadyExists(
                credential.id.to_string(),
            ))?;
        }

        self.triplestore
            .insert_data(
                Some(Self::RDF_DATA_FORMAT),
                credential.serialize((&Self::RDF_DATA_FORMAT).into())?,
            )
            .map_err(ContractError::from)
    }
}
