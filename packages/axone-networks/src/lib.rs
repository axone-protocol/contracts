#![cfg(not(target_arch = "wasm32"))]

//! Axone network definitions for cw-orchestrator.
//!
//! This module provides network configurations for Axone blockchain networks (local, testnet, mainnet).
//! These configurations can be used with cw-orchestrator's Daemon for deployment and interaction.

use cw_orch::daemon::networks::{ChainInfo, ChainKind, NetworkInfo};

/// Axone local network (for development)
pub const AXONE_LOCAL: ChainInfo = ChainInfo {
    kind: ChainKind::Local,
    chain_id: "axone-localnet",
    gas_denom: "uaxone",
    gas_price: 0.025,
    grpc_urls: &["http://localhost:9091"],
    network_info: NetworkInfo {
        chain_name: "axone-localnet",
        pub_address_prefix: "axone",
        coin_type: 118u32,
    },
    lcd_url: Some("http://localhost:1317"),
    fcd_url: None,
};

/// Axone testnet
pub const AXONE_TESTNET: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "axone-dentrite-1",
    gas_denom: "uaxone",
    gas_price: 0.025,
    grpc_urls: &["https://grpc.testnet.axone.xyz:443"],
    network_info: NetworkInfo {
        chain_name: "axone-testnet",
        pub_address_prefix: "axone",
        coin_type: 118u32,
    },
    lcd_url: Some("https://api.testnet.axone.xyz"),
    fcd_url: None,
};

/// Axone mainnet
pub const AXONE_MAINNET: ChainInfo = ChainInfo {
    kind: ChainKind::Mainnet,
    chain_id: "axone-1",
    gas_denom: "uaxone",
    gas_price: 0.025,
    grpc_urls: &["https://grpc.mainnet.axone.xyz:443"],
    network_info: NetworkInfo {
        chain_name: "axone",
        pub_address_prefix: "axone",
        coin_type: 118u32,
    },
    lcd_url: Some("https://api.mainnet.axone.xyz"),
    fcd_url: None,
};

/// Parse an Axone network by its identifier.
///
/// Supported identifiers:
/// - "axone-localnet", "local" → Local development network
/// - "axone-dentrite-1", "testnet", "axone-testnet" → Testnet
/// - "axone-1", "mainnet", "axone-mainnet" → Mainnet
///
/// # Errors
///
/// Returns an error if the network identifier is not recognized.
pub fn parse_network(network_id: &str) -> anyhow::Result<ChainInfo> {
    match network_id.to_lowercase().as_str() {
        "axone-localnet" | "local" => Ok(AXONE_LOCAL),
        "axone-dentrite-1" | "testnet" | "axone-testnet" => Ok(AXONE_TESTNET),
        "axone-1" | "mainnet" | "axone-mainnet" => Ok(AXONE_MAINNET),
        _ => Err(anyhow::anyhow!(
            "Unknown Axone network: {}. Supported: local, testnet, mainnet, axone-localnet, axone-dentrite-1, axone-1",
            network_id
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_local_network() {
        let network = parse_network("local").unwrap();
        assert_eq!(network.chain_id, "axone-localnet");
        assert_eq!(network.gas_denom, "uaxone");

        let network = parse_network("axone-localnet").unwrap();
        assert_eq!(network.chain_id, "axone-localnet");
    }

    #[test]
    fn test_parse_testnet() {
        let network = parse_network("testnet").unwrap();
        assert_eq!(network.chain_id, "axone-dentrite-1");

        let network = parse_network("axone-testnet").unwrap();
        assert_eq!(network.chain_id, "axone-dentrite-1");

        let network = parse_network("axone-dentrite-1").unwrap();
        assert_eq!(network.chain_id, "axone-dentrite-1");
    }

    #[test]
    fn test_parse_mainnet() {
        let network = parse_network("mainnet").unwrap();
        assert_eq!(network.chain_id, "axone-1");

        let network = parse_network("axone-mainnet").unwrap();
        assert_eq!(network.chain_id, "axone-1");

        let network = parse_network("axone-1").unwrap();
        assert_eq!(network.chain_id, "axone-1");
    }

    #[test]
    fn test_parse_unknown_network() {
        let result = parse_network("unknown-network");
        assert!(result.is_err());
    }
}
