use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

pub const DATAVERSE: Item<Dataverse> = Item::new("dataverse");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Dataverse {
    pub name: String,
    pub triplestore_address: Addr,
}
