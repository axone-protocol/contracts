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

    pub fn resolve_from_key(&mut self, key: u128) -> StdResult<Option<Namespace>> {
        self.resolve_cell_from_key(key)
            .map(|maybe_cell| maybe_cell.map(|cell| cell.borrow().clone()))
    }

    pub fn clear(&mut self) -> () {
        self.by_val.clear();
        self.by_key.clear();
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

    fn resolve_cell_from_key(&mut self, key: u128) -> StdResult<Option<Rc<RefCell<Namespace>>>> {
        if let Some(rc) = self.by_key.get(&key) {
            return Ok(Some(rc.clone()));
        }

        namespaces()
            .idx
            .key
            .item(self.storage, key)
            .map(|maybe_ns| maybe_ns.map(|ns| self.insert(ns.1)))
    }

    fn insert(&mut self, ns: Namespace) -> Rc<RefCell<Namespace>> {
        let ns_rc = Rc::new(RefCell::new(ns.clone()));

        self.by_val.insert(ns.value.clone(), ns_rc.clone());
        self.by_key.insert(ns.key.clone(), ns_rc.clone());

        ns_rc
    }
}

pub struct NamespaceBatchService<'a> {
    ns_resolver: NamespaceResolver<'a>,
    ns_key_inc: u128,
    ns_count_diff: i128,
}

impl<'a> NamespaceBatchService<'a> {
    pub fn new(storage: &'a dyn Storage) -> StdResult<Self> {
        Ok(Self {
            ns_resolver: NamespaceResolver::new(storage),
            ns_key_inc: NAMESPACE_KEY_INCREMENT.load(storage)?,
            ns_count_diff: 0,
        })
    }

    pub fn count_ref(&mut self, value: String) -> StdResult<Namespace> {
        self.ns_resolver
            .resolve_cell_from_val(value.clone())
            .map(|maybe_cell| {
                maybe_cell
                    .map(|cell| {
                        let mut ns = cell.borrow_mut();
                        ns.counter += 1;
                        ns.clone()
                    })
                    .unwrap_or_else(|| self.allocate(value))
            })
    }

    pub fn free_ref(&mut self, value: String) -> StdResult<Namespace> {
        self.ns_resolver
            .resolve_cell_from_val(value.clone())
            .and_then(|maybe_cell| {
                let cell = match maybe_cell.filter(|c| c.borrow().counter > 0) {
                    Some(c) => c,
                    None => Err(StdError::generic_err(
                        "Trying to delete a non existing namespace",
                    ))?,
                };

                let mut ns = cell.borrow_mut();
                ns.counter -= 1;
                if ns.counter == 0 {
                    self.ns_count_diff -= 1;
                }

                let tmp = ns.clone();
                Ok(tmp)
            })
    }

    pub fn flush(&mut self, storage: &'a mut dyn Storage) -> StdResult<i128> {
        NAMESPACE_KEY_INCREMENT.save(storage, &self.ns_key_inc)?;

        for entry in &self.ns_resolver.by_val {
            if entry.1.borrow().counter > 0 {
                namespaces().save(storage, entry.0.to_string(), &entry.1.borrow().clone())?;
            } else {
                let res = namespaces().remove(storage, entry.0.to_string());
                match res {
                    Err(StdError::NotFound { .. }) => Ok(()),
                    _ => res,
                }?;
            }
        }

        self.Ok(self.ns_count_diff)
    }

    fn allocate(&mut self, value: String) -> Namespace {
        let ns = Namespace {
            value,
            key: self.ns_key_inc,
            counter: 0u128,
        };

        self.ns_key_inc += 1;
        self.ns_count_diff += 1;

        self.ns_resolver.insert(ns).borrow().clone()
    }
}
