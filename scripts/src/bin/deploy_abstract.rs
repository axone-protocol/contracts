//! Deploy Abstract infrastructure to a network.
//!
//! This script deploys the core Abstract contracts (registry, account factory, module factory, etc.)
//! to the specified network. This must be done before publishing any Abstract modules.

use abstract_interface::Abstract;
use axone_networks::{
    abstract_deployment::{discover_abstract_deployment_from_creators, discover_blob_creators},
    parse_network as parse_axone_network,
};
use clap::Parser;
use cw_orch::{anyhow, daemon::networks::ChainInfo, prelude::*, tokio::runtime::Runtime};
use log::info;

fn is_abstract_deployed(network: &ChainInfo, rt: &Runtime) -> anyhow::Result<bool> {
    let chain = DaemonBuilder::new(network.clone())
        .handle(rt.handle())
        .is_test(true)
        .build_sender(())?;

    let creators = discover_blob_creators(&chain)?;
    info!(
        "   Discovered {} blob creator(s) for Abstract",
        creators.len()
    );
    if creators.is_empty() {
        return Ok(false);
    }

    match discover_abstract_deployment_from_creators(&chain, creators) {
        Ok(deployment) => {
            info!(
                "   Found Abstract deployment for creator {}",
                deployment.creator_addr
            );
            Ok(true)
        }
        Err(_) => Ok(false),
    }
}

fn ensure_abstract_deployed(network: &ChainInfo, rt: &Runtime) -> anyhow::Result<()> {
    if is_abstract_deployed(network, rt)? {
        info!("✅ Abstract infrastructure already deployed on this chain");
        return Ok(());
    }

    let chain = DaemonBuilder::new(network.clone())
        .handle(rt.handle())
        .build()?;

    info!("📦 Deploying Abstract core contracts...");
    if let Err(err) = Abstract::deploy_on(chain, ()) {
        if is_abstract_deployed(network, rt)? {
            info!("✅ Abstract infrastructure is available after the deployment attempt");
            return Ok(());
        }

        return Err(anyhow::Error::new(err).context("failed to deploy Abstract infrastructure"));
    }

    info!("✅ Abstract infrastructure deployed successfully!");
    Ok(())
}

fn deploy_abstract(networks: Vec<ChainInfo>) -> anyhow::Result<()> {
    for network in networks {
        info!(
            "🚀 Deploying Abstract infrastructure to {}...",
            network.chain_id
        );

        let rt = Runtime::new()?;

        info!("   Connected to: {}", network.chain_id);

        ensure_abstract_deployed(&network, &rt)?;
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
