use crate::domain::Authority;
use crate::error::AxoneVcError;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdError, Storage};
use cw_storage_plus::{Item, Map};

const AUTHORITY: Item<Authority> = Item::new("authority");
const CREDENTIALS: Map<&str, CredentialRecord> = Map::new("credentials");

#[cw_serde]
pub struct CredentialRecord {
    pub canonical_nquads: String,
}

impl CredentialRecord {
    pub fn new(canonical_nquads: String) -> Self {
        Self { canonical_nquads }
    }
}

pub fn authority(storage: &dyn Storage) -> Result<Authority, AxoneVcError> {
    Ok(AUTHORITY.load(storage)?)
}

pub fn initialize_authority(
    storage: &mut dyn Storage,
    authority: &Authority,
) -> Result<(), AxoneVcError> {
    if AUTHORITY.may_load(storage)?.is_some() {
        return Err(StdError::generic_err("authority already initialized").into());
    }

    AUTHORITY.save(storage, authority)?;
    Ok(())
}

pub fn has_credential(storage: &dyn Storage, credential_id: &str) -> bool {
    CREDENTIALS.has(storage, credential_id)
}

pub fn record_credential(
    storage: &mut dyn Storage,
    credential_id: &str,
    record: &CredentialRecord,
) -> Result<(), AxoneVcError> {
    CREDENTIALS.save(storage, credential_id, record)?;
    Ok(())
}

#[cfg(test)]
pub fn load_credential(
    storage: &dyn Storage,
    credential_id: &str,
) -> Result<CredentialRecord, AxoneVcError> {
    Ok(CREDENTIALS.load(storage, credential_id)?)
}
