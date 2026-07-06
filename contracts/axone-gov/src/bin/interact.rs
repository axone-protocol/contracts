//! Interact with an axone-gov module installed on an Abstract Account.

use abstract_app::objects::AccountId;
use abstract_client::{AbstractClient, Application};
use axone_gov::{
    contract::interface::AxoneGovInterface,
    msg::{
        AxoneGovExecuteMsg, AxoneGovQueryMsg, ConstitutionResponse, ConstitutionStatusResponse,
        DecideResponse, DecisionResponse, ExecuteMsg as AxoneGovExecuteEndpointMsg,
        QueryMsg as AxoneGovQueryEndpointMsg,
    },
};
use axone_networks::{
    abstract_deployment::seed_abstract_addresses, parse_network as parse_axone_network,
};
use clap::{Parser, Subcommand};
use cosmwasm_std::Binary;
use cw_orch::{
    anyhow,
    daemon::{networks::ChainInfo, Daemon},
    prelude::*,
    tokio::runtime::Runtime,
};
use log::info;
use std::{path::PathBuf, str::FromStr};

fn interact(network: ChainInfo, args: Arguments) -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    let chain = DaemonBuilder::new(network.clone())
        .handle(rt.handle())
        .build()?;

    info!("Connected to: {}", network.chain_id);
    info!("Sender: {}", chain.sender_addr());

    seed_abstract_addresses(&chain, &network, &rt)?;

    let abstract_client: AbstractClient<Daemon> = AbstractClient::new(chain.clone())?;
    let account = abstract_client.fetch_account(args.account_id.clone())?;
    let app: Application<Daemon, AxoneGovInterface<Daemon>> = account.application()?;

    info!("Account address: {}", app.account().address()?);
    info!("axone-gov address: {}", app.address()?);

    match args.command {
        Command::Status => {
            let response: ConstitutionStatusResponse =
                app.query(&module_query(AxoneGovQueryMsg::ConstitutionStatus {}))?;
            println!("constitution_revision: {}", response.constitution_revision);
            println!(
                "constitution_hash: {}",
                binary_to_lower_hex(&response.constitution_hash)
            );
        }
        Command::Constitution => {
            let response: ConstitutionResponse =
                app.query(&module_query(AxoneGovQueryMsg::Constitution {}))?;
            println!(
                "{}",
                String::from_utf8_lossy(response.constitution.as_slice())
            );
        }
        Command::Decide { case, motivated } => {
            let response: DecideResponse = app.query(&module_query(AxoneGovQueryMsg::Decide {
                case,
                motivated: Some(motivated),
            }))?;
            print_decide_response(&response);
        }
        Command::RecordDecision { case, motivated } => {
            app.execute(
                &module_execute(AxoneGovExecuteMsg::RecordDecision {
                    case,
                    motivated: Some(motivated),
                }),
                &[],
            )?;
            println!("record_decision: submitted");
        }
        Command::ReviseConstitution {
            constitution_file,
            case,
        } => {
            let constitution = Binary::from(std::fs::read(&constitution_file)?);
            app.execute(
                &module_execute(AxoneGovExecuteMsg::ReviseConstitution { constitution, case }),
                &[],
            )?;
            println!("revise_constitution: submitted");
        }
        Command::Decision { decision_id } => {
            let response: DecisionResponse =
                app.query(&module_query(AxoneGovQueryMsg::Decision { decision_id }))?;
            print_decision_response(&response);
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Network ID to interact with (e.g., local, testnet, mainnet).
    #[arg(short, long, default_value = "testnet")]
    network_id: String,
    /// Abstract Account ID that has axone-gov installed (for example, local-42).
    #[arg(long, value_parser = parse_account_id)]
    account_id: AccountId,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print the current constitution revision and hash.
    Status,
    /// Print the stored Prolog constitution.
    Constitution,
    /// Query a governance decision without recording it on-chain.
    Decide {
        /// Prolog case term, typically ctx{...}.
        #[arg(long)]
        case: String,
        /// Ask for motivation through governance:decide/3.
        #[arg(long, default_value_t = false)]
        motivated: bool,
    },
    /// Record a governance decision on-chain.
    RecordDecision {
        /// Prolog case term, typically ctx{...}.
        #[arg(long)]
        case: String,
        /// Ask for motivation through governance:decide/3.
        #[arg(long, default_value_t = false)]
        motivated: bool,
    },
    /// Propose and execute a constitution revision.
    ReviseConstitution {
        /// Path to the proposed Prolog constitution file.
        #[arg(long)]
        constitution_file: PathBuf,
        /// Optional Prolog case term merged into the revision case.
        #[arg(long)]
        case: Option<String>,
    },
    /// Print a recorded decision by id.
    Decision {
        /// Decision identifier to query.
        #[arg(long)]
        decision_id: u64,
    },
}

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let args = Arguments::parse();
    let network = parse_axone_network(&args.network_id)
        .or_else(|_| cw_orch::daemon::networks::parse_network(&args.network_id))
        .map_err(anyhow::Error::msg)?;

    interact(network, args)
}

fn parse_account_id(input: &str) -> Result<AccountId, String> {
    AccountId::from_str(input).map_err(|err| err.to_string())
}

fn module_query(query: AxoneGovQueryMsg) -> AxoneGovQueryEndpointMsg {
    AxoneGovQueryEndpointMsg::Module(query)
}

fn module_execute(execute: AxoneGovExecuteMsg) -> AxoneGovExecuteEndpointMsg {
    AxoneGovExecuteEndpointMsg::Module(execute)
}

fn print_decide_response(response: &DecideResponse) {
    println!("verdict: {}", response.verdict);
    if let Some(motivation) = &response.motivation {
        println!("motivation: {motivation}");
    }
}

fn print_decision_response(response: &DecisionResponse) {
    println!("decision_id: {}", response.decision_id);
    println!("constitution_revision: {}", response.constitution_revision);
    println!(
        "constitution_hash: {}",
        binary_to_lower_hex(&response.constitution_hash)
    );
    println!("case_hash: {}", binary_to_lower_hex(&response.case_hash));
    println!("verdict: {}", response.verdict);
    println!(
        "verdict_hash: {}",
        binary_to_lower_hex(&response.verdict_hash)
    );
    if let Some(motivation) = &response.motivation {
        println!("motivation: {motivation}");
    }
    if let Some(motivation_hash) = &response.motivation_hash {
        println!("motivation_hash: {}", binary_to_lower_hex(motivation_hash));
    }
    println!("author: {}", response.author);
    println!("block_height: {}", response.block_height);
    println!("block_time_seconds: {}", response.block_time_seconds);
}

fn binary_to_lower_hex(binary: &Binary) -> String {
    let mut result = String::with_capacity(binary.len() * 2);
    for byte in binary.as_slice() {
        result.push(hex_char(byte >> 4));
        result.push(hex_char(byte & 0x0f));
    }
    result
}

fn hex_char(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=15 => (b'a' + value - 10) as char,
        _ => unreachable!("nibble is always in 0..=15"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_account_id_accepts_local_ids() {
        assert_eq!(
            parse_account_id("local-42").unwrap().to_string(),
            "local-42"
        );
    }

    #[test]
    fn parse_account_id_rejects_invalid_ids() {
        assert!(parse_account_id("42").is_err());
    }

    #[test]
    fn binary_to_lower_hex_formats_bytes() {
        assert_eq!(
            binary_to_lower_hex(&Binary::from(vec![0x00, 0x0f, 0xa5, 0xff])),
            "000fa5ff"
        );
    }
}
