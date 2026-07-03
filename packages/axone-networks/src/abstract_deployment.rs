use abstract_std::{
    native_addrs::{ANS_HOST_SALT, BLOB_CHECKSUM, MODULE_FACTORY_SALT, REGISTRY_SALT},
    objects::{module::ModuleInfo, module_reference::ModuleReference},
    registry::{ModulesResponse, QueryMsg},
    ACCOUNT, ANS_HOST, MODULE_FACTORY, REGISTRY,
};
use cosmrs::{
    proto::{
        cosmos::base::query::v1beta1::PageRequest,
        cosmwasm::wasm::v1::{query_client::QueryClient, QueryCodesRequest},
    },
    AccountId,
};
use cosmwasm_std::{instantiate2_address, Addr, CanonicalAddr};
use cw_orch::{
    anyhow,
    daemon::{networks::ChainInfo, senders::QueryOnlyDaemon, Daemon, DaemonBuilder},
    prelude::*,
    tokio::runtime::Runtime,
};
use std::{collections::BTreeSet, str::FromStr};

#[derive(Debug)]
pub struct AbstractDeploymentInfo {
    pub account_code_id: u64,
    pub ans_host_addr: Addr,
    pub creator_addr: String,
    pub module_factory_addr: Addr,
    pub registry_addr: Addr,
}

pub fn addr_from_instantiate2(creator_addr: &str, salt: &[u8]) -> anyhow::Result<Addr> {
    let creator_account_id =
        AccountId::from_str(creator_addr).map_err(|err| anyhow::Error::msg(err.to_string()))?;
    let creator_canon = CanonicalAddr::from(creator_account_id.to_bytes());
    let contract_canon = instantiate2_address(&BLOB_CHECKSUM, &creator_canon, salt)?;
    let contract_addr = AccountId::new(creator_account_id.prefix(), contract_canon.as_slice())
        .map_err(|err| anyhow::Error::msg(err.to_string()))?;
    Ok(Addr::unchecked(contract_addr.to_string()))
}

pub fn discover_blob_creators(chain: &QueryOnlyDaemon) -> anyhow::Result<BTreeSet<String>> {
    let mut client = QueryClient::new(chain.channel());
    let mut pagination = Some(PageRequest {
        key: vec![],
        offset: 0,
        limit: 200,
        count_total: false,
        reverse: false,
    });
    let mut creators = BTreeSet::new();

    loop {
        let response = chain.rt_handle.block_on(client.codes(QueryCodesRequest {
            pagination: pagination.clone(),
        }))?;
        let response = response.into_inner();

        for code_info in response.code_infos {
            if code_info.data_hash.as_slice() == BLOB_CHECKSUM {
                creators.insert(code_info.creator);
            }
        }

        let next_key = response
            .pagination
            .map(|page| page.next_key)
            .unwrap_or_default();
        if next_key.is_empty() {
            break;
        }

        pagination = Some(PageRequest {
            key: next_key,
            offset: 0,
            limit: 200,
            count_total: false,
            reverse: false,
        });
    }

    Ok(creators)
}

pub fn discover_abstract_deployment(
    chain: &QueryOnlyDaemon,
) -> anyhow::Result<AbstractDeploymentInfo> {
    discover_abstract_deployment_from_creators(chain, discover_blob_creators(chain)?)
}

pub fn discover_abstract_deployment_from_creators(
    chain: &QueryOnlyDaemon,
    creators: BTreeSet<String>,
) -> anyhow::Result<AbstractDeploymentInfo> {
    let wasm = chain.wasm_querier();

    for creator_addr in creators {
        let ans_host_addr = addr_from_instantiate2(&creator_addr, ANS_HOST_SALT)?;
        let registry_addr = addr_from_instantiate2(&creator_addr, REGISTRY_SALT)?;
        let module_factory_addr = addr_from_instantiate2(&creator_addr, MODULE_FACTORY_SALT)?;

        let account_module = wasm.smart_query::<_, ModulesResponse>(
            &registry_addr,
            &QueryMsg::Modules {
                infos: vec![ModuleInfo::from_id_latest(ACCOUNT)?],
            },
        );

        let Ok(account_module) = account_module else {
            continue;
        };

        let Some(module_response) = account_module.modules.first() else {
            continue;
        };

        let ModuleReference::Account(account_code_id) = module_response.module.reference else {
            continue;
        };

        return Ok(AbstractDeploymentInfo {
            account_code_id,
            ans_host_addr,
            creator_addr,
            module_factory_addr,
            registry_addr,
        });
    }

    anyhow::bail!(
        "no Abstract deployment found on-chain for {} / {}",
        REGISTRY,
        MODULE_FACTORY
    );
}

pub fn seed_abstract_addresses(
    chain: &Daemon,
    network: &ChainInfo,
    rt: &Runtime,
) -> anyhow::Result<AbstractDeploymentInfo> {
    let query_chain = DaemonBuilder::new(network.clone())
        .handle(rt.handle())
        .is_test(true)
        .build_sender(())?;
    let deployment = discover_abstract_deployment(&query_chain)?;

    chain
        .state()
        .set_address(ANS_HOST, &deployment.ans_host_addr);
    chain
        .state()
        .set_address(REGISTRY, &deployment.registry_addr);
    chain
        .state()
        .set_address(MODULE_FACTORY, &deployment.module_factory_addr);

    Ok(deployment)
}
