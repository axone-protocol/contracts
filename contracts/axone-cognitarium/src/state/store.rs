use crate::msg;
use crate::msg::StoreResponse;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

pub const STORE: Item<Store> = Item::new("store");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Store {
    pub owner: Addr,
    pub limits: StoreLimits,
    pub stat: StoreStat,
}

impl Store {
    pub fn new(owner: Addr, limits: StoreLimits) -> Store {
        Store {
            owner,
            limits,
            stat: StoreStat::default(),
        }
    }
}

impl From<Store> for StoreResponse {
    fn from(value: Store) -> Self {
        Self {
            owner: value.owner.into(),
            limits: value.limits.into(),
            stat: value.stat.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct StoreLimits {
    pub max_triple_count: Uint128,
    pub max_byte_size: Uint128,
    pub max_triple_byte_size: Uint128,
    pub max_query_limit: u32,
    pub max_query_variable_count: u32,
    pub max_insert_data_byte_size: Uint128,
    pub max_insert_data_triple_count: Uint128,
}

impl From<msg::StoreLimitsInput> for StoreLimits {
    fn from(value: msg::StoreLimitsInput) -> Self {
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

impl From<StoreLimits> for msg::StoreLimits {
    fn from(value: StoreLimits) -> Self {
        msg::StoreLimits {
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

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub struct StoreStat {
    pub triple_count: Uint128,
    pub namespace_count: Uint128,
    pub byte_size: Uint128,
}

impl From<StoreStat> for msg::StoreStat {
    fn from(value: StoreStat) -> Self {
        Self {
            triple_count: value.triple_count,
            namespace_count: value.namespace_count,
            byte_size: value.byte_size,
        }
    }
}
