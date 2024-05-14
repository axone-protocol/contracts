use serde::{Deserialize, Serialize};

use crate::msg::ProgramResponse;
use cw_storage_plus::{Item, Map};
use okp4_objectarium_client::ObjectRef;

/// State to store context during contract instantiation
pub const INSTANTIATE_CONTEXT: Item<'_, String> = Item::new("instantiate");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LawStone {
    pub broken: bool,
    pub law: ObjectRef,
}

impl From<LawStone> for ProgramResponse {
    fn from(value: LawStone) -> ProgramResponse {
        ProgramResponse {
            object_id: value.law.object_id,
            storage_address: value.law.storage_address,
        }
    }
}

pub const PROGRAM: Item<'_, LawStone> = Item::new("program");

pub const DEPENDENCIES: Map<'_, &str, ObjectRef> = Map::new("dependencies");
