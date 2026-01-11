use crate::error::AxoneGovError;
use cosmwasm_std::{Binary, Storage};
use cw_storage_plus::Item;

pub const CONSTITUTION: Item<Binary> = Item::new("constitution");

/// Load the constitution from storage and convert it to a UTF-8 string.
pub fn load_constitution_as_string(storage: &dyn Storage) -> Result<String, AxoneGovError> {
    let constitution = CONSTITUTION.load(storage)?;
    std::str::from_utf8(constitution.as_slice())
        .map(|s| s.to_string())
        .map_err(|err| {
            AxoneGovError::InvalidConstitution(format!("constitution must be valid UTF-8: {err}"))
        })
}
