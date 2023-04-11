use crate::msg;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

pub const STORE: Item<Store> = Item::new("store");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Store {
    pub owner: Addr,
    pub limits: StoreLimits,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct StoreLimits {
    pub max_triple_count: Option<Uint128>,
    pub max_byte_size: Option<Uint128>,
    pub max_triple_byte_size: Option<Uint128>,
    pub max_query_limit: Option<Uint128>,
    pub max_query_variable_count: Option<Uint128>,
    pub max_insert_data_byte_size: Option<Uint128>,
    pub max_insert_data_triple_count: Option<Uint128>,
}

impl From<msg::StoreLimits> for StoreLimits {
    fn from(value: msg::StoreLimits) -> Self {
        StoreLimits {
            max_triple_count: value.max_triple_count,
            max_byte_size: value.max_byte_size,
            max_triple_byte_size: value.max_triple_byte_size,
            max_query_limit: value.max_query_limit,
            max_query_variable_count: value.max_query_variable_count,
            max_insert_data_byte_size: value.max_insert_data_byte_size,
            max_insert_data_triple_count: value.max_insert_data_triple_count,
        }
    }
}
