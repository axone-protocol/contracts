use cw_storage_plus::Item;

#[cosmwasm_schema::cw_serde]
pub struct Config {}

pub const CONFIG: Item<Config> = Item::new("config");
pub const COUNT: Item<i32> = Item::new("count");
