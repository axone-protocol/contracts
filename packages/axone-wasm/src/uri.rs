use crate::error::CosmwasmUriError;
use serde::{de, ser};
use std::collections::HashMap;
use std::fmt::Display;
use url::Url;

const COSMWASM_SCHEME: &str = "cosmwasm";
const COSMWASM_QUERY_PARAM: &str = "query";

/// A CosmWasm URI identifies a resource on a blockchain by referencing a specific instantiated
/// smart contract. It includes the contract's address and uses query parameters to encode the message
/// intended for the contract. The resource identified by the URI is the response provided by the
/// smart contract following this query.
///
/// Its general form is as follows:
///
/// `cosmwasm:{contract_name}:{contract_address}?query={contract_query}`
///
/// Where:
/// - `{contract_name}`: Only informative, represents the corresponding smart contract name or type (e.g. `axone-objectarium`);
/// - `{contract_address}`: The address of the smart contract to query;
/// - `{contract_query}`: The JSON query to perform on the targeted smart contract, URL encoded;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CosmwasmUri {
    pub contract_name: Option<String>,
    pub contract_address: String,
    pub raw_query: String,
}

impl CosmwasmUri {
    pub fn try_new<T>(
        contract_name: Option<String>,
        contract_address: String,
        query: &T,
    ) -> Result<CosmwasmUri, CosmwasmUriError>
    where
        T: ser::Serialize + ?Sized,
    {
        serde_json_wasm::to_string(query)
            .map_err(|e| CosmwasmUriError::SerializeQuery(e.to_string()))
            .map(|raw_query| CosmwasmUri {
                contract_name,
                contract_address,
                raw_query,
            })
    }

    pub fn into_query<T>(self) -> Result<T, CosmwasmUriError>
    where
        T: de::DeserializeOwned,
    {
        serde_json_wasm::from_str(self.raw_query.as_str())
            .map_err(|e| CosmwasmUriError::ParseQuery(e.to_string()))
    }

    fn encode_query(self) -> String {
        return form_urlencoded::Serializer::new(String::new())
            .append_pair(COSMWASM_QUERY_PARAM, self.raw_query.as_str())
            .finish();
    }
}

impl TryFrom<String> for CosmwasmUri {
    type Error = CosmwasmUriError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Url::parse(value.as_str())
            .map_err(CosmwasmUriError::ParseURI)
            .and_then(|uri: Url| {
                if uri.scheme() != COSMWASM_SCHEME {
                    return Err(CosmwasmUriError::Malformed("wrong scheme".to_string()));
                }

                let path = uri.path().to_string();
                let mut path_parts = path.split(':').map(String::from).collect::<Vec<String>>();
                let (contract_name, contract_address) =
                    match (path_parts.pop(), path_parts.pop(), path_parts.pop()) {
                        (Some(address), Some(name), None) if !address.is_empty() => {
                            Ok((Some(name), address))
                        }
                        (Some(address), None, None) if !address.is_empty() => Ok((None, address)),
                        _ => Err(CosmwasmUriError::Malformed("wrong path".to_string())),
                    }?;

                let queries = uri
                    .query_pairs()
                    .into_owned()
                    .collect::<HashMap<String, String>>();

                match queries.get(COSMWASM_QUERY_PARAM) {
                    Some(raw_query) => Ok(CosmwasmUri {
                        contract_name,
                        contract_address,
                        raw_query: raw_query.clone(),
                    }),
                    _ => Err(CosmwasmUriError::Malformed(
                        "missing 'query' query parameter".to_string(),
                    )),
                }
            })
    }
}

impl Display for CosmwasmUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let encoded_query = self.clone().encode_query();
        let str = match self.contract_name.clone() {
            Some(name) => [
                COSMWASM_SCHEME,
                ":",
                name.as_str(),
                ":",
                self.contract_address.as_str(),
                "?",
                encoded_query.as_str(),
            ]
            .join(""),
            _ => [
                COSMWASM_SCHEME,
                ":",
                self.contract_address.as_str(),
                "?",
                encoded_query.as_str(),
            ]
            .join(""),
        };
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use url::ParseError;

    #[test]
    fn proper_string_mappings() {
        let cases = vec![
            (
                CosmwasmUri{
                contract_name: Some("name".to_string()),
                contract_address: "address".to_string(),
                raw_query: "".to_string()
            },
             "cosmwasm:name:address?query=".to_string(),
            ),
            (
                CosmwasmUri{
                    contract_name: Some("name".to_string()),
                    contract_address: "address".to_string(),
                    raw_query: "{\"object_data\":{\"id\":\"1a88ca1632c7323c0aa594000cda26ed9f48b36351c29c3d1e35e0a0474e862e\"}}".to_string()
                },
                "cosmwasm:name:address?query=%7B%22object_data%22%3A%7B%22id%22%3A%221a88ca1632c7323c0aa594000cda26ed9f48b36351c29c3d1e35e0a0474e862e%22%7D%7D".to_string(),
            ),
            (
                CosmwasmUri{
                    contract_name: None,
                    contract_address: "address".to_string(),
                    raw_query: "\"data\"".to_string()
                },
                "cosmwasm:address?query=%22data%22".to_string(),
            ),
        ];

        for case in cases {
            assert_eq!(case.0.clone().to_string(), case.1);
            let res = CosmwasmUri::try_from(case.1);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), case.0);
        }
    }

    #[test]
    fn parse_uri_error() {
        let cases = vec![
            (
                "cosmwasm".to_string(),
                CosmwasmUriError::ParseURI(ParseError::RelativeUrlWithoutBase),
            ),
            (
                "cw:name:address?query=".to_string(),
                CosmwasmUriError::Malformed("wrong scheme".to_string()),
            ),
            (
                "cw:address?query=".to_string(),
                CosmwasmUriError::Malformed("wrong scheme".to_string()),
            ),
            (
                "cosmwasm:too_much:name:address?query=".to_string(),
                CosmwasmUriError::Malformed("wrong path".to_string()),
            ),
            (
                "cosmwasm:?query=".to_string(),
                CosmwasmUriError::Malformed("wrong path".to_string()),
            ),
            (
                "cosmwasm:name:address?".to_string(),
                CosmwasmUriError::Malformed("missing 'query' query parameter".to_string()),
            ),
            (
                "cosmwasm:name:address".to_string(),
                CosmwasmUriError::Malformed("missing 'query' query parameter".to_string()),
            ),
        ];

        for case in cases {
            let res = CosmwasmUri::try_from(case.0);
            assert!(res.is_err());
            assert_eq!(res.err().unwrap(), case.1);
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    struct TestQuery {
        pub content: String,
    }

    #[test]
    fn try_new() {
        let cases = vec![
            (
                Some("name".to_string()),
                "address".to_string(),
                TestQuery {
                    content: "content".to_string(),
                },
                "{\"content\":\"content\"}",
            ),
            (
                None,
                "address".to_string(),
                TestQuery {
                    content: "content".to_string(),
                },
                "{\"content\":\"content\"}",
            ),
        ];

        for case in cases {
            let res = CosmwasmUri::try_new(case.0.clone(), case.1.clone(), &case.2);

            assert!(res.is_ok());
            let uri = res.unwrap();
            assert_eq!(uri.contract_name, case.0);
            assert_eq!(uri.contract_address, case.1);
            assert_eq!(uri.raw_query, case.3);
        }
    }

    #[test]
    fn into_query() {
        let cases = vec![
            (
                CosmwasmUri {
                    contract_name: None,
                    contract_address: "address".to_string(),
                    raw_query: "{\"content\":\"content\"}".to_string(),
                },
                Ok(TestQuery {
                    content: "content".to_string(),
                }),
            ),
            (
                CosmwasmUri {
                    contract_name: None,
                    contract_address: "address".to_string(),
                    raw_query: "".to_string(),
                },
                Err(CosmwasmUriError::ParseQuery(
                    "EOF while parsing a JSON value.".to_string(),
                )),
            ),
        ];

        for case in cases {
            let res = case.0.into_query::<TestQuery>();
            assert_eq!(res, case.1);
        }
    }
}
