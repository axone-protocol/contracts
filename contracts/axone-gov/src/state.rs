use crate::domain::constitution::ConstitutionStatus;
use crate::domain::{Constitution, Decision};
use crate::error::AxoneGovError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_hex, Addr, Binary, Checksum, OverflowError, OverflowOperation, StdError, Storage,
};
use cw_storage_plus::{Item, Map};
use getset::Getters;

pub(crate) struct StateAccess(());
impl StateAccess {
    fn new() -> Self {
        Self(())
    }
}

const CONSTITUTION: Item<Binary> = Item::new("constitution");
const CONSTITUTION_STATUS: Item<ConstitutionStatus> = Item::new("constitution_status");

const INITIAL_CONSTITUTION_REVISION: u64 = 0;
const INITIAL_DECISION_ID_COUNTER: u64 = 0;

const DECISION_ID_COUNTER: Item<u64> = Item::new("decision_id_counter");
const DECISIONS: Map<u64, DecisionRecord> = Map::new("decisions");

#[cw_serde]
#[derive(Getters)]
pub struct DecisionRecord {
    #[getset(get = "pub")]
    id: u64,
    #[getset(get = "pub")]
    constitution_revision: u64,
    #[getset(get = "pub")]
    constitution_hash: [u8; 32],
    #[getset(get = "pub")]
    case: String,
    #[getset(get = "pub")]
    case_hash: [u8; 32],
    #[getset(get = "pub")]
    verdict: String,
    #[getset(get = "pub")]
    verdict_hash: [u8; 32],
    #[getset(get = "pub")]
    motivation: Option<String>,
    #[getset(get = "pub")]
    motivation_hash: Option<[u8; 32]>,
    #[getset(get = "pub")]
    author: Addr,
    #[getset(get = "pub")]
    block_height: u64,
    #[getset(get = "pub")]
    block_time_seconds: u64,
}

impl DecisionRecord {
    pub fn constitution_hash_hex(&self) -> String {
        to_hex(self.constitution_hash)
    }

    pub fn case_hash_hex(&self) -> String {
        to_hex(self.case_hash)
    }

    pub fn verdict_hash_hex(&self) -> String {
        to_hex(self.verdict_hash)
    }

    pub fn motivation_hash_hex(&self) -> Option<String> {
        self.motivation_hash.map(to_hex)
    }
}

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

pub fn record_decision(
    storage: &mut dyn Storage,
    decision: Decision,
) -> Result<DecisionRecord, AxoneGovError> {
    let decision_id = next_decision_id(storage)?;

    let record = DecisionRecord {
        id: decision_id,
        constitution_revision: *decision.constitution_revision(),
        constitution_hash: *decision.constitution_hash(),
        case: decision.case().to_string(),
        case_hash: *Checksum::generate(decision.case().as_bytes()).as_ref(),
        verdict: decision.verdict().to_string(),
        verdict_hash: *Checksum::generate(decision.verdict().as_bytes()).as_ref(),
        motivation: decision.motivation().clone(),
        motivation_hash: decision
            .motivation()
            .as_ref()
            .map(|motivation| *Checksum::generate(motivation.as_bytes()).as_ref()),
        author: decision.author().clone(),
        block_height: *decision.height(),
        block_time_seconds: *decision.time_seconds(),
    };

    DECISIONS.save(storage, record.id, &record)?;
    Ok(record)
}

fn next_decision_id(storage: &mut dyn Storage) -> Result<u64, AxoneGovError> {
    let next = current_decision_id(storage)?
        .checked_add(1)
        .ok_or_else(|| StdError::overflow(OverflowError::new(OverflowOperation::Add)))?;

    DECISION_ID_COUNTER.save(storage, &next)?;
    Ok(next)
}

fn current_decision_id(storage: &dyn Storage) -> Result<u64, AxoneGovError> {
    let current = DECISION_ID_COUNTER
        .may_load(storage)?
        .unwrap_or(INITIAL_DECISION_ID_COUNTER);
    Ok(current)
}
