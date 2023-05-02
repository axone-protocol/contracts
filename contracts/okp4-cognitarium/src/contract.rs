use crate::contract::execute::insert;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{DataFormat, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Store, NAMESPACE_KEY_INCREMENT, STORE};

// version info for migration info
const CONTRACT_NAME: &str = concat!("crates.io:", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    STORE.save(deps.storage, &Store::new(info.sender, msg.limits.into()))?;
    NAMESPACE_KEY_INCREMENT.save(deps.storage, &0u128)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::InsertData { format, data } => {
            insert(deps, info, format.unwrap_or(DataFormat::Turtle), data)
        }
    }
}

pub mod execute {
    use super::*;
    use crate::msg::DataFormat;
    use crate::rdf::TripleReader;
    use crate::state::TripleStorer;
    use std::io::BufReader;

    pub fn insert(
        deps: DepsMut,
        info: MessageInfo,
        format: DataFormat,
        data: Binary,
    ) -> Result<Response, ContractError> {
        if STORE.load(deps.storage)?.owner != info.sender {
            Err(ContractError::Unauthorized)?
        }

        let buf = BufReader::new(data.as_slice());
        let mut reader = TripleReader::new(format, buf);
        let mut storer = TripleStorer::new(deps.storage)?;
        let count = storer.store_all(&mut reader)?;

        Ok(Response::new()
            .add_attribute("action", "insert")
            .add_attribute("triple_count", count))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Err(StdError::generic_err("Not implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::StoreError;
    use crate::msg::ExecuteMsg::InsertData;
    use crate::msg::{StoreLimitsInput, StoreLimitsInputBuilder};
    use crate::state;
    use crate::state::{namespaces, triples, Namespace};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Attribute, Order, Uint128};
    use std::env;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            limits: StoreLimitsInput {
                max_triple_count: Some(Uint128::from(1u128)),
                max_byte_size: Some(Uint128::from(2u128)),
                max_triple_byte_size: Some(Uint128::from(3u128)),
                max_query_limit: Some(4),
                max_query_variable_count: Some(5),
                max_insert_data_byte_size: Some(Uint128::from(6u128)),
                max_insert_data_triple_count: Some(Uint128::from(7u128)),
            },
        };

        let info = mock_info("owner", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        let store = STORE.load(&deps.storage).unwrap();
        assert_eq!(store.owner, info.sender);
        assert_eq!(
            store.limits,
            state::StoreLimits {
                max_triple_count: Uint128::from(1u128),
                max_byte_size: Uint128::from(2u128),
                max_triple_byte_size: Uint128::from(3u128),
                max_query_limit: 4,
                max_query_variable_count: 5,
                max_insert_data_byte_size: Uint128::from(6u128),
                max_insert_data_triple_count: Uint128::from(7u128),
            }
        );
        assert_eq!(
            store.stat,
            state::StoreStat {
                triple_count: Uint128::zero(),
                byte_size: Uint128::zero(),
            }
        );

        assert_eq!(NAMESPACE_KEY_INCREMENT.load(&deps.storage).unwrap(), 0u128);
    }

    #[test]
    fn proper_insert() {
        let cases = vec![
            InsertData {
                format: Some(DataFormat::RDFXml),
                data: read_test_data("sample.rdf.xml"),
            },
            InsertData {
                format: Some(DataFormat::Turtle),
                data: read_test_data("sample.ttl"),
            },
            InsertData {
                format: Some(DataFormat::NTriples),
                data: read_test_data("sample.nt"),
            },
            InsertData {
                format: None,
                data: read_test_data("sample.ttl"),
            },
        ];

        for case in cases {
            let mut deps = mock_dependencies();

            let info = mock_info("owner", &[]);
            instantiate(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                InstantiateMsg {
                    limits: StoreLimitsInput::default(),
                },
            )
            .unwrap();

            let res = execute(deps.as_mut(), mock_env(), info.clone(), case);

            assert!(res.is_ok());
            assert_eq!(
                res.unwrap().attributes,
                vec![
                    Attribute::new("action", "insert"),
                    Attribute::new("triple_count", "40")
                ]
            );

            assert_eq!(
                triples()
                    .range_raw(&deps.storage, None, None, Order::Ascending)
                    .count(),
                40
            );
            assert_eq!(
                STORE.load(&deps.storage).unwrap().stat.triple_count,
                Uint128::from(40u128),
            );
            assert_eq!(NAMESPACE_KEY_INCREMENT.load(&deps.storage).unwrap(), 17u128);
            assert_eq!(
                namespaces()
                    .load(
                        &deps.storage,
                        "https://ontology.okp4.space/dataverse/dataspace/".to_string()
                    )
                    .unwrap(),
                Namespace {
                    key: 0u128,
                    counter: 5u128,
                }
            )
        }
    }

    #[test]
    fn insert_unauthorized() {
        let mut deps = mock_dependencies();
        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("owner", &[]),
            InstantiateMsg {
                limits: StoreLimitsInput::default(),
            },
        )
        .unwrap();

        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("not-owner", &[]),
            InsertData {
                format: Some(DataFormat::RDFXml),
                data: read_test_data("sample.rdf.xml"),
            },
        );
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), ContractError::Unauthorized);
    }

    #[test]
    fn insert_limits() {
        let cases = vec![
            (
                StoreLimitsInputBuilder::default()
                    .max_triple_count(30u128)
                    .build()
                    .unwrap(),
                Some(ContractError::from(StoreError::TripleCount(30u128.into()))),
            ),
            (
                StoreLimitsInputBuilder::default()
                    .max_triple_count(40u128)
                    .build()
                    .unwrap(),
                None,
            ),
            (
                StoreLimitsInputBuilder::default()
                    .max_byte_size(50u128)
                    .build()
                    .unwrap(),
                Some(ContractError::from(StoreError::ByteSize(50u128.into()))),
            ),
            (
                StoreLimitsInputBuilder::default()
                    .max_byte_size(50000u128)
                    .build()
                    .unwrap(),
                None,
            ),
            (
                StoreLimitsInputBuilder::default()
                    .max_insert_data_byte_size(500u128)
                    .build()
                    .unwrap(),
                Some(ContractError::from(StoreError::InsertDataByteSize(
                    500u128.into(),
                ))),
            ),
            (
                StoreLimitsInputBuilder::default()
                    .max_insert_data_byte_size(50000u128)
                    .build()
                    .unwrap(),
                None,
            ),
            (
                StoreLimitsInputBuilder::default()
                    .max_triple_byte_size(150u128)
                    .build()
                    .unwrap(),
                Some(ContractError::from(StoreError::TripleByteSize(
                    176u128.into(),
                    150u128.into(),
                ))),
            ),
            (
                StoreLimitsInputBuilder::default()
                    .max_triple_byte_size(400u128)
                    .build()
                    .unwrap(),
                None,
            ),
            (
                StoreLimitsInputBuilder::default()
                    .max_insert_data_triple_count(30u128)
                    .build()
                    .unwrap(),
                Some(ContractError::from(StoreError::InsertDataTripleCount(
                    30u128.into(),
                ))),
            ),
            (
                StoreLimitsInputBuilder::default()
                    .max_insert_data_triple_count(40u128)
                    .build()
                    .unwrap(),
                None,
            ),
        ];

        let exec_msg = InsertData {
            format: Some(DataFormat::RDFXml),
            data: read_test_data("sample.rdf.xml"),
        };
        for case in cases {
            let mut deps = mock_dependencies();

            let info = mock_info("owner", &[]);
            instantiate(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                InstantiateMsg { limits: case.0 },
            )
            .unwrap();

            let res = execute(deps.as_mut(), mock_env(), info.clone(), exec_msg.clone());

            if let Some(err) = case.1 {
                assert!(res.is_err());
                assert_eq!(res.err().unwrap(), err);
            } else {
                assert!(res.is_ok());
            }
        }
    }

    fn read_test_data(file: &str) -> Binary {
        let mut bytes: Vec<u8> = Vec::new();

        File::open(
            Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
                .join("testdata")
                .join(file),
        )
        .unwrap()
        .read_to_end(&mut bytes)
        .unwrap();

        Binary::from(bytes)
    }
}
