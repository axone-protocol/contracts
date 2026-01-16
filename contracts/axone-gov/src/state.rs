use crate::error::AxoneGovError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Checksum, Storage};
use cw_storage_plus::Item;

pub const CONSTITUTION: Item<Binary> = Item::new("constitution");
pub const CONSTITUTION_STATUS: Item<ConstitutionStatus> = Item::new("constitution_status");

#[cw_serde]
pub struct ConstitutionStatus {
    pub constitution_revision: u64,
    pub constitution_hash: [u8; 32],
}

impl ConstitutionStatus {
    pub fn from_constitution(constitution: &Binary) -> Self {
        let checksum = Checksum::generate(constitution.as_slice());
        Self {
            constitution_revision: 1,
            constitution_hash: *checksum.as_ref(),
        }
    }

    pub fn constitution_hash_base64(&self) -> String {
        Binary::from(self.constitution_hash).to_base64()
    }
}

/// Load the constitution from storage and convert it to a UTF-8 string.
pub fn load_constitution_as_string(storage: &dyn Storage) -> Result<String, AxoneGovError> {
    let constitution = CONSTITUTION.load(storage)?;
    std::str::from_utf8(constitution.as_slice())
        .map(ToString::to_string)
        .map_err(|err| AxoneGovError::ConstitutionUtf8(err.to_string()))
}
