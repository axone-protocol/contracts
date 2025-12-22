//! Install the axone-gov module on an Abstract Account.
//!
//! This script installs (instantiates) the axone-gov module on an Abstract Account.
//! The module must be published first using the publish script.
//! ```

use axone_gov::{
    contract::interface::AxoneGovInterface,
    msg::{AxoneGovExecuteMsgFns, AxoneGovInstantiateMsg, AxoneGovQueryMsgFns},
    AXONE_GOV_ID,
};

use abstract_app::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Application};
use axone_networks::parse_network as parse_axone_network;
use clap::Parser;
use cw_orch::{anyhow, daemon::networks::ChainInfo, prelude::*, tokio::runtime::Runtime};
use log::info;

fn install(networks: Vec<ChainInfo>) -> anyhow::Result<()> {
    for network in networks {
        info!("📥 Installing axone-gov on {}...", network.chain_id);

        let rt = Runtime::new()?;
        let chain = DaemonBuilder::new(network.clone())
            .handle(rt.handle())
            .build()?;

        info!("   Connected to: {}", network.chain_id);
        info!("   Sender: {}", chain.sender_addr());

        let abstract_client: AbstractClient<Daemon> = AbstractClient::new(chain.clone())?;

        let app_namespace = Namespace::from_id(AXONE_GOV_ID)?;
        let account = abstract_client.fetch_or_build_account(app_namespace, |builder| {
            builder.namespace(Namespace::from_id(AXONE_GOV_ID).unwrap())
        })?;

        info!("📦 Account address: {}", account.address()?);

        info!("📥 Installing axone-gov module...");
        let app: Application<Daemon, AxoneGovInterface<Daemon>> = account
            .install_app::<AxoneGovInterface<_>>(&AxoneGovInstantiateMsg { count: 0 }, &[])?;

        info!("✅ axone-gov installed successfully!");
        info!("Module details:");
        info!("   Address: {}", app.address()?);
        info!("   Account: {}", account.address()?);

        info!("🧪 Testing module functionality...");
        let count = app.count()?;
        info!("   Initial count: {}", count.count);

        info!("   Incrementing...");
        app.increment()?;

        let count = app.count()?;
        info!("   Count after increment: {}", count.count);

        info!("✅ Module is working correctly!");
        info!(
            "You can now interact with the module at: {}",
            app.address()?
        );
    }
    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Network IDs to install on (e.g., local, testnet, mainnet)
    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    network_ids: Vec<String>,
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let args = Arguments::parse();
    let networks = args
        .network_ids
        .iter()
        .map(|n| parse_axone_network(n).or_else(|_| cw_orch::daemon::networks::parse_network(n)))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    install(networks).unwrap();
}
