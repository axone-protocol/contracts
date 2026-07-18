use crate::domain::Authority;
use crate::error::AxoneVcError;
use crate::index::OneToManyIndex;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Order, StdError, StdResult, Storage, Timestamp};
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, Item, Map, MultiIndex};

const AUTHORITY: Item<Authority> = Item::new("authority");
const REVOKED_CREDENTIALS: Map<&str, CredentialTombstone> = Map::new("revoked_credentials");

const CREDENTIALS: IndexedMap<&str, CredentialRecord, CredentialIndexes<'static>> = IndexedMap::new(
    "credentials",
    CredentialIndexes {
        subject: MultiIndex::new(
            |_, r| r.subject.clone(),
            "credentials",
            "credentials__subject",
        ),
        credential_type: OneToManyIndex::new(
            |_, r| r.types.as_slice(),
            "credentials",
            "credentials__type",
        ),
    },
);

pub struct CredentialIndexes<'a> {
    pub subject: MultiIndex<'a, String, CredentialRecord, &'a str>,
    pub credential_type: OneToManyIndex<'a, String, CredentialRecord, &'a str>,
}

impl IndexList<CredentialRecord> for CredentialIndexes<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<CredentialRecord>> + '_> {
        let indexes: Vec<&dyn Index<CredentialRecord>> = vec![&self.subject, &self.credential_type];
        Box::new(indexes.into_iter())
    }
}

#[cw_serde]
pub struct CredentialRecord {
    pub canonical_nquads: String,
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub types: Vec<String>,
    #[serde(default)]
    pub valid_from: Option<Timestamp>,
    #[serde(default)]
    pub valid_until: Option<Timestamp>,
}

impl CredentialRecord {
    pub fn new(
        canonical_nquads: String,
        subject: String,
        types: Vec<String>,
        valid_from: Option<Timestamp>,
        valid_until: Option<Timestamp>,
    ) -> Self {
        Self {
            canonical_nquads,
            subject,
            types,
            valid_from,
            valid_until,
        }
    }
}

#[cw_serde]
#[derive(Default)]
pub struct CredentialTombstone {}

impl CredentialTombstone {
    pub fn new() -> Self {
        Self::default()
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

pub fn credential(
    storage: &dyn Storage,
    credential_id: &str,
) -> Result<Option<CredentialRecord>, AxoneVcError> {
    Ok(CREDENTIALS.may_load(storage, credential_id)?)
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
    CREDENTIALS.remove(storage, credential_id)?;
    REVOKED_CREDENTIALS.save(storage, credential_id, tombstone)?;
    Ok(())
}

#[cfg(test)]
pub fn load_credential(
    storage: &dyn Storage,
    credential_id: &str,
) -> Result<CredentialRecord, AxoneVcError> {
    credential(storage, credential_id)?
        .ok_or_else(|| StdError::not_found("CredentialRecord").into())
}

#[cfg(test)]
mod tests {
    use super::CredentialRecord;
    use cosmwasm_std::from_json;

    #[test]
    fn credential_record_without_validity_bounds_deserializes_as_unbounded() {
        let record: CredentialRecord = from_json(br#"{"canonical_nquads":"<credential>"}"#)
            .expect("legacy credential record should deserialize");

        assert_eq!(record.subject, "");
        assert!(record.types.is_empty());
        assert_eq!(record.valid_from, None);
        assert_eq!(record.valid_until, None);
    }
}
