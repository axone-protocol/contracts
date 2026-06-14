use crate::{contract::AxoneVcResult, domain::Authority, state};

use cosmwasm_std::{Addr, Storage};

pub fn initialize_authority(
    storage: &mut dyn Storage,
    chain_id: &str,
    account_addr: &Addr,
) -> AxoneVcResult<Authority> {
    let authority = Authority::new(chain_id, account_addr)?;
    state::initialize_authority(storage, &authority)?;

    Ok(authority)
}

pub fn authority(storage: &dyn Storage) -> AxoneVcResult<Authority> {
    state::authority(storage)
}

#[cfg(test)]
mod tests {
    use super::{authority, initialize_authority};
    use bech32::{Bech32, Hrp};
    use cosmwasm_std::{testing::mock_dependencies, Addr};

    #[test]
    fn initialize_authority_persists_canonical_authority() {
        let mut deps = mock_dependencies();
        let payload = [0x42; 20];
        let account_addr = bech32::encode::<Bech32>(Hrp::parse("axone").unwrap(), &payload)
            .expect("valid address");

        let initialized_authority = initialize_authority(
            deps.as_mut().storage,
            "axone-localnet-1",
            &Addr::unchecked(account_addr),
        )
        .expect("authority should initialize");

        let stored = authority(deps.as_ref().storage).expect("authority should be stored");

        assert_eq!(stored.did(), initialized_authority.did());
        assert!(stored
            .did()
            .starts_with("did:pkh:cosmos:axone-localnet-1:cosmos1"));
    }

    #[test]
    fn initialize_authority_rejects_second_initialization() {
        let mut deps = mock_dependencies();
        let payload = [0x24; 20];
        let account_addr = bech32::encode::<Bech32>(Hrp::parse("axone").unwrap(), &payload)
            .expect("valid address");
        let account_addr = Addr::unchecked(account_addr);

        initialize_authority(deps.as_mut().storage, "axone-localnet-1", &account_addr)
            .expect("first initialization should succeed");

        let err = initialize_authority(deps.as_mut().storage, "axone-localnet-1", &account_addr)
            .expect_err("second initialization should fail");

        assert_eq!(
            err.to_string(),
            "Generic error: authority already initialized"
        );
    }

    #[test]
    fn authority_returns_persisted_value() {
        let mut deps = mock_dependencies();
        let payload = [0x11; 20];
        let account_addr = bech32::encode::<Bech32>(Hrp::parse("cosmos").unwrap(), &payload)
            .expect("valid address");
        let expected = initialize_authority(
            deps.as_mut().storage,
            "cosmoshub-4",
            &Addr::unchecked(account_addr),
        )
        .expect("authority should initialize");

        let stored = authority(deps.as_ref().storage).expect("authority should load");

        assert_eq!(stored.did(), expected.did());
    }
}
