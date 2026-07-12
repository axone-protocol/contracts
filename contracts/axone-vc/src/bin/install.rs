//! Install the axone-vc module on an Abstract Account.

use axone_vc::{contract::interface::AxoneVcInterface, msg::AxoneVcInstantiateMsg, AXONE_VC_ID};

use abstract_app::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Application};
use axone_networks::parse_network as parse_axone_network;
use clap::Parser;
use cw_orch::{anyhow, daemon::networks::ChainInfo, prelude::*, tokio::runtime::Runtime};
use log::info;

fn install(networks: Vec<ChainInfo>) -> anyhow::Result<()> {
    for network in networks {
        info!("Installing axone-vc on {}...", network.chain_id);

        let rt = Runtime::new()?;
        let chain = DaemonBuilder::new(network.clone())
            .handle(rt.handle())
            .build()?;

        let abstract_client: AbstractClient<Daemon> = AbstractClient::new(chain)?;

        let app_namespace = Namespace::from_id(AXONE_VC_ID)?;
        let account = abstract_client.fetch_or_build_account(app_namespace.clone(), |builder| {
            builder.namespace(app_namespace.clone())
        })?;

        let app: Application<Daemon, AxoneVcInterface<Daemon>> =
            account.install_app::<AxoneVcInterface<_>>(&AxoneVcInstantiateMsg {}, &[])?;

        info!("axone-vc installed at {}", app.address()?);
    }

    Ok(())
}

#[derive(Debug, Default, Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
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
        .map(|n| parse_axone_network(n).or_else(|_| networks::parse_network(n)))
        .collect::<Result<Vec<_>, _>>()
        .map_err(anyhow::Error::msg)?;

    install(networks)
}
