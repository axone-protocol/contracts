//! Deploy Abstract infrastructure to a network.
//!
//! This script deploys the core Abstract contracts (registry, account factory, module factory, etc.)
//! to the specified network. This must be done before publishing any Abstract modules.

use abstract_interface::Abstract;
use axone_networks::parse_network as parse_axone_network;
use clap::Parser;
use cw_orch::{anyhow, daemon::networks::ChainInfo, prelude::*, tokio::runtime::Runtime};
use log::info;

fn deploy_abstract(networks: Vec<ChainInfo>) -> anyhow::Result<()> {
    for network in networks {
        info!(
            "🚀 Deploying Abstract infrastructure to {}...",
            network.chain_id
        );

        let rt = Runtime::new()?;
        let chain = DaemonBuilder::new(network.clone())
            .handle(rt.handle())
            .build()?;

        info!("   Connected to: {}", network.chain_id);
        info!("   Sender: {}", chain.sender_addr());

        // Deploy Abstract infrastructure - this uploads and instantiates all core contracts
        info!("📦 Deploying Abstract core contracts...");
        let _abstr = Abstract::deploy_on(chain.clone(), ())?;

        info!("✅ Abstract infrastructure deployed successfully!");
    }
    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Network IDs to deploy to (e.g., local, testnet, mainnet)
    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    network_ids: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let args = Arguments::parse();
    let networks = args
        .network_ids
        .iter()
        .map(|n| {
            parse_axone_network(n)
                .or_else(|_| networks::parse_network(n))
                .map_err(anyhow::Error::msg)
        })
        .collect::<Result<Vec<_>, _>>()?;

    deploy_abstract(networks)
}
