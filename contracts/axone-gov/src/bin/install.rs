//! Install the axone-gov module on an Abstract Account.
//!
//! This script installs (instantiates) the axone-gov module on an Abstract Account.
//! The module must be published first using the publish script.
//! ```

use axone_gov::{contract::interface::AxoneGovInterface, msg::AxoneGovInstantiateMsg};

use abstract_app::objects::AccountId;
use abstract_client::{AbstractClient, Application};
use axone_networks::{
    abstract_deployment::seed_abstract_addresses, parse_network as parse_axone_network,
};
use clap::{ArgGroup, Parser};
use cosmwasm_std::Binary;
use cw_orch::{daemon::networks::ChainInfo, prelude::*, tokio::runtime::Runtime};
use log::info;
use std::{path::PathBuf, str::FromStr};

fn install(networks: Vec<ChainInfo>, account_id: AccountId, constitution: Binary) {
    for network in networks {
        info!("📥 Installing axone-gov on {}...", network.chain_id);

        let rt = Runtime::new().expect("Failed to create tokio runtime");
        let chain = DaemonBuilder::new(network.clone())
            .handle(rt.handle())
            .build()
            .expect("Failed to build daemon connection");

        info!("   Connected to: {}", network.chain_id);
        info!("   Sender: {}", chain.sender_addr());

        seed_abstract_addresses(&chain, &network, &rt).unwrap_or_else(|err| {
            panic!(
                "Failed to seed Abstract addresses for {} from on-chain deployment: {}",
                network.chain_id, err
            )
        });

        let abstract_client: AbstractClient<Daemon> =
            AbstractClient::new(chain.clone()).expect("Failed to connect to Abstract client");

        let account = abstract_client
            .fetch_account(account_id.clone())
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to fetch target Abstract Account on {}: {}",
                    network.chain_id, err
                )
            });

        info!(
            "📦 Account address: {}",
            account.address().expect("Failed to get account address")
        );

        info!("📥 Installing axone-gov module...");
        let app: Application<Daemon, AxoneGovInterface<Daemon>> = account
            .install_app::<AxoneGovInterface<_>>(
                &AxoneGovInstantiateMsg {
                    constitution: constitution.clone(),
                },
                &[],
            )
            .expect("Failed to install axone-gov module");

        info!("✅ axone-gov installed successfully!");
        info!("Module details:");
        info!(
            "   Address: {}",
            app.address().expect("Failed to get app address")
        );
        info!(
            "   Account: {}",
            account.address().expect("Failed to get account address")
        );

        info!("✅ Module installed correctly!");
        info!(
            "You can now interact with the module at: {}",
            app.address().expect("Failed to get app address")
        );
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(group = ArgGroup::new("constitution_source")
    .required(true)
    .multiple(false)
    .args(&["constitution_file", "constitution"])
)]
struct Arguments {
    /// Network IDs to install on (e.g., local, testnet, mainnet)
    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    network_ids: Vec<String>,
    /// Abstract Account ID to govern (for example, local-42).
    #[arg(long, value_parser = parse_account_id)]
    account_id: AccountId,
    /// Path to a Prolog constitution file.
    #[arg(long)]
    constitution_file: Option<PathBuf>,
    /// Inline Prolog constitution source.
    #[arg(long)]
    constitution: Option<String>,
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
        .expect("Failed to parse network IDs. Please check your network configuration.");

    let constitution = load_constitution(&args);

    install(networks, args.account_id, constitution);
}

fn parse_account_id(input: &str) -> Result<AccountId, String> {
    AccountId::from_str(input).map_err(|err| err.to_string())
}

fn load_constitution(args: &Arguments) -> Binary {
    if let Some(path) = &args.constitution_file {
        info!("📜 Using constitution file: {}", path.display());
        let data = std::fs::read(path).expect("Failed to read constitution file");
        return Binary::from(data);
    }

    if let Some(text) = &args.constitution {
        info!("📜 Using inline constitution program");
        return Binary::from(text.clone().into_bytes());
    }

    unreachable!("constitution source is enforced by clap");
}
