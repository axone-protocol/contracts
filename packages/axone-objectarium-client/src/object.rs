use axone_objectarium::msg::QueryMsg::ObjectData;
use axone_objectarium::msg::{ExecuteMsg, QueryMsg};
use axone_wasm::error::CosmwasmUriError;
use axone_wasm::uri::CosmwasmUri;
use cosmwasm_std::{to_json_binary, Coin, StdResult, WasmMsg};
use serde::{Deserialize, Serialize};

const CONTRACT_NAME: &str = "axone-objectarium";

/// Represents a reference to an Object stored in the `axone-objectarium` contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ObjectRef {
    /// The object id in the `axone-objectarium` contract.
    pub object_id: String,

    /// The `axone-objectarium` contract address on which the object is stored.
    pub storage_address: String,
}

impl ObjectRef {
    fn to_wasm_exec_msg<T>(&self, msg: &T, funds: Vec<Coin>) -> StdResult<WasmMsg>
    where
        T: Serialize + ?Sized,
    {
        Ok(WasmMsg::Execute {
            contract_addr: self.storage_address.clone(),
            msg: to_json_binary(msg)?,
            funds,
        })
    }

    pub fn to_exec_forget_msg(&self, funds: Vec<Coin>) -> StdResult<WasmMsg> {
        self.to_wasm_exec_msg(
            &ExecuteMsg::ForgetObject {
                id: self.object_id.clone(),
            },
            funds,
        )
    }

    pub fn to_exec_pin_msg(&self, funds: Vec<Coin>) -> StdResult<WasmMsg> {
        self.to_wasm_exec_msg(
            &ExecuteMsg::PinObject {
                id: self.object_id.clone(),
            },
            funds,
        )
    }

    pub fn to_exec_unpin_msg(&self, funds: Vec<Coin>) -> StdResult<WasmMsg> {
        self.to_wasm_exec_msg(
            &ExecuteMsg::UnpinObject {
                id: self.object_id.clone(),
            },
            funds,
        )
    }
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
    use cosmwasm_std::from_json;

    #[test]
    fn uri_to_object() {
        let cases = vec![
            (
                CosmwasmUri {
                    contract_name: None,
                    contract_address: "axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv".to_string(),
                    raw_query: "{\"object_data\":{\"id\":\"4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05\"}}".to_string(),
                },
                Ok(ObjectRef {
                    object_id: "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05".to_string(),
                    storage_address: "axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv".to_string(),
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
    fn object_to_uri() {
        let res = CosmwasmUri::try_from(ObjectRef {
            object_id: "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05"
                .to_string(),
            storage_address: "axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv"
                .to_string(),
        });

        assert!(res.is_ok());
        assert_eq!(res.unwrap().to_string(), "cosmwasm:axone-objectarium:axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv?query=%7B%22object_data%22%3A%7B%22id%22%3A%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D");
    }

    #[test]
    fn object_to_wasm_msg() {
        let funds = vec![Coin::new(100u128, "uknow")];
        let object = ObjectRef {
            object_id: "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05"
                .to_string(),
            storage_address: "axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv"
                .to_string(),
        };

        type ToExecuteMsgFn = Box<dyn FnOnce(ObjectRef, Vec<Coin>) -> StdResult<WasmMsg>>;
        let cases: Vec<(ToExecuteMsgFn, ExecuteMsg)> = vec![
            (
                Box::from(|obj: ObjectRef, f| obj.to_exec_forget_msg(f)),
                ExecuteMsg::ForgetObject {
                    id: object.object_id.clone(),
                },
            ),
            (
                Box::from(|obj: ObjectRef, f| obj.to_exec_pin_msg(f)),
                ExecuteMsg::PinObject {
                    id: object.object_id.clone(),
                },
            ),
            (
                Box::from(|obj: ObjectRef, f| obj.to_exec_unpin_msg(f)),
                ExecuteMsg::UnpinObject {
                    id: object.object_id.clone(),
                },
            ),
        ];

        for case in cases {
            let res = case.0(object.clone(), funds.clone());
            assert!(res.is_ok());

            match res.unwrap() {
                WasmMsg::Execute {
                    contract_addr: addr,
                    msg,
                    funds: f,
                } => {
                    assert_eq!(addr, object.storage_address.clone());
                    assert_eq!(f, funds);
                    let exec_res = from_json::<ExecuteMsg>(&msg);
                    assert!(exec_res.is_ok());
                    assert_eq!(exec_res.unwrap(), case.1)
                }
                _ => panic!("Expected 'WasmMsg::Execute'"),
            }
        }
    }
}
