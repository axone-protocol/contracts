//! Publishes the module to the Abstract platform by uploading it and registering it on the app store.
//!
//! Info: The mnemonic used to register the module must be the same as the owner of the account that claimed the namespace.
use axone_gov::AXONE_GOV_ID;

use abstract_app::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Publisher};
use abstract_interface::Abstract;
use axone_gov::AxoneGovInterface;
use axone_networks::parse_network as parse_axone_network;
use clap::Parser;
use cw_orch::{anyhow, daemon::networks::ChainInfo, prelude::*, tokio::runtime::Runtime};
use log::{info, warn};

fn publish(networks: Vec<ChainInfo>) -> anyhow::Result<()> {
    for network in networks {
        let rt = Runtime::new()?;
        let chain = DaemonBuilder::new(network.clone())
            .handle(rt.handle())
            .build()?;

        let app_namespace = Namespace::from_id(AXONE_GOV_ID)?;

        let abstract_client: AbstractClient<Daemon> =
            AbstractClient::new(chain.clone()).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to connect to Abstract infrastructure on {}.\n\
                    Error: {}\n\n\
                    Please deploy Abstract first using:\n\
                    cargo make deploy-abstract {}",
                    network.chain_id,
                    e,
                    network.chain_id
                )
            })?;

        let publisher_acc = abstract_client
            .fetch_or_build_account(app_namespace.clone(), |builder| {
                builder.namespace(app_namespace.clone())
            })?;

        let publisher: Publisher<_> = publisher_acc.publisher()?;

        if publisher.account().owner()? != chain.sender_addr() {
            panic!("The current sender can not publish to this namespace. Please use the wallet that owns the Account that owns the Namespace.")
        }

        publisher.publish_app::<AxoneGovInterface<Daemon>>()?;

        match Abstract::load_from(chain.clone()).and_then(|abstr| {
            abstr
                .registry
                .approve_all_modules_for_namespace(app_namespace.clone())
        }) {
            Ok(_) => info!(
                "✅ Approved pending module(s) for namespace '{}' on {}",
                app_namespace, network.chain_id
            ),
            Err(err) => warn!(
                "⚠️  Skipped auto-approve for namespace '{}' on {}: {}",
                app_namespace, network.chain_id, err
            ),
        }
    }
    Ok(())
}

#[derive(Debug, Default, Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Network Id to publish on (supports: local, testnet, mainnet, axone-localnet, axone-dentrite-1, axone-1)
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
        .map(|n| parse_axone_network(n).or_else(|_| cw_orch::daemon::networks::parse_network(n)))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| anyhow::anyhow!("Invalid network id: {}", e))?;

    publish(networks)
}
