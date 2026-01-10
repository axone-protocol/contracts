use cosmwasm_std::Binary;
use cw_storage_plus::Item;

pub const CONSTITUTION: Item<Binary> = Item::new("constitution");
