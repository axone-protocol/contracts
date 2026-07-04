//! List Abstract deployment information for one or more networks.

use axone_networks::{
    abstract_deployment::discover_abstract_deployment, parse_network as parse_axone_network,
};
use clap::Parser;
use cw_orch::{anyhow, daemon::networks::ChainInfo, prelude::*, tokio::runtime::Runtime};

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
