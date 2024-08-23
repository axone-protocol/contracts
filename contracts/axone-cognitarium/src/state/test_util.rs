use crate::state::{Namespace, NamespaceSolver};
use cosmwasm_std::{StdError, StdResult};
use std::collections::BTreeMap;

pub struct InMemoryNamespaceSolver {
    by_val: BTreeMap<String, Namespace>,
    by_key: BTreeMap<u128, Namespace>,
}

impl InMemoryNamespaceSolver {
    pub fn with(namespaces: Vec<(u128, &str)>) -> Self {
        let mut by_val = BTreeMap::new();
        let mut by_key = BTreeMap::new();
        for (key, value) in namespaces {
            let ns = Namespace {
                value: value.to_string(),
                key,
                counter: 1,
            };
            by_val.insert(value.to_string(), ns.clone());
            by_key.insert(key, ns);
        }
        Self { by_val, by_key }
    }
}

impl NamespaceSolver for InMemoryNamespaceSolver {
    fn resolve_from_key(&mut self, key: u128) -> StdResult<Namespace> {
        self.by_key
            .get(&key)
            .ok_or_else(|| StdError::not_found("Namespace"))
            .cloned()
    }

    fn resolve_from_val(&mut self, _value: String) -> StdResult<Namespace> {
        self.by_val
            .get(&_value)
            .ok_or_else(|| StdError::not_found("Namespace"))
            .cloned()
    }
}
