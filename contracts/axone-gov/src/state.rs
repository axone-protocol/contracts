use crate::domain::constitution::ConstitutionStatus;
use crate::domain::Constitution;
use crate::error::AxoneGovError;
use cosmwasm_std::{Binary, OverflowError, OverflowOperation, StdError, Storage};
use cw_storage_plus::Item;

pub(crate) struct StateAccess(());
impl StateAccess {
    fn new() -> Self {
        Self(())
    }
}

const CONSTITUTION: Item<Binary> = Item::new("constitution");
const CONSTITUTION_STATUS: Item<ConstitutionStatus> = Item::new("constitution_status");

const INITIAL_CONSTITUTION_REVISION: u64 = 0;

pub fn save_initial_constitution(
    storage: &mut dyn Storage,
    constitution: &Constitution,
) -> Result<ConstitutionStatus, AxoneGovError> {
    if CONSTITUTION_STATUS.may_load(storage)?.is_some() {
        return Err(StdError::generic_err("constitution already initialized").into());
    }
    let status = ConstitutionStatus::new(INITIAL_CONSTITUTION_REVISION, constitution.hash());

    CONSTITUTION.save(storage, constitution.bytes())?;
    CONSTITUTION_STATUS.save(storage, &status)?;

    Ok(status)
}

pub fn save_revised_constitution(
    storage: &mut dyn Storage,
    constitution: &Constitution,
) -> Result<ConstitutionStatus, AxoneGovError> {
    let current = load_constitution_status(storage)?;
    let next_revision = current
        .constitution_revision()
        .checked_add(1)
        .ok_or_else(|| StdError::overflow(OverflowError::new(OverflowOperation::Add)))?;
    let status = ConstitutionStatus::new(next_revision, constitution.hash());

    CONSTITUTION.save(storage, constitution.bytes())?;
    CONSTITUTION_STATUS.save(storage, &status)?;

    Ok(status)
}

pub fn load_constitution(storage: &dyn Storage) -> Result<Constitution, AxoneGovError> {
    let bytes = CONSTITUTION.load(storage)?;
    let _status = load_constitution_status(storage)?;

    Ok(Constitution::from_state(bytes, &StateAccess::new()))
}

pub fn load_constitution_status(
    storage: &dyn Storage,
) -> Result<ConstitutionStatus, AxoneGovError> {
    let status = CONSTITUTION_STATUS.load(storage)?;
    Ok(status)
}
