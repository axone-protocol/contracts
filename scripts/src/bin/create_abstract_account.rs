//! Create one synthetic Abstract Account on a target network.

use axone_networks::{
    abstract_deployment::seed_abstract_addresses, parse_network as parse_axone_network,
};
use axone_scripts::synthetic_account::{
    self, DEFAULT_DESCRIPTION, DEFAULT_LINK, DEFAULT_NAME_PREFIX,
};
use clap::Parser;
use cw_orch::{anyhow, daemon::networks::ChainInfo, prelude::*, tokio::runtime::Runtime};
use log::info;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Arguments {
    /// Network ID to create an account on (e.g., testnet, axone-dendrite-2).
    #[arg(short, long, default_value = "testnet")]
    network_id: String,
    /// Prefix used for the generated Abstract Account name.
    #[arg(long, default_value = DEFAULT_NAME_PREFIX)]
    name_prefix: String,
    /// Unique marker appended to the generated name.
    #[arg(long)]
    run_marker: Option<String>,
    /// Description stored in the Abstract Account.
    #[arg(long, default_value = DEFAULT_DESCRIPTION)]
    description: String,
    /// Link stored in the Abstract Account.
    #[arg(long, default_value = DEFAULT_LINK)]
    link: String,
}

fn create_abstract_account(network: ChainInfo, args: &Arguments) -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    let chain = DaemonBuilder::new(network.clone())
        .handle(rt.handle())
        .build()?;
    seed_abstract_addresses(&chain, &network, &rt)?;

    let client = abstract_client::AbstractClient::new(chain)?;
    let marker = args
        .run_marker
        .clone()
        .unwrap_or_else(synthetic_account::default_marker);
    let account = synthetic_account::create(
        &client,
        &args.name_prefix,
        &marker,
        &args.description,
        &args.link,
    )?;

    info!(
        "Created Abstract Account: id={}, address={}",
        account.id()?,
        account.address()?
    );
    Ok(())
}

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let args = Arguments::parse();
    let network = parse_axone_network(&args.network_id)
        .or_else(|_| networks::parse_network(&args.network_id))
        .map_err(anyhow::Error::msg)?;
    create_abstract_account(network, &args)
}
