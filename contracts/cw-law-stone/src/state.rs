use cosmwasm_std::StdError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::UriError;
use crate::msg::ProgramResponse;
use crate::ContractError;
use cw_storage::msg::QueryMsg as StorageQuery;
use cw_storage::msg::QueryMsg;
use cw_storage_plus::{Item, Map};
use url::Url;

/// State to store context during contract instantiation
pub const INSTANTIATE_CONTEXT: Item<'_, String> = Item::new("instantiate");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LawStone {
    pub broken: bool,
    pub law: Object,
}

impl From<LawStone> for ProgramResponse {
    fn from(value: LawStone) -> ProgramResponse {
        ProgramResponse {
            object_id: value.law.object_id,
            storage_address: value.law.storage_address,
        }
    }
}

/// Represent a link to an Object stored in the `cw-storage` contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Object {
    /// The object id in the `cw-storage` contract.
    pub object_id: String,

    /// The `cw-storage` contract address on which the object is stored.
    pub storage_address: String,
}

impl Object {
    const COSMWASM_SCHEME: &'static str = "cosmwasm";
}

impl TryFrom<Url> for Object {
    type Error = UriError;

    fn try_from(value: Url) -> Result<Self, UriError> {
        if value.scheme() != Object::COSMWASM_SCHEME {
            return Err(UriError::WrongScheme {
                scheme: value.scheme().to_string(),
                wanted: vec![Object::COSMWASM_SCHEME.to_string()],
            });
        }

        let path = value.path().to_string();
        let paths = path.split(':').collect::<Vec<&str>>();
        if paths.is_empty() || paths.len() > 2 {
            return Err(UriError::IncompatiblePath);
        }
        let storage_address = paths.last().ok_or(UriError::IncompatiblePath)?.to_string();

        let queries = value
            .query_pairs()
            .into_owned()
            .collect::<HashMap<String, String>>();

        if let Some(query) = queries.get("query") {
            let json: QueryMsg = serde_json_wasm::from_str(query.as_str())?;

            return match json {
                QueryMsg::ObjectData { id: object_id } => Ok(Object {
                    object_id,
                    storage_address,
                }),
                _ => Err(UriError::IncompatibleQuery),
            };
        }

        Err(UriError::MissingQueryKey)
    }
}

impl TryInto<Url> for Object {
    type Error = ContractError;

    fn try_into(self) -> Result<Url, Self::Error> {
        let raw = [
            Object::COSMWASM_SCHEME,
            ":cw-storage:",
            self.storage_address.as_str(),
            "?",
            form_urlencoded::Serializer::new(String::new())
                .append_pair(
                    "query",
                    serde_json_wasm::to_string(&StorageQuery::ObjectData { id: self.object_id })
                        .map_err(|e| {
                            ContractError::Std(StdError::serialize_err("StorageQuery", e))
                        })?
                        .as_str(),
                )
                .finish()
                .as_str(),
        ]
        .join("");

        Url::parse(&raw).map_err(|e| ContractError::LogicLoadUri {
            uri: raw,
            error: UriError::Parse(e),
        })
    }
}

pub const PROGRAM: Item<'_, LawStone> = Item::new("program");

pub const DEPENDENCIES: Map<'_, &str, Object> = Map::new("dependencies");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_try_from() {
        let cases = vec![
            (
                "coco:okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s?query=%7B%22object_data%22%3A%7B%22id%22%3A%20%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D".to_string(),
                Some(UriError::WrongScheme { scheme: "coco".to_string(), wanted: vec!["cosmwasm".to_string()] }),
                None
            ),
            (
                "cosmwasm:bob:alice:okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s?query=%7B%22object_data%22%3A%7B%22id%22%3A%20%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D".to_string(),
                Some(UriError::IncompatiblePath),
                None
            ),
            (
                "cosmwasm:cw-storage:okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s?q=%7B%22object_data%22%3A%7B%22id%22%3A%20%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D".to_string(),
                Some(UriError::MissingQueryKey),
                None
            ),
            (
                "cosmwasm:cw-storage:okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s?query=%7B%22object%22%3A%7B%22id%22%3A%20%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D".to_string(),
                Some(UriError::IncompatibleQuery),
                None
            ),
            (
                "cosmwasm:cw-storage:okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s?query=%7B%22object_data%22%3A%7B%22id%22%3A%20%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D".to_string(),
                None,
                Some(
                    Object {
                        object_id: "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05".to_string(),
                        storage_address: "okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s".to_string(),
                    }
                )
            ),
            (
                "cosmwasm:okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s?query=%7B%22object_data%22%3A%7B%22id%22%3A%20%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D".to_string(),
                None,
                Some(
                    Object {
                        object_id: "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05".to_string(),
                        storage_address: "okp41dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqavca4s".to_string(),
                    }
                )
            ),
        ];

        for case in cases {
            match Url::parse(case.0.as_str()) {
                Ok(url) => {
                    let result = Object::try_from(url);

                    if let Some(err) = case.1 {
                        assert_eq!(err.to_string(), result.unwrap_err().to_string())
                    } else if let Some(o) = case.2 {
                        assert_eq!(o, result.unwrap())
                    }
                }
                Err(_) => panic!("no error should be thrown"),
            }
        }
    }
}
