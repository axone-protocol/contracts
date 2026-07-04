//! Create synthetic Abstract Accounts on a target network.

use abstract_client::AbstractClient;
use axone_networks::{
    abstract_deployment::seed_abstract_addresses, parse_network as parse_axone_network,
};
use clap::Parser;
use cw_orch::{anyhow, daemon::networks::ChainInfo, prelude::*, tokio::runtime::Runtime};
use log::info;
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_COUNT: u8 = 5;
const MAX_COUNT: u8 = 50;
const DEFAULT_NAME_PREFIX: &str = "axone-testnet-identity";
const DEFAULT_DESCRIPTION: &str =
    "Synthetic AXONE testnet identity generated for traffic stimulation.";
const DEFAULT_LINK: &str = "https://axone.xyz/testnet";
const MAX_ABSTRACT_NAME_LENGTH: usize = 64;

fn create_abstract_accounts(network: ChainInfo, args: &Arguments) -> anyhow::Result<()> {
    info!(
        "Creating {} Abstract Account(s) on {}...",
        args.count, network.chain_id
    );

    let rt = Runtime::new()?;
    let chain = DaemonBuilder::new(network.clone())
        .handle(rt.handle())
        .build()?;

    info!("Connected to: {}", network.chain_id);
    info!("Sender: {}", chain.sender_addr());

    seed_abstract_addresses(&chain, &network, &rt)?;
    info!(
        "Seeded Abstract addresses for {} from on-chain deployment",
        network.chain_id
    );

    let abstract_client = AbstractClient::new(chain)?;
    let run_marker = run_marker();

    for index in 1..=args.count {
        let name = account_name(&args.name_prefix, &run_marker, index);
        let account = abstract_client
            .account_builder()
            .name(name.clone())
            .description(args.description.clone())
            .link(args.link.clone())
            .build()?;

        info!(
            "Created Abstract Account {}/{}: id={}, address={}",
            index,
            args.count,
            account.id()?,
            account.address()?
        );
    }

    Ok(())
}

fn parse_count(input: &str) -> Result<u8, String> {
    let count = input
        .parse::<u8>()
        .map_err(|_| format!("count must be an integer between 1 and {MAX_COUNT}"))?;

    if !(1..=MAX_COUNT).contains(&count) {
        return Err(format!("count must be between 1 and {MAX_COUNT}"));
    }

    Ok(count)
}

fn account_name(prefix: &str, run_marker: &str, index: u8) -> String {
    let suffix = format!("-{run_marker}-{index}");
    let max_prefix_len = MAX_ABSTRACT_NAME_LENGTH.saturating_sub(suffix.len());
    let prefix = truncate_ascii(prefix, max_prefix_len);

    format!("{prefix}{suffix}")
}

fn truncate_ascii(value: &str, max_len: usize) -> String {
    let mut result = String::with_capacity(max_len);
    for ch in value.chars() {
        if result.len() + ch.len_utf8() > max_len {
            break;
        }
        result.push(ch);
    }
    result
}

fn run_marker() -> String {
    if let Ok(run_id) = std::env::var("GITHUB_RUN_ID") {
        let attempt = std::env::var("GITHUB_RUN_ATTEMPT").unwrap_or_else(|_| "1".to_string());
        return format!("gh{run_id}a{attempt}");
    }

    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();
    format!("local{seconds}")
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Network ID to create accounts on (e.g., testnet, axone-dendrite-2)
    #[arg(short, long, default_value = "testnet")]
    network_id: String,
    /// Number of Abstract Accounts to create.
    #[arg(short, long, default_value_t = DEFAULT_COUNT, value_parser = parse_count)]
    count: u8,
    /// Prefix used for generated Abstract Account names.
    #[arg(long, default_value = DEFAULT_NAME_PREFIX)]
    name_prefix: String,
    /// Description stored in each Abstract Account.
    #[arg(long, default_value = DEFAULT_DESCRIPTION)]
    description: String,
    /// Link stored in each Abstract Account.
    #[arg(long, default_value = DEFAULT_LINK)]
    link: String,
}

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let args = Arguments::parse();
    let network = parse_axone_network(&args.network_id)
        .or_else(|_| networks::parse_network(&args.network_id))
        .map_err(anyhow::Error::msg)?;

    create_abstract_accounts(network, &args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_count_accepts_valid_values() {
        assert_eq!(parse_count("1"), Ok(1));
        assert_eq!(parse_count("5"), Ok(5));
        assert_eq!(parse_count("50"), Ok(50));
    }

    #[test]
    fn parse_count_rejects_invalid_values() {
        assert!(parse_count("0").is_err());
        assert!(parse_count("51").is_err());
        assert!(parse_count("abc").is_err());
    }

    #[test]
    fn account_name_includes_run_marker_and_index() {
        assert_eq!(
            account_name("axone-testnet-identity", "gh123a1", 5),
            "axone-testnet-identity-gh123a1-5"
        );
    }

    #[test]
    fn account_name_is_capped_to_abstract_limit() {
        let name = account_name(
            "a-very-long-prefix-that-would-exceed-the-abstract-account-name-limit",
            "gh123456789a1",
            50,
        );

        assert!(name.len() <= MAX_ABSTRACT_NAME_LENGTH);
        assert!(name.ends_with("-gh123456789a1-50"));
    }

    #[test]
    fn account_name_is_capped_to_abstract_limit_with_multibyte_prefix() {
        let name = account_name("identity-🧪🧪🧪🧪🧪🧪🧪🧪🧪🧪", "gh123456789a1", 50);

        assert!(name.len() <= MAX_ABSTRACT_NAME_LENGTH);
        assert!(name.ends_with("-gh123456789a1-50"));
        assert!(name.is_char_boundary(name.len()));
    }
}
