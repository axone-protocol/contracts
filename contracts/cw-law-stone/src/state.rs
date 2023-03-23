use std::borrow::Cow;
use std::collections::HashMap;
use cosmwasm_std::Binary;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};
use url::Url;
use crate::ContractError;
use crate::ContractError::NotImplemented;
use crate::error::UriError;
use cw_storage::msg::QueryMsg;

/// State to store context during contract instantiation
pub const INSTANTIATE_CONTEXT: Item<'_, (String, Binary)> = Item::new("instantiate");

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
    const COSMWASM_PATH_KEY: &'static str = "cw-storage";
}
impl TryFrom<Url> for Object {
    type Error = UriError;

    fn try_from(value: Url) -> Result<Self, UriError> {
        if value.scheme() != Object::COSMWASM_SCHEME {
            return Err(UriError::WrongScheme { scheme: value.scheme().to_string(), wanted: vec![Object::COSMWASM_SCHEME.to_string()] })
        }

        let path = value.path().to_string();
        let paths = path.split(":").collect::<Vec<&str>>();
        if paths.len() == 0 || paths.len() > 2 {
            return Err(UriError::IncompatiblePath)
        }
        let storage_address = paths.last().ok_or(UriError::IncompatiblePath)?.to_string();

        let queries = value.query_pairs().into_owned().collect::<HashMap<String, String>>();

        if let Some(query) = queries.get("query") {
            let json: QueryMsg = serde_json_wasm::from_str(query.as_str())?;

            return match json {
                QueryMsg::ObjectData { id: object_id } => Ok(Object { object_id, storage_address }),
                _ => Err(UriError::IncompatibleQuery)
            }
        }

        Err(UriError::MissingQueryKey)
    }
}

pub const PROGRAM: Item<'_, Object> = Item::new("program");

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
                    }
                    else if let Some(o) = case.2 {
                        assert_eq!(o, result.unwrap())
                    }
                }
                Err(_) => panic!("no error should be thrown")
            }
        }
    }
}
