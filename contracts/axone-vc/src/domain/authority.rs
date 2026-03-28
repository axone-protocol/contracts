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
    fn builds_did_using_canonical_cosmos_hrp() {
        let payload = [0x42; 20];
        let axone_addr = bech32::encode::<Bech32>(Hrp::parse("axone").unwrap(), &payload)
            .expect("valid axone bech32 address");
        let cosmos_addr = bech32::encode::<Bech32>(Hrp::parse("cosmos").unwrap(), &payload)
            .expect("valid cosmos bech32 address");

        let authority = Authority::new("axone-localnet-1", &Addr::unchecked(axone_addr))
            .expect("authority should build");

        assert_eq!(
            authority.did(),
            format!("did:pkh:cosmos:axone-localnet-1:{cosmos_addr}")
        );
    }
}
