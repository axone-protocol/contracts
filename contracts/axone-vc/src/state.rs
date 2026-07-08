use crate::domain::Authority;
use crate::error::AxoneVcError;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdError, Storage, Timestamp};
use cw_storage_plus::{Item, Map};

const AUTHORITY: Item<Authority> = Item::new("authority");
const CREDENTIALS: Map<&str, CredentialRecord> = Map::new("credentials");
const REVOKED_CREDENTIALS: Map<&str, CredentialTombstone> = Map::new("revoked_credentials");

#[cw_serde]
pub struct CredentialRecord {
    pub canonical_nquads: String,
}

impl CredentialRecord {
    pub fn new(canonical_nquads: String) -> Self {
        Self { canonical_nquads }
    }
}

#[cw_serde]
pub struct CredentialTombstone {
    pub revoked_at: Timestamp,
}

impl CredentialTombstone {
    pub fn new(revoked_at: Timestamp) -> Self {
        Self { revoked_at }
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

pub fn is_revoked(storage: &dyn Storage, credential_id: &str) -> bool {
    REVOKED_CREDENTIALS.has(storage, credential_id)
}

pub fn revoke_credential(
    storage: &mut dyn Storage,
    credential_id: &str,
    tombstone: &CredentialTombstone,
) -> Result<(), AxoneVcError> {
    CREDENTIALS.remove(storage, credential_id);
    REVOKED_CREDENTIALS.save(storage, credential_id, tombstone)?;
    Ok(())
}

#[cfg(test)]
pub fn load_credential(
    storage: &dyn Storage,
    credential_id: &str,
) -> Result<CredentialRecord, AxoneVcError> {
    Ok(CREDENTIALS.load(storage, credential_id)?)
}
