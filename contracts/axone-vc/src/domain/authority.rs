use bech32::{Bech32, Hrp};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, StdError, StdResult};

const DID_PKH_PREFIX: &str = "did:pkh:cosmos:";
const CANONICAL_COSMOS_HRP: &str = "cosmos";

#[cw_serde]
pub struct Authority(String);

impl Authority {
    pub fn new(chain_id: &str, account_addr: &Addr) -> StdResult<Self> {
        let canonical_account_addr = canonical_cosmos_account_address(account_addr)?;
        let mut did = String::with_capacity(
            DID_PKH_PREFIX.len() + chain_id.len() + 1 + canonical_account_addr.len(),
        );
        did.push_str(DID_PKH_PREFIX);
        did.push_str(chain_id);
        did.push(':');
        did.push_str(&canonical_account_addr);

        Ok(Self(did))
    }

    pub fn did(&self) -> &str {
        &self.0
    }
}

fn canonical_cosmos_account_address(account_addr: &Addr) -> StdResult<String> {
    let (_hrp, data) = bech32::decode(account_addr.as_str())
        .map_err(|err| StdError::generic_err(err.to_string()))?;
    let hrp =
        Hrp::parse(CANONICAL_COSMOS_HRP).map_err(|err| StdError::generic_err(err.to_string()))?;

    bech32::encode::<Bech32>(hrp, &data).map_err(|err| StdError::generic_err(err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::Authority;
    use bech32::{Bech32, Hrp};
    use cosmwasm_std::Addr;

    #[test]
    fn canonical_cosmos_address_conversion() {
        let cases = vec![
            ("axone", "axone-localnet-1"),
            ("cosmos", "cosmoshub-4"),
            ("osmo", "osmosis-1"),
            ("neutron", "neutron-1"),
        ];

        let payload = [0x42; 20];

        for (input_hrp, chain_id) in cases {
            let input_addr = bech32::encode::<Bech32>(Hrp::parse(input_hrp).unwrap(), &payload)
                .expect("valid bech32 address");
            let cosmos_addr = bech32::encode::<Bech32>(Hrp::parse("cosmos").unwrap(), &payload)
                .expect("valid cosmos bech32 address");

            let authority = Authority::new(chain_id, &Addr::unchecked(input_addr));

            assert!(
                authority.is_ok(),
                "authority creation should succeed for HRP: {}",
                input_hrp
            );

            let authority = authority.unwrap();
            let expected_did = format!("did:pkh:cosmos:{}:{}", chain_id, cosmos_addr);

            assert_eq!(
                authority.did(),
                expected_did,
                "DID mismatch for HRP: {} on chain: {}",
                input_hrp,
                chain_id
            );
        }
    }

    #[test]
    fn invalid_bech32_address() {
        let cases = vec![
            "not-a-valid-address",
            "cosmos1invalid_address",
            "",
            "cosmos1",
        ];

        for invalid_addr in cases {
            let result = Authority::new("axone-localnet-1", &Addr::unchecked(invalid_addr));
            assert!(
                result.is_err(),
                "authority creation should fail for invalid address: {}",
                invalid_addr
            );
        }
    }

    #[test]
    fn did_includes_chain_id() {
        let cases = vec!["axone-localnet-1", "axone-testnet", "axone-1"];

        let payload = [0xAA; 20];
        let axone_addr = bech32::encode::<Bech32>(Hrp::parse("axone").unwrap(), &payload)
            .expect("valid axone bech32 address");
        let cosmos_addr = bech32::encode::<Bech32>(Hrp::parse("cosmos").unwrap(), &payload)
            .expect("valid cosmos bech32 address");

        for chain_id in cases {
            let authority = Authority::new(chain_id, &Addr::unchecked(&axone_addr))
                .expect("authority should build");

            let expected = format!("did:pkh:cosmos:{}:{}", chain_id, cosmos_addr);
            assert_eq!(authority.did(), expected);
        }
    }

    #[test]
    fn did_prefix_is_correct() {
        let payload = [0xFF; 20];
        let cosmos_addr = bech32::encode::<Bech32>(Hrp::parse("cosmos").unwrap(), &payload)
            .expect("valid cosmos bech32 address");
        let address = bech32::encode::<Bech32>(Hrp::parse("test").unwrap(), &payload)
            .expect("valid test bech32 address");

        let authority = Authority::new("test-chain", &Addr::unchecked(address))
            .expect("authority should build");

        assert!(
            authority.did().starts_with("did:pkh:cosmos:"),
            "DID should start with 'did:pkh:cosmos:'"
        );
        assert!(authority.did().contains("test-chain"));
        assert!(authority.did().contains(&cosmos_addr));
    }
}
