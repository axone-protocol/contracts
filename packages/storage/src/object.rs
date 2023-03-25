use cw_storage::msg::QueryMsg;
use cw_storage::msg::QueryMsg::ObjectData;
use logic_bindings::error::CosmwasmUriError;
use logic_bindings::uri::CosmwasmUri;
use serde::{Deserialize, Serialize};

const CONTRACT_NAME: &'static str = "cw-storage";

/// Represents a reference to an Object stored in the `cw-storage` contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ObjectRef {
    /// The object id in the `cw-storage` contract.
    pub object_id: String,

    /// The `cw-storage` contract address on which the object is stored.
    pub storage_address: String,
}

impl TryFrom<CosmwasmUri> for ObjectRef {
    type Error = CosmwasmUriError;

    fn try_from(value: CosmwasmUri) -> Result<Self, Self::Error> {
        let address = value.contract_address.clone();
        value
            .into_query::<QueryMsg>()
            .and_then(|query: QueryMsg| match query {
                ObjectData { id: object_id } => Ok(ObjectRef {
                    storage_address: address,
                    object_id,
                }),
                _ => Err(CosmwasmUriError::Malformed(
                    "wrong query content".to_string(),
                )),
            })
    }
}

impl TryFrom<ObjectRef> for CosmwasmUri {
    type Error = CosmwasmUriError;

    fn try_from(value: ObjectRef) -> Result<Self, Self::Error> {
        CosmwasmUri::try_new(
            Some(CONTRACT_NAME.to_string()),
            value.storage_address,
            &ObjectData {
                id: value.object_id,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uri_to_object() {
        let cases = vec![
            (
                CosmwasmUri {
                    contract_name: None,
                    contract_address: "okp41ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pqrteqt3".to_string(),
                    raw_query: "{\"object_data\":{\"id\":\"4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05\"}}".to_string(),
                },
                Ok(ObjectRef {
                    object_id: "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05".to_string(),
                    storage_address: "okp41ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pqrteqt3".to_string(),
                }),
            ),
            (
                CosmwasmUri {
                    contract_name: None,
                    contract_address: "address".to_string(),
                    raw_query: "{\"object\":{\"id\":\"myid\"}}".to_string(),
                },
                Err(CosmwasmUriError::Malformed(
                    "wrong query content".to_string(),
                )),
            ),
        ];

        for case in cases {
            let res = ObjectRef::try_from(case.0);
            assert_eq!(res, case.1);
        }
    }

    #[test]
    fn object_ro_uri() {
        let res = CosmwasmUri::try_from(ObjectRef {
            object_id: "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05"
                .to_string(),
            storage_address: "okp41ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pqrteqt3"
                .to_string(),
        });

        assert!(res.is_ok());
        assert_eq!(res.unwrap().to_string(), "cosmwasm:cw-storage:okp41ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pqrteqt3?query=%7B%22object_data%22%3A%7B%22id%22%3A%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D");
    }
}
