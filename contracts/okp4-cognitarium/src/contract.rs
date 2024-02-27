#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{DataFormat, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Store, BLANK_NODE_IDENTIFIER_COUNTER, NAMESPACE_KEY_INCREMENT, STORE};

// version info for migration info
const CONTRACT_NAME: &str = concat!("crates.io:", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<'_>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    STORE.save(deps.storage, &Store::new(info.sender, msg.limits.into()))?;
    NAMESPACE_KEY_INCREMENT.save(deps.storage, &0u128)?;
    BLANK_NODE_IDENTIFIER_COUNTER.save(deps.storage, &0u128)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<'_>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::InsertData { format, data } => {
            execute::insert(deps, info, format.unwrap_or_default(), data)
        }
        ExecuteMsg::DeleteData {
            prefixes,
            delete,
            r#where,
        } => execute::delete(deps, info, prefixes, delete, r#where),
    }
}

pub mod execute {
    use super::*;
    use crate::msg::{DataFormat, HasVariables, Prefix, TripleDeleteTemplate, WhereClause};
    use crate::querier::{PlanBuilder, QueryEngine};
    use crate::rdf::PrefixMap;
    use crate::state::HasCachedNamespaces;
    use crate::storer::StoreEngine;
    use cosmwasm_std::Uint128;
    use okp4_rdf::serde::TripleReader;
    use std::io::BufReader;

    pub fn verify_owner(deps: &DepsMut<'_>, info: &MessageInfo) -> Result<(), ContractError> {
        if STORE.load(deps.storage)?.owner != info.sender {
            Err(ContractError::Unauthorized)
        } else {
            Ok(())
        }
    }

    pub fn insert(
        deps: DepsMut<'_>,
        info: MessageInfo,
        format: DataFormat,
        data: Binary,
    ) -> Result<Response, ContractError> {
        verify_owner(&deps, &info)?;

        let buf = BufReader::new(data.as_slice());
        let mut reader = TripleReader::new(&(&format).into(), buf);
        let mut storer = StoreEngine::new(deps.storage)?;
        let count = storer.store_all(&mut reader)?;

        Ok(Response::new()
            .add_attribute("action", "insert")
            .add_attribute("triple_count", count))
    }

    pub fn delete(
        deps: DepsMut<'_>,
        info: MessageInfo,
        prefixes: Vec<Prefix>,
        delete: Vec<TripleDeleteTemplate>,
        r#where: WhereClause,
    ) -> Result<Response, ContractError> {
        verify_owner(&deps, &info)?;

        if delete.is_empty() {
            return Ok(Response::new()
                .add_attribute("action", "delete")
                .add_attribute("triple_count", Uint128::from(0u128)));
        }

        let prefix_map = <PrefixMap>::from(prefixes).into_inner();
        let mut plan_builder = PlanBuilder::new(deps.storage, &prefix_map, None);
        let plan = plan_builder.build_plan(&r#where)?;

        let triples = QueryEngine::new(deps.storage)
            .select(plan, delete.as_select_item())?
            .solutions
            .resolve_triples(
                deps.storage,
                &prefix_map,
                delete
                    .into_iter()
                    .map(|t| (t.subject, t.predicate, t.object))
                    .collect(),
                plan_builder.cached_namespaces(),
            )?;

        let mut store = StoreEngine::new(deps.storage)?;
        let count = store.delete_all(&triples)?;

        Ok(Response::new()
            .add_attribute("action", "delete")
            .add_attribute("triple_count", count))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<'_>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Store => to_json_binary(&query::store(deps)?),
        QueryMsg::Select { query } => to_json_binary(&query::select(deps, query)?),
        QueryMsg::Describe { query, format } => {
            to_json_binary(&query::describe(deps, query, format.unwrap_or_default())?)
        }
        QueryMsg::Construct { query, format } => to_json_binary(&query::construct(
            deps,
            query,
            format.unwrap_or(DataFormat::default()),
        )?),
    }
}

pub mod query {
    use super::*;
    use crate::msg::{
        ConstructQuery, ConstructResponse, DescribeQuery, DescribeResponse, HasVariables, Node,
        SelectQuery, SelectResponse, SimpleWhereCondition, StoreResponse, TriplePattern,
        VarOrNamedNode, VarOrNode, VarOrNodeOrLiteral, WhereCondition,
    };
    use crate::querier::{PlanBuilder, QueryEngine};
    use crate::rdf::PrefixMap;
    use crate::state::HasCachedNamespaces;
    use okp4_rdf::serde::TripleWriter;

    pub fn store(deps: Deps<'_>) -> StdResult<StoreResponse> {
        STORE.load(deps.storage).map(Into::into)
    }

    pub fn select(deps: Deps<'_>, query: SelectQuery) -> StdResult<SelectResponse> {
        let store = STORE.load(deps.storage)?;

        if query.select.len() > store.limits.max_query_variable_count as usize {
            Err(StdError::generic_err(
                "Maximum query variable count exceeded",
            ))?;
        }

        let count = query.limit.unwrap_or(store.limits.max_query_limit);
        if count > store.limits.max_query_limit {
            Err(StdError::generic_err("Maximum query limit exceeded"))?;
        }

        let prefix_map = PrefixMap::from(query.prefixes).into_inner();
        let mut plan_builder =
            PlanBuilder::new(deps.storage, &prefix_map, None).with_limit(count as usize);
        let plan = plan_builder.build_plan(&query.r#where)?;

        QueryEngine::new(deps.storage)
            .select(plan, query.select)
            .and_then(|res| util::map_select_solutions(deps, res, plan_builder.cached_namespaces()))
    }

    pub fn describe(
        deps: Deps<'_>,
        query: DescribeQuery,
        format: DataFormat,
    ) -> StdResult<DescribeResponse> {
        let (p, o) = ("_2p".to_owned(), "_3o".to_owned());

        let store = STORE.load(deps.storage)?;

        let (select, r#where) = match &query.resource {
            VarOrNamedNode::Variable(var) => {
                let select = TriplePattern {
                    subject: VarOrNode::Variable(var.clone()),
                    predicate: VarOrNamedNode::Variable(format!("{var}{p}")),
                    object: VarOrNodeOrLiteral::Variable(format!("{var}{o}")),
                };

                let mut r#where = query.r#where;
                r#where.push(WhereCondition::Simple(SimpleWhereCondition::TriplePattern(
                    select.clone(),
                )));

                (vec![select], r#where)
            }
            VarOrNamedNode::NamedNode(iri) => {
                let select = TriplePattern {
                    subject: VarOrNode::Node(Node::NamedNode(iri.clone())),
                    predicate: VarOrNamedNode::Variable(p),
                    object: VarOrNodeOrLiteral::Variable(o),
                };

                (
                    vec![select.clone()],
                    vec![WhereCondition::Simple(SimpleWhereCondition::TriplePattern(
                        select,
                    ))],
                )
            }
        };
        let prefix_map = <PrefixMap>::from(query.prefixes).into_inner();
        let mut plan_builder = PlanBuilder::new(deps.storage, &prefix_map, None)
            .with_limit(store.limits.max_query_limit as usize);
        let plan = plan_builder.build_plan(&r#where)?;

        let atoms = QueryEngine::new(deps.storage)
            .select(plan, select.as_select_item())
            .and_then(|res| {
                res.solutions.resolve_atoms(
                    deps.storage,
                    &prefix_map,
                    select
                        .into_iter()
                        .map(|p| (p.subject, p.predicate, p.object))
                        .collect(),
                    plan_builder.cached_namespaces(),
                )
            })?;

        let out: Vec<u8> = Vec::default();
        let mut writer = TripleWriter::new(&(&format).into(), out);

        for atom in &atoms {
            let triple = atom.into();

            writer.write(&triple).map_err(|e| {
                StdError::serialize_err(
                    "triple",
                    format!("Error writing triple {}: {}", &triple, e),
                )
            })?;
        }
        let out = writer
            .finish()
            .map_err(|e| StdError::serialize_err("triple", format!("Error writing triple: {e}")))?;

        Ok(DescribeResponse {
            format,
            data: Binary::from(out),
        })
    }

    pub fn construct(
        deps: Deps<'_>,
        query: ConstructQuery,
        format: DataFormat,
    ) -> StdResult<ConstructResponse> {
        let ConstructQuery {
            construct,
            prefixes,
            r#where,
        } = query;
        if construct.is_empty() {
            return Ok(ConstructResponse {
                format,
                data: Binary::from(vec![]),
            });
        }

        let prefix_map = <PrefixMap>::from(prefixes).into_inner();
        let mut plan_builder = PlanBuilder::new(deps.storage, &prefix_map, None);
        let plan = plan_builder.build_plan(&r#where)?;

        let atoms = QueryEngine::new(deps.storage)
            .select(plan, construct.as_select_item())
            .and_then(|res| {
                res.solutions.resolve_atoms(
                    deps.storage,
                    &prefix_map,
                    construct
                        .into_iter()
                        .map(|t| (t.subject, t.predicate, t.object))
                        .collect(),
                    plan_builder.cached_namespaces(),
                )
            })?;

        let out: Vec<u8> = Vec::default();
        let mut writer = TripleWriter::new(&(&format).into(), out);

        for atom in &atoms {
            let triple = atom.into();
            writer.write(&triple).map_err(|e| {
                StdError::serialize_err(
                    "triple",
                    format!("Error writing triple {}: {}", &triple, e),
                )
            })?;
        }

        let out = writer
            .finish()
            .map_err(|e| StdError::serialize_err("triple", format!("Error writing triple: {e}")))?;

        Ok(ConstructResponse {
            format,
            data: Binary::from(out),
        })
    }
}

pub mod util {
    use super::*;
    use crate::msg::{Head, Results, SelectResponse, Value};
    use crate::querier::SelectResults;
    use crate::state::{Namespace, NamespaceResolver};
    use okp4_rdf::normalize::IdentifierIssuer;
    use std::collections::BTreeMap;

    pub fn map_select_solutions(
        deps: Deps<'_>,
        res: SelectResults<'_>,
        ns_cache: Vec<Namespace>,
    ) -> StdResult<SelectResponse> {
        let mut ns_resolver: NamespaceResolver = ns_cache.into();
        let mut id_issuer = IdentifierIssuer::new("b", 0u128);

        let mut bindings: Vec<BTreeMap<String, Value>> = vec![];
        for solution in res.solutions {
            let vars = solution?;
            let resolved = vars
                .into_iter()
                .map(|(name, var)| -> StdResult<(String, Value)> {
                    Ok((
                        name,
                        var.as_value(
                            &mut |ns_key| {
                                let res = ns_resolver.resolve_from_key(deps.storage, ns_key);
                                res.and_then(NamespaceResolver::none_as_error_middleware)
                                    .map(|ns| ns.value)
                            },
                            &mut id_issuer,
                        )?,
                    ))
                })
                .collect::<StdResult<BTreeMap<String, Value>>>()?;
            bindings.push(resolved);
        }

        Ok(SelectResponse {
            head: Head { vars: res.head },
            results: Results { bindings },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::StoreError;
    use crate::msg::ExecuteMsg::{DeleteData, InsertData};
    use crate::msg::Node::{BlankNode, NamedNode};
    use crate::msg::SimpleWhereCondition::TriplePattern;
    use crate::msg::IRI::{Full, Prefixed};
    use crate::msg::{
        ConstructQuery, ConstructResponse, DescribeQuery, DescribeResponse, Head, Literal, Prefix,
        Results, SelectItem, SelectQuery, SelectResponse, StoreLimitsInput,
        StoreLimitsInputBuilder, StoreResponse, Value, VarOrNamedNode, VarOrNamedNodeOrLiteral,
        VarOrNode, VarOrNodeOrLiteral, WhereCondition,
    };
    use crate::state::{
        namespaces, triples, Namespace, Node, Object, StoreLimits, StoreStat, Subject, Triple,
    };
    use crate::{msg, state};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_json, Addr, Attribute, Order, Uint128};
    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    use std::{env, u128};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            limits: StoreLimitsInput {
                max_triple_count: Uint128::from(1u128),
                max_byte_size: Uint128::from(2u128),
                max_triple_byte_size: Uint128::from(3u128),
                max_query_limit: 4,
                max_query_variable_count: 5,
                max_insert_data_byte_size: Uint128::from(6u128),
                max_insert_data_triple_count: Uint128::from(7u128),
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
        assert_eq!(
            BLANK_NODE_IDENTIFIER_COUNTER.load(&deps.storage).unwrap(),
            0u128
        );
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
                InstantiateMsg::default(),
            )
            .unwrap();

            let res = execute(deps.as_mut(), mock_env(), info.clone(), case);

            assert!(res.is_ok());
            assert_eq!(
                res.unwrap().attributes,
                vec![
                    Attribute::new("action", "insert"),
                    Attribute::new("triple_count", "40"),
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
                        "https://ontology.okp4.space/dataverse/dataspace/".to_string(),
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
                            Object::Named(Node {
                                namespace: 4u128,
                                value: "0x04d1f1b8f8a7a28f9a5a254c326a963a22f5a5b5d5f5e5d5c5b5a5958575655"
                                    .to_string(),
                            }).as_hash()
                                .as_bytes(),
                            Node {
                                namespace: 3u128,
                                value: "hasRegistrar".to_string(),
                            }
                                .key(),
                            Subject::Named(Node {
                                namespace: 0u128,
                                value: "97ff7e16-c08d-47be-8475-211016c82e33".to_string(),
                            })
                                .key()
                        ),
                    )
                    .unwrap(),
                Triple {
                    object: Object::Named(Node {
                        namespace: 4u128,
                        value: "0x04d1f1b8f8a7a28f9a5a254c326a963a22f5a5b5d5f5e5d5c5b5a5958575655"
                            .to_string(),
                    }),
                    predicate: Node {
                        namespace: 3u128,
                        value: "hasRegistrar".to_string(),
                    },
                    subject: Subject::Named(Node {
                        namespace: 0u128,
                        value: "97ff7e16-c08d-47be-8475-211016c82e33".to_string(),
                    }),
                }
            )
        }
    }

    #[test]
    fn insert_existing_triples() {
        let mut deps = mock_dependencies();

        let info = mock_info("owner", &[]);
        instantiate(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            InstantiateMsg::default(),
        )
        .unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            InsertData {
                format: Some(DataFormat::RDFXml),
                data: read_test_data("sample.rdf.xml"),
            },
        )
        .unwrap();

        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            InsertData {
                format: Some(DataFormat::RDFXml),
                data: read_test_data("sample.rdf.xml"),
            },
        );

        assert!(res.is_ok());
        assert_eq!(
            res.unwrap().attributes,
            vec![
                Attribute::new("action", "insert"),
                Attribute::new("triple_count", "0"),
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
    }

    #[test]
    fn insert_unauthorized() {
        let mut deps = mock_dependencies();
        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("owner", &[]),
            InstantiateMsg::default(),
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
    fn proper_delete() {
        let id = "https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473";
        let cases = vec![
            (
                DeleteData {
                    prefixes: vec![],
                    delete: vec![msg::TripleDeleteTemplate {
                        subject: VarOrNamedNode::NamedNode(Full(
                            "https://ontology.okp4.space/dataverse/dataspace/metadata/unknown"
                                .to_string(),
                        )),
                        predicate: VarOrNamedNode::NamedNode(Full(
                            "https://ontology.okp4.space/core/hasTopic".to_string(),
                        )),
                        object: VarOrNamedNodeOrLiteral::NamedNode(Full(
                            "https://ontology.okp4.space/thesaurus/topic/Test".to_string(),
                        )),
                    }],
                    r#where: vec![WhereCondition::Simple(TriplePattern(msg::TriplePattern {
                        subject: VarOrNode::Node(NamedNode(Full(
                            "https://ontology.okp4.space/dataverse/dataspace/metadata/unknown"
                                .to_string(),
                        ))),
                        predicate: VarOrNamedNode::NamedNode(Full(
                            "https://ontology.okp4.space/core/hasTopic".to_string(),
                        )),
                        object: VarOrNodeOrLiteral::Node(NamedNode(Full(
                            "https://ontology.okp4.space/thesaurus/topic/Test".to_string(),
                        ))),
                    }))],
                },
                0,
                0,
                Uint128::from(7103u128),
            ),
            (
                DeleteData {
                    prefixes: vec![],
                    delete: vec![msg::TripleDeleteTemplate {
                        subject: VarOrNamedNode::NamedNode(Full(id.to_string())),
                        predicate: VarOrNamedNode::NamedNode(Full(
                            "https://ontology.okp4.space/core/hasTopic".to_string(),
                        )),
                        object: VarOrNamedNodeOrLiteral::NamedNode(Full(
                            "https://ontology.okp4.space/thesaurus/topic/Test".to_string(),
                        )),
                    }],
                    r#where: vec![WhereCondition::Simple(TriplePattern(msg::TriplePattern {
                        subject: VarOrNode::Node(NamedNode(Full(id.to_string()))),
                        predicate: VarOrNamedNode::NamedNode(Full(
                            "https://ontology.okp4.space/core/hasTopic".to_string(),
                        )),
                        object: VarOrNodeOrLiteral::Node(NamedNode(Full(
                            "https://ontology.okp4.space/thesaurus/topic/Test".to_string(),
                        ))),
                    }))],
                },
                1,
                0,
                Uint128::from(6921u128),
            ),
            (
                DeleteData {
                    prefixes: vec![
                        Prefix {
                            prefix: "core".to_string(),
                            namespace: "https://ontology.okp4.space/core/".to_string(),
                        },
                        Prefix {
                            prefix: "thesaurus".to_string(),
                            namespace: "https://ontology.okp4.space/thesaurus/topic/".to_string(),
                        },
                    ],
                    delete: vec![msg::TripleDeleteTemplate {
                        subject: VarOrNamedNode::NamedNode(Full(id.to_string())),
                        predicate: VarOrNamedNode::NamedNode(Prefixed("core:hasTopic".to_string())),
                        object: VarOrNamedNodeOrLiteral::NamedNode(Prefixed(
                            "thesaurus:Test".to_string(),
                        )),
                    }],
                    r#where: vec![WhereCondition::Simple(TriplePattern(msg::TriplePattern {
                        subject: VarOrNode::Node(NamedNode(Full(id.to_string()))),
                        predicate: VarOrNamedNode::NamedNode(Prefixed("core:hasTopic".to_string())),
                        object: VarOrNodeOrLiteral::Node(NamedNode(Prefixed(
                            "thesaurus:Test".to_string(),
                        ))),
                    }))],
                },
                1,
                0,
                Uint128::from(6921u128),
            ),
            (
                DeleteData {
                    prefixes: vec![
                        Prefix {
                            prefix: "core".to_string(),
                            namespace: "https://ontology.okp4.space/core/".to_string(),
                        },
                        Prefix {
                            prefix: "thesaurus".to_string(),
                            namespace: "https://ontology.okp4.space/thesaurus/topic/".to_string(),
                        },
                    ],
                    delete: vec![msg::TripleDeleteTemplate {
                        subject: VarOrNamedNode::NamedNode(Full(id.to_string())),
                        predicate: VarOrNamedNode::NamedNode(Prefixed("core:hasTopic".to_string())),
                        object: VarOrNamedNodeOrLiteral::Variable("o".to_string()),
                    }],
                    r#where: vec![WhereCondition::Simple(TriplePattern(msg::TriplePattern {
                        subject: VarOrNode::Node(NamedNode(Full(id.to_string()))),
                        predicate: VarOrNamedNode::NamedNode(Prefixed("core:hasTopic".to_string())),
                        object: VarOrNodeOrLiteral::Variable("o".to_string()),
                    }))],
                },
                1,
                0,
                Uint128::from(6921u128),
            ),
            (
                DeleteData {
                    prefixes: vec![],
                    delete: vec![msg::TripleDeleteTemplate {
                        subject: VarOrNamedNode::NamedNode(Full(id.to_string())),
                        predicate: VarOrNamedNode::Variable("p".to_string()),
                        object: VarOrNamedNodeOrLiteral::Variable("o".to_string()),
                    }],
                    r#where: vec![WhereCondition::Simple(TriplePattern(msg::TriplePattern {
                        subject: VarOrNode::Node(NamedNode(Full(id.to_string()))),
                        predicate: VarOrNamedNode::Variable("p".to_string()),
                        object: VarOrNodeOrLiteral::Variable("o".to_string()),
                    }))],
                },
                11,
                2,
                Uint128::from(5272u128),
            ),
            // (
            //     DeleteData {
            //         prefixes: vec![],
            //         delete: vec![],
            //         r#where: vec![WhereCondition::Simple(TriplePattern(msg::TriplePattern {
            //             subject: VarOrNode::Node(NamedNode(Full(id.to_string()))),
            //             predicate: VarOrNamedNode::Variable("p".to_string()),
            //             object: VarOrNodeOrLiteral::Variable("o".to_string()),
            //         }))],
            //     },
            //     11,
            //     2,
            //     Uint128::from(5272u128),
            // ),
            // (
            //     DeleteData {
            //         prefixes: vec![],
            //         delete: vec![],
            //         r#where: vec![WhereCondition::Simple(TriplePattern(msg::TriplePattern {
            //             subject: VarOrNode::Variable("s".to_string()),
            //             predicate: VarOrNamedNode::Variable("p".to_string()),
            //             object: VarOrNodeOrLiteral::Variable("0".to_string()),
            //         }))],
            //     },
            //     40,
            //     17,
            //     Uint128::from(0u128),
            // ),
        ];

        for case in cases {
            let mut deps = mock_dependencies();

            let info = mock_info("owner", &[]);
            instantiate(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                InstantiateMsg::default(),
            )
            .unwrap();

            execute(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                InsertData {
                    format: Some(DataFormat::RDFXml),
                    data: read_test_data("sample.rdf.xml"),
                },
            )
            .unwrap();

            let res = execute(deps.as_mut(), mock_env(), info, case.0);

            assert!(res.is_ok());
            assert_eq!(
                res.unwrap().attributes,
                vec![
                    Attribute::new("action", "delete"),
                    Attribute::new("triple_count", case.1.to_string()),
                ]
            );

            assert_eq!(
                STORE.load(&deps.storage).unwrap().stat,
                StoreStat {
                    triple_count: (40u128 - u128::try_from(case.1).unwrap()).into(),
                    namespace_count: (17u128 - u128::try_from(case.2).unwrap()).into(),
                    byte_size: case.3,
                },
            );
            assert_eq!(
                triples()
                    .range_raw(&deps.storage, None, None, Order::Ascending)
                    .count(),
                40 - case.1
            );
            assert_eq!(
                namespaces()
                    .range_raw(&deps.storage, None, None, Order::Ascending)
                    .count(),
                17 - case.2
            );
        }
    }

    #[test]
    fn invalid_delete() {
        struct TC {
            command: ExecuteMsg,
            expected: ContractError,
        }
        let cases = vec![
            TC {
                command: DeleteData {
                    prefixes: vec![],
                    delete: vec![msg::TripleDeleteTemplate {
                        subject: VarOrNamedNode::NamedNode(Prefixed("foo:bar".to_string())),
                        predicate: VarOrNamedNode::NamedNode(Full(
                            "https://ontology.okp4.space/core/hasTopic".to_string(),
                        )),
                        object: VarOrNamedNodeOrLiteral::NamedNode(Full(
                            "https://ontology.okp4.space/thesaurus/topic/Test".to_string(),
                        )),
                    }],
                    r#where: vec![WhereCondition::Simple(TriplePattern(msg::TriplePattern {
                        subject: VarOrNode::Node(NamedNode(Prefixed("foo:bar".to_string()))),
                        predicate: VarOrNamedNode::NamedNode(Full(
                            "https://ontology.okp4.space/core/hasTopic".to_string(),
                        )),
                        object: VarOrNodeOrLiteral::Node(NamedNode(Full(
                            "https://ontology.okp4.space/thesaurus/topic/Test".to_string(),
                        ))),
                    }))],
                },
                expected: StdError::generic_err("Prefix not found: foo").into(),
            },
            TC {
                command: DeleteData {
                    prefixes: vec![],
                    delete: vec![msg::TripleDeleteTemplate {
                        subject: VarOrNamedNode::NamedNode(Full(
                            "https://ontology.okp4.space/thesaurus/topic/Test".to_string(),
                        )),
                        predicate: VarOrNamedNode::Variable("z".to_string()),
                        object: VarOrNamedNodeOrLiteral::Variable("o".to_string()),
                    }],
                    r#where: vec![WhereCondition::Simple(TriplePattern(msg::TriplePattern {
                        subject: VarOrNode::Node(NamedNode(Full(
                            "https://ontology.okp4.space/thesaurus/topic/Test".to_string(),
                        ))),
                        predicate: VarOrNamedNode::Variable("p".to_string()),
                        object: VarOrNodeOrLiteral::Variable("o".to_string()),
                    }))],
                },
                expected: StdError::generic_err("Selected variable not found in query").into(),
            },
            TC {
                command: DeleteData {
                    prefixes: vec![],
                    delete: vec![msg::TripleDeleteTemplate {
                        subject: VarOrNamedNode::NamedNode(Full(
                            "https://ontology.okp4.space/thesaurus/topic/Test".to_string(),
                        )),
                        predicate: VarOrNamedNode::NamedNode(Full(
                            "https://ontology.okp4.space/core/hasTopic".to_string(),
                        )),
                        object: VarOrNamedNodeOrLiteral::NamedNode(Full(
                            "https://ontology.okp4.space/thesaurus/topic/Test".to_string(),
                        )),
                    }],
                    r#where: vec![],
                },
                expected: StdError::generic_err("Empty basic graph pattern").into(),
            },
        ];

        for case in cases {
            let mut deps = mock_dependencies();

            let info = mock_info("owner", &[]);
            instantiate(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                InstantiateMsg::default(),
            )
            .unwrap();

            execute(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                InsertData {
                    format: Some(DataFormat::RDFXml),
                    data: read_test_data("sample.rdf.xml"),
                },
            )
            .unwrap();

            let res = execute(deps.as_mut(), mock_env(), info, case.command);

            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), case.expected);
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
            from_json::<StoreResponse>(&res.unwrap()).unwrap(),
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
                },
            }
        );
    }

    fn read_test_data(file: &str) -> Binary {
        let mut bytes: Vec<u8> = Vec::new();

        File::open(
            Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
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
                            predicate: VarOrNamedNode::NamedNode(Full(
                                "https://ontology.okp4.space/core/hasDescription".to_string(),
                            )),
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
                            ]),
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
                            predicate: VarOrNamedNode::NamedNode(Prefixed(
                                "core:hasDescription".to_string(),
                            )),
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
                            predicate: VarOrNamedNode::Variable("a".to_string()),
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
            ),
        ];

        let mut deps = mock_dependencies();

        let info = mock_info("owner", &[]);
        instantiate(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            InstantiateMsg::default(),
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

            let result = from_json::<SelectResponse>(&res.unwrap()).unwrap();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn proper_select_blank_nodes() {
        let cases = vec![
            (
                SelectQuery {
                    prefixes: vec![Prefix { prefix: "core".to_string(), namespace: "https://ontology.okp4.space/core/".to_string() }],
                    select: vec![SelectItem::Variable("a".to_string()), SelectItem::Variable("b".to_string())],
                    r#where: vec![
                        WhereCondition::Simple(TriplePattern(
                            msg::TriplePattern {
                                subject: VarOrNode::Variable("a".to_string()),
                                predicate: VarOrNamedNode::NamedNode(Prefixed(
                                    "core:hasTemporalCoverage".to_string(),
                                )),
                                object: VarOrNodeOrLiteral::Node(BlankNode("a".to_string())),
                            },
                        )),
                        WhereCondition::Simple(TriplePattern(
                            msg::TriplePattern {
                                subject: VarOrNode::Node(BlankNode("a".to_string())),
                                predicate: VarOrNamedNode::NamedNode(Prefixed(
                                    "core:hasStartDate".to_string(),
                                )),
                                object: VarOrNodeOrLiteral::Variable("b".to_string()),
                            },
                        ))],
                    limit: None,
                },
                SelectResponse {
                    head: Head { vars: vec!["a".to_string(), "b".to_string()] },
                    results: Results {
                        bindings: vec![
                            BTreeMap::from([
                                (
                                    "a".to_string(),
                                    Value::URI {
                                        value: Full("https://ontology.okp4.space/dataverse/dataset/metadata/80b1f84e-86dc-4730-b54f-701ad9b1888a".to_string())
                                    }
                                ),
                                (
                                    "b".to_string(),
                                    Value::Literal {
                                        value: "2022-01-01T00:00:00+00:00".to_string(),
                                        lang: None,
                                        datatype: Some(Full("http://www.w3.org/2001/XMLSchema#dateTime".to_string())),
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
                    select: vec![SelectItem::Variable("a".to_string()), SelectItem::Variable("b".to_string())],
                    r#where: vec![
                        WhereCondition::Simple(TriplePattern(
                            msg::TriplePattern {
                                subject: VarOrNode::Variable("a".to_string()),
                                predicate: VarOrNamedNode::NamedNode(Prefixed(
                                    "core:hasTemporalCoverage".to_string(),
                                )),
                                object: VarOrNodeOrLiteral::Variable("blank".to_string()),
                            },
                        )),
                        WhereCondition::Simple(TriplePattern(
                            msg::TriplePattern {
                                subject: VarOrNode::Variable("blank".to_string()),
                                predicate: VarOrNamedNode::NamedNode(Prefixed(
                                    "core:hasStartDate".to_string(),
                                )),
                                object: VarOrNodeOrLiteral::Variable("b".to_string()),
                            },
                        ))],
                    limit: None,
                },
                SelectResponse {
                    head: Head { vars: vec!["a".to_string(), "b".to_string()] },
                    results: Results {
                        bindings: vec![
                            BTreeMap::from([
                                (
                                    "a".to_string(),
                                    Value::URI {
                                        value: Full("https://ontology.okp4.space/dataverse/dataset/metadata/80b1f84e-86dc-4730-b54f-701ad9b1888a".to_string())
                                    }
                                ),
                                (
                                    "b".to_string(),
                                    Value::Literal {
                                        value: "2022-01-01T00:00:00+00:00".to_string(),
                                        lang: None,
                                        datatype: Some(Full("http://www.w3.org/2001/XMLSchema#dateTime".to_string())),
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
                    select: vec![SelectItem::Variable("a".to_string()), SelectItem::Variable("b".to_string())],
                    r#where: vec![
                        WhereCondition::Simple(TriplePattern(
                            msg::TriplePattern {
                                subject: VarOrNode::Variable("a".to_string()),
                                predicate: VarOrNamedNode::NamedNode(Prefixed(
                                    "core:hasTemporalCoverage".to_string(),
                                )),
                                object: VarOrNodeOrLiteral::Node(BlankNode("blank1".to_string())),
                            },
                        )),
                        WhereCondition::Simple(TriplePattern(
                            msg::TriplePattern {
                                subject: VarOrNode::Node(BlankNode("blank2".to_string())),
                                predicate: VarOrNamedNode::NamedNode(Prefixed(
                                    "core:hasInformation".to_string(),
                                )),
                                object: VarOrNodeOrLiteral::Variable("b".to_string()),
                            },
                        ))],
                    limit: None,
                },
                SelectResponse {
                    head: Head { vars: vec!["a".to_string(), "b".to_string()] },
                    results: Results {
                        bindings: vec![
                            BTreeMap::from([
                                (
                                    "a".to_string(),
                                    Value::URI {
                                        value: Full("https://ontology.okp4.space/dataverse/dataset/metadata/80b1f84e-86dc-4730-b54f-701ad9b1888a".to_string())
                                    }
                                ),
                                (
                                    "b".to_string(),
                                    Value::Literal {
                                        value: "this is a dataset".to_string(),
                                        lang: None,
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
                    select: vec![SelectItem::Variable("a".to_string()), SelectItem::Variable("b".to_string())],
                    r#where: vec![
                        WhereCondition::Simple(TriplePattern(
                            msg::TriplePattern {
                                subject: VarOrNode::Variable("a".to_string()),
                                predicate: VarOrNamedNode::NamedNode(Prefixed(
                                    "core:hasTemporalCoverage".to_string(),
                                )),
                                object: VarOrNodeOrLiteral::Variable("b".to_string()),
                            },
                        ))],
                    limit: None,
                },
                SelectResponse {
                    head: Head { vars: vec!["a".to_string(), "b".to_string()] },
                    results: Results {
                        bindings: vec![
                            BTreeMap::from([
                                (
                                    "a".to_string(),
                                    Value::URI {
                                        value: Full("https://ontology.okp4.space/dataverse/dataset/metadata/80b1f84e-86dc-4730-b54f-701ad9b1888a".to_string())
                                    }
                                ),
                                (
                                    "b".to_string(),
                                    Value::BlankNode {
                                        value: "b0".to_string(),
                                    }
                                )
                            ])
                        ],
                    },
                },
            ),
        ];

        let mut deps = mock_dependencies();

        let info = mock_info("owner", &[]);
        instantiate(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            InstantiateMsg::default(),
        )
        .unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            info,
            InsertData {
                format: Some(DataFormat::Turtle),
                data: read_test_data("blank-nodes.ttl"),
            },
        )
        .unwrap();

        for (q, expected) in cases {
            let res = query(deps.as_ref(), mock_env(), QueryMsg::Select { query: q });
            assert!(res.is_ok());

            let result = from_json::<SelectResponse>(&res.unwrap()).unwrap();
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
                        predicate: VarOrNamedNode::NamedNode(Prefixed(
                            "invalid:hasDescription".to_string(),
                        )),
                        object: VarOrNodeOrLiteral::Literal(Literal::LanguageTaggedString {
                            value: "A test Dataset.".to_string(),
                            language: "en".to_string(),
                        }),
                    }))],
                    limit: None,
                },
                Err(StdError::generic_err("Prefix not found: invalid")),
            ),
            (
                SelectQuery {
                    prefixes: vec![],
                    select: vec![SelectItem::Variable("u".to_string())],
                    r#where: vec![WhereCondition::Simple(TriplePattern(msg::TriplePattern {
                        subject: VarOrNode::Variable("a".to_string()),
                        predicate: VarOrNamedNode::NamedNode(Full(
                            "https://ontology.okp4.space/core/hasDescription".to_string(),
                        )),
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
                    max_query_variable_count: 1,
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

    #[test]
    fn formats_describe() {
        let cases = vec![
            (
                QueryMsg::Describe {
                    query: DescribeQuery {
                        prefixes: vec![],
                        resource: VarOrNamedNode::NamedNode(Full("https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473".to_string())),
                        r#where: vec![],
                    },
                    format: Some(DataFormat::Turtle),
                },
                DescribeResponse {
                    format: DataFormat::Turtle,
                    data: Binary::from(
                        "<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://ontology.okp4.space/metadata/dataspace/GeneralMetadata> , <http://www.w3.org/2002/07/owl#NamedIndividual> ;
\t<https://ontology.okp4.space/core/hasTag> \"Test\" , \"OKP4\" ;
\t<https://ontology.okp4.space/core/hasTitle> \"Data Space de test\"@fr , \"Test Data Space\"@en ;
\t<https://ontology.okp4.space/core/hasTopic> <https://ontology.okp4.space/thesaurus/topic/Test> ;
\t<https://ontology.okp4.space/core/describes> <https://ontology.okp4.space/dataverse/dataspace/97ff7e16-c08d-47be-8475-211016c82e33> ;
\t<https://ontology.okp4.space/core/hasPublisher> \"OKP4\" ;
\t<https://ontology.okp4.space/core/hasDescription> \"A test Data Space.\"@en , \"Un Data Space de test.\"@fr .
\
                ".to_string().as_bytes().to_vec()),
                }
            ),
            (
                QueryMsg::Describe {
                    query: DescribeQuery {
                        prefixes: vec![],
                        resource: VarOrNamedNode::NamedNode(Full("https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473".to_string())),
                        r#where: vec![],
                    },
                    format: Some(DataFormat::RDFXml),
                },
                DescribeResponse {
                    format: DataFormat::RDFXml,
                    data: Binary::from(
                        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">\
<rdf:Description rdf:about=\"https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473\">\
<type xmlns=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\" rdf:resource=\"https://ontology.okp4.space/metadata/dataspace/GeneralMetadata\"/>\
<type xmlns=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\" rdf:resource=\"http://www.w3.org/2002/07/owl#NamedIndividual\"/>\
<hasTag xmlns=\"https://ontology.okp4.space/core/\">Test</hasTag><hasTag xmlns=\"https://ontology.okp4.space/core/\">OKP4</hasTag>\
<hasTitle xmlns=\"https://ontology.okp4.space/core/\" xml:lang=\"fr\">Data Space de test</hasTitle>\
<hasTitle xmlns=\"https://ontology.okp4.space/core/\" xml:lang=\"en\">Test Data Space</hasTitle>\
<hasTopic xmlns=\"https://ontology.okp4.space/core/\" rdf:resource=\"https://ontology.okp4.space/thesaurus/topic/Test\"/>\
<describes xmlns=\"https://ontology.okp4.space/core/\" rdf:resource=\"https://ontology.okp4.space/dataverse/dataspace/97ff7e16-c08d-47be-8475-211016c82e33\"/>\
<hasPublisher xmlns=\"https://ontology.okp4.space/core/\">OKP4</hasPublisher>\
<hasDescription xmlns=\"https://ontology.okp4.space/core/\" xml:lang=\"en\">A test Data Space.</hasDescription>\
<hasDescription xmlns=\"https://ontology.okp4.space/core/\" xml:lang=\"fr\">Un Data Space de test.</hasDescription></rdf:Description>\
</rdf:RDF>\
\
                ".to_string().as_bytes().to_vec()),
                }
            ),
            (
                QueryMsg::Describe {
                    query: DescribeQuery {
                        prefixes: vec![],
                        resource: VarOrNamedNode::NamedNode(Full("https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473".to_string())),
                        r#where: vec![WhereCondition::Simple(TriplePattern(
                            msg::TriplePattern {
                                subject: VarOrNode::Variable("a".to_string()),
                                predicate: VarOrNamedNode::NamedNode(Full(
                                    "https://ontology.okp4.space/core/hasDescription".to_string(),
                                )),
                                object: VarOrNodeOrLiteral::Variable("b".to_string()),
                            },
                        ))],
                    },
                    format: Some(DataFormat::NTriples),
                },
                DescribeResponse {
                    format: DataFormat::NTriples,
                    data: Binary::from(
                        "<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://ontology.okp4.space/metadata/dataspace/GeneralMetadata> .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#NamedIndividual> .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasTag> \"Test\" .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasTag> \"OKP4\" .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasTitle> \"Data Space de test\"@fr .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasTitle> \"Test Data Space\"@en .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasTopic> <https://ontology.okp4.space/thesaurus/topic/Test> .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/describes> <https://ontology.okp4.space/dataverse/dataspace/97ff7e16-c08d-47be-8475-211016c82e33> .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasPublisher> \"OKP4\" .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasDescription> \"A test Data Space.\"@en .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasDescription> \"Un Data Space de test.\"@fr .
\
                ".to_string().as_bytes().to_vec()),
                }
            ),
            (
                QueryMsg::Describe {
                    query: DescribeQuery {
                        prefixes: vec![],
                        resource: VarOrNamedNode::NamedNode(Full("https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473".to_string())),
                        r#where: vec![],
                    },
                    format: Some(DataFormat::NQuads),
                },
                DescribeResponse {
                    format: DataFormat::NQuads,
                    data: Binary::from(
                        "<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://ontology.okp4.space/metadata/dataspace/GeneralMetadata> .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#NamedIndividual> .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasTag> \"Test\" .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasTag> \"OKP4\" .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasTitle> \"Data Space de test\"@fr .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasTitle> \"Test Data Space\"@en .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasTopic> <https://ontology.okp4.space/thesaurus/topic/Test> .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/describes> <https://ontology.okp4.space/dataverse/dataspace/97ff7e16-c08d-47be-8475-211016c82e33> .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasPublisher> \"OKP4\" .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasDescription> \"A test Data Space.\"@en .
<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasDescription> \"Un Data Space de test.\"@fr .
\
                ".to_string().as_bytes().to_vec()),
                }
            ),
        ];

        let mut deps = mock_dependencies();

        let info = mock_info("owner", &[]);
        instantiate(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            InstantiateMsg::default(),
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
            let res = query(deps.as_ref(), mock_env(), q);

            assert!(res.is_ok());

            let result = from_json::<DescribeResponse>(&res.unwrap()).unwrap();

            assert_eq!(result.format, expected.format);
            assert_eq!(
                String::from_utf8_lossy(&result.data),
                String::from_utf8_lossy(&expected.data)
            );
        }
    }

    #[test]
    fn prefixes_describe() {
        let cases = vec![
            (
                QueryMsg::Describe {
                    query: DescribeQuery {
                        prefixes: vec![
                            Prefix {
                                prefix: "metadata".to_string(),
                                namespace: "https://ontology.okp4.space/dataverse/dataspace/metadata/".to_string(),
                            },
                        ],
                        resource: VarOrNamedNode::NamedNode(Prefixed("metadata:dcf48417-01c5-4b43-9bc7-49e54c028473".to_string())),
                        r#where: vec![],
                    },
                    format: Some(DataFormat::Turtle),
                },
                DescribeResponse {
                    format: DataFormat::Turtle,
                    data: Binary::from(
                        "<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://ontology.okp4.space/metadata/dataspace/GeneralMetadata> , <http://www.w3.org/2002/07/owl#NamedIndividual> ;
\t<https://ontology.okp4.space/core/hasTag> \"Test\" , \"OKP4\" ;
\t<https://ontology.okp4.space/core/hasTitle> \"Data Space de test\"@fr , \"Test Data Space\"@en ;
\t<https://ontology.okp4.space/core/hasTopic> <https://ontology.okp4.space/thesaurus/topic/Test> ;
\t<https://ontology.okp4.space/core/describes> <https://ontology.okp4.space/dataverse/dataspace/97ff7e16-c08d-47be-8475-211016c82e33> ;
\t<https://ontology.okp4.space/core/hasPublisher> \"OKP4\" ;
\t<https://ontology.okp4.space/core/hasDescription> \"A test Data Space.\"@en , \"Un Data Space de test.\"@fr .
\
                ".to_string().as_bytes().to_vec()),
                }
            ),
        ];

        let mut deps = mock_dependencies();

        let info = mock_info("owner", &[]);
        instantiate(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            InstantiateMsg::default(),
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
            let res = query(deps.as_ref(), mock_env(), q);

            assert!(res.is_ok());

            let result = from_json::<DescribeResponse>(&res.unwrap()).unwrap();

            assert_eq!(result.format, expected.format);
            assert_eq!(
                String::from_utf8_lossy(&result.data),
                String::from_utf8_lossy(&expected.data)
            );
        }
    }

    #[test]
    fn variable_describe() {
        let cases = vec![
            (
                QueryMsg::Describe {
                    query: DescribeQuery {
                        prefixes: vec![Prefix { prefix: "core".to_string(), namespace: "https://ontology.okp4.space/core/".to_string() }],
                        resource: VarOrNamedNode::Variable("a".to_string()),
                        r#where: vec![WhereCondition::Simple(TriplePattern(
                            msg::TriplePattern {
                                subject: VarOrNode::Variable("a".to_string()),
                                predicate: VarOrNamedNode::NamedNode(Prefixed(
                                    "core:hasDescription".to_string(),
                                )),
                                object: VarOrNodeOrLiteral::Literal(Literal::LanguageTaggedString { value: "A test Dataset.".to_string(), language: "en".to_string() }),
                            },
                        ))],
                    },
                    format: Some(DataFormat::Turtle),
                },
                DescribeResponse {
                    format: DataFormat::Turtle,
                    data: Binary::from(
                        "<https://ontology.okp4.space/dataverse/dataset/metadata/d1615703-4ee1-4e2f-997e-15aecf1eea4e> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://ontology.okp4.space/metadata/dataset/GeneralMetadata> , <http://www.w3.org/2002/07/owl#NamedIndividual> ;\n\t<https://ontology.okp4.space/core/hasTag> \"test\" ;\n\t<https://ontology.okp4.space/core/hasTitle> \"test Dataset\"@en , \"Dataset de test\"@fr ;\n\t<https://ontology.okp4.space/core/hasTopic> <https://ontology.okp4.space/thesaurus/topic/Test> ;\n\t<https://ontology.okp4.space/core/describes> <https://ontology.okp4.space/dataverse/dataset/0ea1fc7a-dd97-4adc-a10e-169c6597bcde> ;\n\t<https://ontology.okp4.space/core/hasFormat> <https://ontology.okp4.space/thesaurus/media-type/application_vndms-excel> ;\n\t<https://ontology.okp4.space/core/hasCreator> \"Me\" ;\n\t<https://ontology.okp4.space/core/hasLicense> <https://ontology.okp4.space/thesaurus/license/LO-FR-1_0> ;\n\t<https://ontology.okp4.space/core/hasPublisher> \"OKP4\" ;\n\t<https://ontology.okp4.space/core/hasDescription> \"Un Dataset de test.\"@fr , \"A test Dataset.\"@en .\n".to_string().as_bytes().to_vec()),
                }
            ),
        ];

        let mut deps = mock_dependencies();

        let info = mock_info("owner", &[]);
        instantiate(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            InstantiateMsg::default(),
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
            let res = query(deps.as_ref(), mock_env(), q);

            assert!(res.is_ok());

            let result = from_json::<DescribeResponse>(&res.unwrap()).unwrap();

            assert_eq!(result.format, expected.format);
            assert_eq!(
                String::from_utf8_lossy(&result.data),
                String::from_utf8_lossy(&expected.data)
            );
        }
    }

    #[test]
    fn variable_mutiple_resources_describe() {
        let cases = vec![
            (
                QueryMsg::Describe {
                    query: DescribeQuery {
                        prefixes: vec![Prefix { prefix: "core".to_string(), namespace: "https://ontology.okp4.space/core/".to_string() }],
                        resource: VarOrNamedNode::Variable("a".to_string()),
                        r#where: vec![WhereCondition::Simple(TriplePattern(
                            msg::TriplePattern {
                                subject: VarOrNode::Variable("a".to_string()),
                                predicate: VarOrNamedNode::NamedNode(Prefixed(
                                    "core:hasPublisher".to_string(),
                                )),
                                object: VarOrNodeOrLiteral::Literal(Literal::Simple("OKP4".to_string())),
                            },
                        ))],
                    },
                    format: Some(DataFormat::Turtle),
                },
                DescribeResponse {
                    format: DataFormat::Turtle,
                    data: Binary::from(
                        "<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://ontology.okp4.space/metadata/dataspace/GeneralMetadata> , <http://www.w3.org/2002/07/owl#NamedIndividual> ;\n\t<https://ontology.okp4.space/core/hasTag> \"Test\" , \"OKP4\" ;\n\t<https://ontology.okp4.space/core/hasTitle> \"Data Space de test\"@fr , \"Test Data Space\"@en ;\n\t<https://ontology.okp4.space/core/hasTopic> <https://ontology.okp4.space/thesaurus/topic/Test> ;\n\t<https://ontology.okp4.space/core/describes> <https://ontology.okp4.space/dataverse/dataspace/97ff7e16-c08d-47be-8475-211016c82e33> ;\n\t<https://ontology.okp4.space/core/hasPublisher> \"OKP4\" ;\n\t<https://ontology.okp4.space/core/hasDescription> \"A test Data Space.\"@en , \"Un Data Space de test.\"@fr .\n<https://ontology.okp4.space/dataverse/dataset/metadata/d1615703-4ee1-4e2f-997e-15aecf1eea4e> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://ontology.okp4.space/metadata/dataset/GeneralMetadata> , <http://www.w3.org/2002/07/owl#NamedIndividual> ;\n\t<https://ontology.okp4.space/core/hasTag> \"test\" ;\n\t<https://ontology.okp4.space/core/hasTitle> \"test Dataset\"@en , \"Dataset de test\"@fr ;\n\t<https://ontology.okp4.space/core/hasTopic> <https://ontology.okp4.space/thesaurus/topic/Test> ;\n\t<https://ontology.okp4.space/core/describes> <https://ontology.okp4.space/dataverse/dataset/0ea1fc7a-dd97-4adc-a10e-169c6597bcde> ;\n\t<https://ontology.okp4.space/core/hasFormat> <https://ontology.okp4.space/thesaurus/media-type/application_vndms-excel> ;\n\t<https://ontology.okp4.space/core/hasCreator> \"Me\" ;\n\t<https://ontology.okp4.space/core/hasLicense> <https://ontology.okp4.space/thesaurus/license/LO-FR-1_0> ;\n\t<https://ontology.okp4.space/core/hasPublisher> \"OKP4\" ;\n\t<https://ontology.okp4.space/core/hasDescription> \"Un Dataset de test.\"@fr , \"A test Dataset.\"@en .\n".to_string().as_bytes().to_vec()),
                }
            ),
        ];

        let mut deps = mock_dependencies();

        let info = mock_info("owner", &[]);
        instantiate(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            InstantiateMsg::default(),
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
            let res = query(deps.as_ref(), mock_env(), q);

            assert!(res.is_ok());

            let result = from_json::<DescribeResponse>(&res.unwrap()).unwrap();

            assert_eq!(result.format, expected.format);
            assert_eq!(
                String::from_utf8_lossy(&result.data),
                String::from_utf8_lossy(&expected.data)
            );
        }
    }

    #[test]
    fn blanknode_describe() {
        let cases = vec![
            (
                QueryMsg::Describe {
                    query: DescribeQuery {
                        prefixes: vec![
                            Prefix { prefix: "core".to_string(), namespace: "https://ontology.okp4.space/core/".to_string() },
                            Prefix { prefix: "metadata-dataset".to_string(), namespace: "https://ontology.okp4.space/dataverse/dataset/metadata/".to_string() },
                        ],
                        resource: VarOrNamedNode::Variable("x".to_string()),
                        r#where: vec![WhereCondition::Simple(TriplePattern(
                            msg::TriplePattern {
                                subject: VarOrNode::Node(NamedNode(Prefixed("metadata-dataset:80b1f84e-86dc-4730-b54f-701ad9b1888a".to_string()))),
                                predicate: VarOrNamedNode::NamedNode(Prefixed(
                                    "core:hasTemporalCoverage".to_string(),
                                )),
                                object: VarOrNodeOrLiteral::Variable("x".to_string()),
                            },
                        )),
                        ],
                    },
                    format: Some(DataFormat::Turtle),
                },
                DescribeResponse {
                    format: DataFormat::Turtle,
                    data: Binary::from(
                        "<b0> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#NamedIndividual> , <https://ontology.okp4.space/core/Period> ;\n\t<https://ontology.okp4.space/core/hasStartDate> \"2022-01-01T00:00:00+00:00\"^^<http://www.w3.org/2001/XMLSchema#dateTime> .\n".to_string().as_bytes().to_vec()),
                }
            ),
        ];

        let mut deps = mock_dependencies();

        let info = mock_info("owner", &[]);
        instantiate(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            InstantiateMsg::default(),
        )
        .unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            info,
            InsertData {
                format: Some(DataFormat::Turtle),
                data: read_test_data("blank-nodes.ttl"),
            },
        )
        .unwrap();

        for (q, expected) in cases {
            let res = query(deps.as_ref(), mock_env(), q);

            assert!(res.is_ok());

            let result = from_json::<DescribeResponse>(&res.unwrap()).unwrap();

            assert_eq!(result.format, expected.format);
            assert_eq!(
                String::from_utf8_lossy(&result.data),
                String::from_utf8_lossy(&expected.data)
            );
        }
    }

    #[test]
    fn proper_construct() {
        let id = "https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473";
        let cases = vec![
                        // (
                        //      QueryMsg::Construct {
                        //          query: ConstructQuery {
                        //              prefixes: vec![],
                        //              construct: vec![],
                        //              r#where: vec![WhereCondition::Simple(TriplePattern(msg::TriplePattern {
                        //                  subject: VarOrNode::Node(NamedNode(Full(id.to_string()))),
                        //                  predicate: VarOrNamedNode::NamedNode(Full(
                        //                      "https://ontology.okp4.space/core/hasTag".to_string(),
                        //                  )),
                        //                  object: VarOrNodeOrLiteral::Variable("o".to_string()),
                        //              }))],
                        //          },
                        //          format: None,
                        //      },
                        //      ConstructResponse {
                        //          format: DataFormat::Turtle,
                        //          data: Binary::from(
                        //              "<https://ontology.okp4.space/dataverse/dataspace/metadata/dcf48417-01c5-4b43-9bc7-49e54c028473> <https://ontology.okp4.space/core/hasTag> \"Test\" , \"OKP4\" .\n".to_string().as_bytes().to_vec()),
                        //      },
                        //  ),
                         (
                             QueryMsg::Construct {
                                 query: ConstructQuery {
                                     prefixes: vec![
                                         Prefix { prefix: "my-ns".to_string(), namespace: "https://my-ns.org/".to_string() },
                                         Prefix { prefix: "metadata-dataset".to_string(), namespace: "https://ontology.okp4.space/dataverse/dataset/metadata/".to_string() },
                                     ],
                                     construct: vec![
                                         msg::TripleConstructTemplate {
                                             subject: VarOrNode::Node(NamedNode(Prefixed("my-ns:instance-1".to_string()))),
                                             predicate: VarOrNamedNode::NamedNode(Full(
                                                 "https://my-ns/predicate/tag".to_string(),
                                             )),
                                             object: VarOrNodeOrLiteral::Variable("o".to_string()),
                                         }
                                     ],
                                     r#where: vec![WhereCondition::Simple(TriplePattern(msg::TriplePattern {
                                         subject: VarOrNode::Node(NamedNode(Full(id.to_string()))),
                                         predicate: VarOrNamedNode::NamedNode(Full(
                                             "https://ontology.okp4.space/core/hasTag".to_string(),
                                         )),
                                         object: VarOrNodeOrLiteral::Variable("o".to_string()),
                                     }))],
                                 },
                                 format: Some(DataFormat::NTriples),
                             },
                             ConstructResponse {
                                 format: DataFormat::NTriples,
                                 data: Binary::from(
                                     "<https://my-ns.org/instance-1> <https://my-ns/predicate/tag> \"Test\" .\n<https://my-ns.org/instance-1> <https://my-ns/predicate/tag> \"OKP4\" .\n".to_string().as_bytes().to_vec()),
                             },
                         )];

        for (q, expected) in cases {
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
                info.clone(),
                InsertData {
                    format: Some(DataFormat::RDFXml),
                    data: read_test_data("sample.rdf.xml"),
                },
            )
            .unwrap();

            let res = query(deps.as_ref(), mock_env(), q);

            assert!(res.is_ok());

            let result = from_json::<DescribeResponse>(&res.unwrap()).unwrap();

            assert_eq!(result.format, expected.format);
            assert_eq!(
                String::from_utf8_lossy(&result.data),
                String::from_utf8_lossy(&expected.data)
            );
        }
    }
}
