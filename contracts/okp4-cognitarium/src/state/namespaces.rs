use cosmwasm_std::{StdError, StdResult, Storage};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, UniqueIndex};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

/// Store a key increment used a unique key for referencing a namespace. Given the size of an `u128`
/// there is no need to implement a garbage collector mechanism in case some namespaces are removed.
pub const NAMESPACE_KEY_INCREMENT: Item<'_, u128> = Item::new("namespace_key");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Namespace {
    /// The namespace value.
    pub value: String,

    /// The unique, incremented key issues to reference this namespace from a triple IRI.
    pub key: u128,

    /// A reference counter to this namespace.
    pub counter: u128,
}

pub struct NamespaceIndexes<'a> {
    pub key: UniqueIndex<'a, u128, Namespace, String>,
}

impl IndexList<Namespace> for NamespaceIndexes<'_> {
    fn get_indexes(&self) -> Box<dyn Iterator<Item = &'_ dyn Index<Namespace>> + '_> {
        let key: &dyn Index<Namespace> = &self.key;
        Box::new(vec![key].into_iter())
    }
}

pub fn namespaces<'a>() -> IndexedMap<'a, String, Namespace, NamespaceIndexes<'a>> {
    IndexedMap::new(
        "NAMESPACE",
        NamespaceIndexes {
            key: UniqueIndex::new(|ns| ns.key, "NAMESPACE__KEY"),
        },
    )
}

/// [NamespaceResolver] is a [Namespace] querying service allowing to resolve namespaces either by
/// namespace's value or namespace's internal state key. It implements a two way indexed in memory
/// cache to mitigate state access.
pub struct NamespaceResolver<'a> {
    storage: &'a dyn Storage,
    by_val: BTreeMap<String, Rc<RefCell<Namespace>>>,
    by_key: BTreeMap<u128, Rc<RefCell<Namespace>>>,
}

impl<'a> NamespaceResolver<'a> {
    pub fn new(storage: &'a dyn Storage) -> Self {
        Self {
            storage,
            by_key: BTreeMap::new(),
            by_val: BTreeMap::new(),
        }
    }

    pub fn resolve_from_val(&mut self, value: String) -> StdResult<Option<Namespace>> {
        self.resolve_cell_from_val(value)
            .map(|maybe_cell| maybe_cell.map(|cell| cell.borrow().clone()))
    }

    fn resolve_cell_from_val(
        &mut self,
        value: String,
    ) -> StdResult<Option<Rc<RefCell<Namespace>>>> {
        if let Some(rc) = self.by_val.get(value.as_str()) {
            return Ok(Some(rc.clone()));
        }

        namespaces()
            .may_load(self.storage, value)
            .map(|maybe_ns| maybe_ns.map(|ns| self.insert(ns)))
    }

    fn insert(&mut self, ns: Namespace) -> Rc<RefCell<Namespace>> {
        let ns_rc = Rc::new(RefCell::new(ns.clone()));

        self.by_val.insert(ns.value.clone(), ns_rc.clone());
        self.by_key.insert(ns.key.clone(), ns_rc.clone());

        ns_rc
    }
}
