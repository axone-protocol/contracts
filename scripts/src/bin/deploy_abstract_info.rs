//! List Abstract deployment information for one or more networks.

use abstract_std::{
    native_addrs::{BLOB_CHECKSUM, MODULE_FACTORY_SALT, REGISTRY_SALT},
    objects::{module::ModuleInfo, module_reference::ModuleReference},
    registry::{ModulesResponse, QueryMsg},
    ACCOUNT, MODULE_FACTORY, REGISTRY,
};
use axone_networks::parse_network as parse_axone_network;
use clap::Parser;
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
    daemon::{networks::ChainInfo, senders::QueryOnlyDaemon},
    prelude::*,
    tokio::runtime::Runtime,
};
use std::{collections::BTreeSet, str::FromStr};

#[derive(Debug)]
struct AbstractDeploymentInfo {
    account_code_id: u64,
    module_factory_addr: Addr,
    registry_addr: Addr,
}

fn addr_from_instantiate2(creator_addr: &str, salt: &[u8]) -> anyhow::Result<Addr> {
    let creator_account_id =
        AccountId::from_str(creator_addr).map_err(|err| anyhow::Error::msg(err.to_string()))?;
    let creator_canon = CanonicalAddr::from(creator_account_id.to_bytes());
    let contract_canon = instantiate2_address(&BLOB_CHECKSUM, &creator_canon, salt)?;
    let contract_addr = AccountId::new(creator_account_id.prefix(), contract_canon.as_slice())
        .map_err(|err| anyhow::Error::msg(err.to_string()))?;
    Ok(Addr::unchecked(contract_addr.to_string()))
}

fn discover_blob_creators(chain: &QueryOnlyDaemon) -> anyhow::Result<BTreeSet<String>> {
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

fn discover_abstract_deployment(chain: &QueryOnlyDaemon) -> anyhow::Result<AbstractDeploymentInfo> {
    let wasm = chain.wasm_querier();

    for creator_addr in discover_blob_creators(chain)? {
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

        let account_code_id = match module_response.module.reference {
            ModuleReference::Account(code_id) => code_id,
            ref reference => {
                anyhow::bail!("unexpected account module reference: {reference:?}");
            }
        };

        return Ok(AbstractDeploymentInfo {
            account_code_id,
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

fn print_abstract_info(networks: Vec<(String, ChainInfo)>) -> anyhow::Result<()> {
    for (index, (network_id, network)) in networks.into_iter().enumerate() {
        let rt = Runtime::new()?;
        let chain = DaemonBuilder::new(network.clone())
            .handle(rt.handle())
            .is_test(true)
            .build_sender(())?;
        let deployment = discover_abstract_deployment(&chain)?;

        if index > 0 {
            println!();
        }

        println!("NETWORK: {network_id}");
        println!("CHAIN_ID: {}", network.chain_id);
        println!("ACCOUNT_CODE_ID: {}", deployment.account_code_id);
        println!("MODULE_FACTORY_ADDR: {}", deployment.module_factory_addr);
        println!("REGISTRY_ADDR: {}", deployment.registry_addr);
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Network IDs to inspect (e.g., local, testnet, mainnet)
    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    network_ids: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .try_init();

    let args = Arguments::parse();
    let networks = args
        .network_ids
        .iter()
        .map(|network_id| {
            parse_axone_network(network_id)
                .or_else(|_| networks::parse_network(network_id))
                .map(|network| (network_id.clone(), network))
                .map_err(anyhow::Error::msg)
        })
        .collect::<Result<Vec<_>, _>>()?;

    print_abstract_info(networks)
}
