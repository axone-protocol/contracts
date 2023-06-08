use crate::contract::execute::insert;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
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
        _ => Err(StdError::generic_err("Not implemented").into()),
    }
}

pub mod execute {
    use super::*;
    use crate::msg::DataFormat;
    use crate::rdf::TripleReader;
    use crate::storer::TripleStorer;
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
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Store => to_binary(&query::store(deps)?),
        QueryMsg::Select { query } => to_binary(&query::select(deps, query)?),
        _ => Err(StdError::generic_err("Not implemented")),
    }
}

pub mod query {
    use super::*;
    use crate::msg::{SelectQuery, SelectResponse, StoreResponse};
    use crate::querier::{PlanBuilder, QueryEngine};

    pub fn store(deps: Deps) -> StdResult<StoreResponse> {
        STORE.load(deps.storage).map(|s| s.into())
    }

    pub fn select(deps: Deps, query: SelectQuery) -> StdResult<SelectResponse> {
        let store = STORE.load(deps.storage)?;

        if query.select.len() > store.limits.max_query_variable_count as usize {
            Err(StdError::generic_err(
                "Maximum query variable count exceeded",
            ))?
        }

        let count = query.limit.unwrap_or(store.limits.max_query_limit);
        if count > store.limits.max_query_limit {
            Err(StdError::generic_err("Maximum query limit exceeded"))?
        }

        let mut plan_builder =
            PlanBuilder::new(deps.storage, query.prefixes).with_limit(count as usize);

        QueryEngine::new(deps.storage).select(plan_builder.build_plan(query.r#where)?, query.select)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::StoreError;
    use crate::msg::ExecuteMsg::InsertData;
    use crate::msg::Node::NamedNode;
    use crate::msg::SimpleWhereCondition::TriplePattern;
    use crate::msg::IRI::{Full, Prefixed};
    use crate::msg::{
        Head, Literal, Prefix, Results, SelectItem, SelectQuery, SelectResponse, StoreLimitsInput,
        StoreLimitsInputBuilder, StoreResponse, Value, VarOrNode, VarOrNodeOrLiteral,
        WhereCondition,
    };
    use crate::state::{
        namespaces, triples, Namespace, Node, Object, StoreLimits, StoreStat, Subject, Triple,
    };
    use crate::{msg, state};
    use blake3::Hash;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, Addr, Attribute, Order, Uint128};
    use std::collections::BTreeMap;
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
            StoreStat {
                triple_count: Uint128::zero(),
                namespace_count: Uint128::zero(),
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
                format: Some(DataFormat::NQuads),
                data: read_test_data("sample.nq"),
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
                STORE.load(&deps.storage).unwrap().stat,
                StoreStat {
                    triple_count: 40u128.into(),
                    namespace_count: 17u128.into(),
                    byte_size: 7103u128.into(),
                },
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
                    value: "https://ontology.okp4.space/dataverse/dataspace/".to_string(),
                    key: 0u128,
                    counter: 5u128,
                }
            );
            assert_eq!(
                triples()
                    .load(
                        &deps.storage,
                        (
                            Hash::from_hex(
                                "09653b5306fa80dc7bea8313d84ac6ed9ded591d42c7f4838c39d1d7a4f09d03"
                            )
                            .unwrap()
                            .as_bytes(),
                            Node {
                                namespace: 3u128,
                                value: "hasRegistrar".to_string()
                            }
                            .key(),
                            Subject::Named(Node {
                                namespace: 0u128,
                                value: "97ff7e16-c08d-47be-8475-211016c82e33".to_string()
                            })
                            .key()
                        )
                    )
                    .unwrap(),
                Triple {
                    object: Object::Named(Node {
                        namespace: 4u128,
                        value: "0x04d1f1b8f8a7a28f9a5a254c326a963a22f5a5b5d5f5e5d5c5b5a5958575655"
                            .to_string()
                    }),
                    predicate: Node {
                        namespace: 3u128,
                        value: "hasRegistrar".to_string()
                    },
                    subject: Subject::Named(Node {
                        namespace: 0u128,
                        value: "97ff7e16-c08d-47be-8475-211016c82e33".to_string()
                    }),
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

    #[test]
    fn proper_store() {
        let mut deps = mock_dependencies();
        STORE
            .save(
                deps.as_mut().storage,
                &Store {
                    owner: Addr::unchecked("owner"),
                    limits: StoreLimits {
                        max_triple_count: 1u128.into(),
                        max_byte_size: 2u128.into(),
                        max_triple_byte_size: 3u128.into(),
                        max_query_limit: 4u32,
                        max_query_variable_count: 5u32,
                        max_insert_data_byte_size: 6u128.into(),
                        max_insert_data_triple_count: 7u128.into(),
                    },
                    stat: StoreStat {
                        triple_count: 1u128.into(),
                        namespace_count: 2u128.into(),
                        byte_size: 3u128.into(),
                    },
                },
            )
            .unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Store);
        assert!(res.is_ok());
        assert_eq!(
            from_binary::<StoreResponse>(&res.unwrap()).unwrap(),
            StoreResponse {
                owner: "owner".to_string(),
                limits: msg::StoreLimits {
                    max_triple_count: 1u128.into(),
                    max_byte_size: 2u128.into(),
                    max_triple_byte_size: 3u128.into(),
                    max_query_limit: 4u32,
                    max_query_variable_count: 5u32,
                    max_insert_data_byte_size: 6u128.into(),
                    max_insert_data_triple_count: 7u128.into(),
                },
                stat: msg::StoreStat {
                    triple_count: 1u128.into(),
                    namespace_count: 2u128.into(),
                    byte_size: 3u128.into(),
                }
            }
        );
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

    #[test]
    fn proper_select() {
        let cases = vec![
            (
                SelectQuery {
                    prefixes: vec![],
                    select: vec![
                        SelectItem::Variable("a".to_string()),
                        SelectItem::Variable("b".to_string()),
                    ],
                    r#where: vec![WhereCondition::Simple(TriplePattern(
                        msg::TriplePattern {
                            subject: VarOrNode::Variable("a".to_string()),
                            predicate: VarOrNode::Node(NamedNode(Full(
                                "https://ontology.okp4.space/core/hasDescription".to_string(),
                            ))),
                            object: VarOrNodeOrLiteral::Variable("b".to_string()),
                        },
                    ))],
                    limit: None,
                },
                SelectResponse {
                    head: Head {
                        vars: vec!["a".to_string(), "b".to_string()],
                    },
                    results: Results {
                        bindings: vec![
                            BTreeMap::from([
                                (
                                    "a".to_string(),
                                    Value::URI {
                                        value: Full("https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473".to_string())
                                    }
                                ),
                                (
                                    "b".to_string(),
                                    Value::Literal {
                                        value: "A test Data Space.".to_string(),
                                        lang: Some("en".to_string()),
                                        datatype: None,
                                    }
                                )
                            ]),
                            BTreeMap::from([
                                (
                                    "a".to_string(),
                                    Value::URI {
                                        value: Full("https://ontology.okp4.space/dataverse/dataset/metadata/d1615703-4ee1-4e2f-997e-15aecf1eea4e".to_string())
                                    }
                                ),
                                (
                                    "b".to_string(),
                                    Value::Literal {
                                        value: "Un Dataset de test.".to_string(),
                                        lang: Some("fr".to_string()),
                                        datatype: None,
                                    }
                                )
                            ]),
                            BTreeMap::from([
                                (
                                    "a".to_string(),
                                    Value::URI {
                                        value: Full("https://ontology.okp4.space/dataverse/dataset/metadata/d1615703-4ee1-4e2f-997e-15aecf1eea4e".to_string())
                                    }
                                ),
                                (
                                    "b".to_string(),
                                    Value::Literal {
                                        value: "A test Dataset.".to_string(),
                                        lang: Some("en".to_string()),
                                        datatype: None,
                                    }
                                )
                            ]),
                            BTreeMap::from([
                                (
                                    "a".to_string(),
                                    Value::URI {
                                        value: Full("https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473".to_string())
                                    }
                                ),
                                (
                                    "b".to_string(),
                                    Value::Literal {
                                        value: "Un Data Space de test.".to_string(),
                                        lang: Some("fr".to_string()),
                                        datatype: None,
                                    }
                                )
                            ])
                        ],
                    },
                },
            ),
            (
                SelectQuery {
                    prefixes: vec![Prefix { prefix: "core".to_string(), namespace: "https://ontology.okp4.space/core/".to_string() }],
                    select: vec![
                     SelectItem::Variable("a".to_string()),
                    ],
                    r#where: vec![WhereCondition::Simple(TriplePattern(
                     msg::TriplePattern {
                         subject: VarOrNode::Variable("a".to_string()),
                         predicate: VarOrNode::Node(NamedNode(Prefixed(
                             "core:hasDescription".to_string(),
                         ))),
                         object: VarOrNodeOrLiteral::Literal(Literal::LanguageTaggedString { value: "A test Dataset.".to_string(), language: "en".to_string() }),
                     },
                    ))],
                    limit: None,
                },
                SelectResponse {
                 head: Head {
                     vars: vec!["a".to_string()],
                 },
                 results: Results {
                     bindings: vec![
                         BTreeMap::from([
                             (
                                 "a".to_string(),
                                 Value::URI {
                                     value: Full("https://ontology.okp4.space/dataverse/dataset/metadata/d1615703-4ee1-4e2f-997e-15aecf1eea4e".to_string())
                                 }
                             ),
                         ])
                     ],
                 },
                },
            ),
            (
                SelectQuery {
                    prefixes: vec![],
                    select: vec![
                        SelectItem::Variable("a".to_string()),
                    ],
                    r#where: vec![WhereCondition::Simple(TriplePattern(
                        msg::TriplePattern {
                            subject: VarOrNode::Node(NamedNode(Full("https://ontology.okp4.space/dataverse/dataset/metadata/d1615703-4ee1-4e2f-997e-15aecf1eea4e".to_string()))),
                            predicate: VarOrNode::Variable("a".to_string()),
                            object: VarOrNodeOrLiteral::Literal(Literal::LanguageTaggedString { value: "A test Dataset.".to_string(), language: "en".to_string() }),
                        },
                    ))],
                    limit: None,
                },
                SelectResponse {
                    head: Head {
                        vars: vec!["a".to_string()],
                    },
                    results: Results {
                        bindings: vec![
                            BTreeMap::from([
                                (
                                    "a".to_string(),
                                    Value::URI {
                                        value: Full("https://ontology.okp4.space/core/hasDescription".to_string())
                                    }
                                ),
                            ])
                        ],
                    },
                },
            )
        ];

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

        execute(
            deps.as_mut(),
            mock_env(),
            info,
            InsertData {
                format: Some(DataFormat::RDFXml),
                data: read_test_data("sample.rdf.xml"),
            },
        )
        .unwrap();

        for (q, expected) in cases {
            let res = query(deps.as_ref(), mock_env(), QueryMsg::Select { query: q });
            assert!(res.is_ok());

            let result = from_binary::<SelectResponse>(&res.unwrap()).unwrap();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn invalid_select() {
        let cases = vec![
            (
                SelectQuery {
                    prefixes: vec![],
                    select: vec![
                        SelectItem::Variable("a".to_string()),
                        SelectItem::Variable("b".to_string()),
                    ],
                    r#where: vec![],
                    limit: None,
                },
                Err(StdError::generic_err(
                    "Maximum query variable count exceeded",
                )),
            ),
            (
                SelectQuery {
                    prefixes: vec![],
                    select: vec![],
                    r#where: vec![],
                    limit: Some(8000),
                },
                Err(StdError::generic_err("Maximum query limit exceeded")),
            ),
            (
                SelectQuery {
                    prefixes: vec![Prefix {
                        prefix: "core".to_string(),
                        namespace: "https://ontology.okp4.space/core/".to_string(),
                    }],
                    select: vec![SelectItem::Variable("a".to_string())],
                    r#where: vec![WhereCondition::Simple(TriplePattern(msg::TriplePattern {
                        subject: VarOrNode::Variable("a".to_string()),
                        predicate: VarOrNode::Node(NamedNode(Prefixed(
                            "invalid:hasDescription".to_string(),
                        ))),
                        object: VarOrNodeOrLiteral::Literal(Literal::LanguageTaggedString {
                            value: "A test Dataset.".to_string(),
                            language: "en".to_string(),
                        }),
                    }))],
                    limit: None,
                },
                Err(StdError::generic_err(
                    "Malformed prefixed IRI: prefix not found",
                )),
            ),
            (
                SelectQuery {
                    prefixes: vec![],
                    select: vec![SelectItem::Variable("u".to_string())],
                    r#where: vec![WhereCondition::Simple(TriplePattern(msg::TriplePattern {
                        subject: VarOrNode::Variable("a".to_string()),
                        predicate: VarOrNode::Node(NamedNode(Full(
                            "https://ontology.okp4.space/core/hasDescription".to_string(),
                        ))),
                        object: VarOrNodeOrLiteral::Literal(Literal::LanguageTaggedString {
                            value: "A test Dataset.".to_string(),
                            language: "en".to_string(),
                        }),
                    }))],
                    limit: None,
                },
                Err(StdError::generic_err(
                    "Selected variable not found in query",
                )),
            ),
        ];

        let mut deps = mock_dependencies();

        let info = mock_info("owner", &[]);
        instantiate(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            InstantiateMsg {
                limits: StoreLimitsInput {
                    max_query_variable_count: Some(1),
                    ..Default::default()
                },
            },
        )
        .unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            info,
            InsertData {
                format: Some(DataFormat::RDFXml),
                data: read_test_data("sample.rdf.xml"),
            },
        )
        .unwrap();

        for (q, expected) in cases {
            let res = query(deps.as_ref(), mock_env(), QueryMsg::Select { query: q });
            assert_eq!(res, expected);
        }
    }
}
