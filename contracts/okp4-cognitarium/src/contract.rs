use crate::contract::execute::insert;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
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
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::InsertData { input } => insert(deps, input),
    }
}

pub mod execute {
    use super::*;
    use crate::error::StoreError;
    use crate::msg::DataInput;
    use crate::rdf;
    use crate::rdf::NSResolveFn;
    use crate::state::{namespaces, triples, Namespace, NAMESPACE_KEY_INCREMENT};
    use blake3::Hash;
    use cosmwasm_std::Storage;
    use std::collections::BTreeMap;

    pub fn insert(deps: DepsMut, graph: DataInput) -> Result<Response, ContractError> {
        let mut store = STORE.load(deps.storage)?;

        let old_count = store.stat.triples_count;
        let mut ns_key_inc = NAMESPACE_KEY_INCREMENT.load(deps.storage)?;
        let mut ns_cache: BTreeMap<String, Namespace> = BTreeMap::new();

        let mut triple_reader = rdf::read_triples(&graph);

        loop {
            let next = triple_reader.next(&mut ns_resolver(
                deps.storage,
                &mut ns_key_inc,
                &mut ns_cache,
            ));

            match next {
                None => {
                    break;
                }
                Some(res) => {
                    let triple = res.map_err(ContractError::from)?;
                    store.stat.triples_count += Uint128::one();

                    if store.stat.triples_count > store.limits.max_triple_count {
                        Err(ContractError::from(StoreError::MaxTriplesLimitExceeded(
                            store.limits.max_triple_count,
                        )))?
                    }

                    let object_hash: Hash = triple.object.as_hash();
                    triples()
                        .save(
                            deps.storage,
                            (
                                object_hash.as_bytes(),
                                triple.predicate.clone(),
                                triple.subject.clone(),
                            ),
                            &triple,
                        )
                        .map_err(ContractError::Std)?;
                }
            }
        }

        STORE.save(deps.storage, &store)?;
        NAMESPACE_KEY_INCREMENT.save(deps.storage, &ns_key_inc)?;
        for entry in ns_cache {
            namespaces().save(deps.storage, entry.0, &entry.1)?;
        }

        Ok(Response::new()
            .add_attribute("action", "insert")
            .add_attribute("triple_count", store.stat.triples_count - old_count))
    }

    fn ns_resolver<'a>(
        store: &'a dyn Storage,
        ns_key_inc: &'a mut u128,
        ns_cache: &'a mut BTreeMap<String, Namespace>,
    ) -> NSResolveFn<'a> {
        Box::new(|ns_str| -> Result<u128, StdError> {
            match ns_cache.get_mut(ns_str.as_str()) {
                Some(namespace) => {
                    namespace.counter += 1;
                    Ok(namespace.key)
                }
                None => {
                    let mut namespace = match namespaces().load(store, ns_str.clone()) {
                        Err(StdError::NotFound { .. }) => {
                            let n = Namespace {
                                key: *ns_key_inc,
                                counter: 0u128,
                            };
                            *ns_key_inc += 1;
                            Ok(n)
                        }
                        Ok(n) => Ok(n),
                        Err(e) => Err(e),
                    }?;

                    namespace.counter += 1;
                    ns_cache.insert(ns_str.clone(), namespace.clone());
                    Ok(namespace.key)
                }
            }
        })
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
    use crate::msg::{DataInput, StoreLimitsInput, StoreLimitsInputBuilder};
    use crate::state;
    use crate::state::{namespaces, triples, Namespace};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Attribute, Order};
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
                triples_count: Uint128::zero(),
            }
        );

        assert_eq!(NAMESPACE_KEY_INCREMENT.load(&deps.storage).unwrap(), 0u128);
    }

    #[test]
    fn proper_insert() {
        let cases = vec![
            DataInput::RDFXml(read_test_data("sample.rdf.xml")),
            DataInput::Turtle(read_test_data("sample.ttl")),
            DataInput::NTriples(read_test_data("sample.nt")),
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

            let res = execute(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                ExecuteMsg::InsertData { input: case },
            );

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
                STORE.load(&deps.storage).unwrap().stat.triples_count,
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
            ExecuteMsg::InsertData {
                input: DataInput::RDFXml(Binary::from(&[])),
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
                Some(ContractError::from(StoreError::MaxTriplesLimitExceeded(
                    30u128.into(),
                ))),
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
                Some(ContractError::from(StoreError::MaxByteSize(50u128.into()))),
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
                    .max_insert_data_byte_size(50u128)
                    .build()
                    .unwrap(),
                Some(ContractError::from(StoreError::MaxInsertDataByteSize(
                    50u128.into(),
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
                    .max_insert_data_triple_count(30u128)
                    .build()
                    .unwrap(),
                Some(ContractError::from(StoreError::MaxInsertDataTripleCount(
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

        let exec_msg = ExecuteMsg::InsertData {
            input: DataInput::RDFXml(read_test_data("sample.rdf.xml")),
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
