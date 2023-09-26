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
/// namespace's value or namespace's internal state key. It implements a two way indexed in-memory
/// cache to mitigate state access.
pub struct NamespaceResolver {
    by_val: BTreeMap<String, Rc<RefCell<Namespace>>>,
    by_key: BTreeMap<u128, Rc<RefCell<Namespace>>>,
}

impl NamespaceResolver {
    pub fn new() -> Self {
        Self {
            by_key: BTreeMap::new(),
            by_val: BTreeMap::new(),
        }
    }

    /// Resolve a [Namespace] from its value, returning it from cache in priority before accessing
    /// the state.
    pub fn resolve_from_val(
        &mut self,
        storage: &dyn Storage,
        value: String,
    ) -> StdResult<Option<Namespace>> {
        self.resolve_cell_from_val(storage, value)
            .map(|maybe_cell| maybe_cell.map(|cell| cell.borrow().clone()))
    }

    /// Resolve a [Namespace] from its internal key, returning it from cache in priority before accessing
    /// the state.
    pub fn resolve_from_key(
        &mut self,
        storage: &dyn Storage,
        key: u128,
    ) -> StdResult<Option<Namespace>> {
        self.resolve_cell_from_key(storage, key)
            .map(|maybe_cell| maybe_cell.map(|cell| cell.borrow().clone()))
    }

    /// Resolve a counting reference to a memory location of a cached [Namespace] from its value,
    /// returning it from cache in priority before accessing the state. It allows to mutate it in place.
    fn resolve_cell_from_val(
        &mut self,
        storage: &dyn Storage,
        value: String,
    ) -> StdResult<Option<Rc<RefCell<Namespace>>>> {
        if let Some(rc) = self.by_val.get(value.as_str()) {
            return Ok(Some(rc.clone()));
        }

        namespaces()
            .may_load(storage, value)
            .map(|maybe_ns| maybe_ns.map(|ns| self.insert(ns)))
    }

    /// Resolve a counting reference to a memory location of a cached [Namespace] from its key,
    /// returning it from cache in priority before accessing the state. It allows to mutate it in place.
    fn resolve_cell_from_key(
        &mut self,
        storage: &dyn Storage,
        key: u128,
    ) -> StdResult<Option<Rc<RefCell<Namespace>>>> {
        if let Some(rc) = self.by_key.get(&key) {
            return Ok(Some(rc.clone()));
        }

        namespaces()
            .idx
            .key
            .item(storage, key)
            .map(|maybe_ns| maybe_ns.map(|ns| self.insert(ns.1)))
    }

    /// Cache a namespace by creating a dedicated mutable memory location shared between indexes
    /// returning a counted reference to it.
    fn insert(&mut self, ns: Namespace) -> Rc<RefCell<Namespace>> {
        let ns_rc = Rc::new(RefCell::new(ns.clone()));

        self.by_val.insert(ns.value, ns_rc.clone());
        self.by_key.insert(ns.key, ns_rc.clone());

        ns_rc
    }

    /// Utility middleware to consider `StdResult::Ok(None)` as `Err(StdError::NotFound)` of namespace.
    /// Typically used with [Self::resolve_from_key].
    pub fn none_as_error_middleware(resolve_res: Option<Namespace>) -> StdResult<Namespace> {
        match resolve_res {
            Some(ns) => Ok(ns),
            None => Err(StdError::not_found("Namespace")),
        }
    }
}

impl Default for NamespaceResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Used when managing an internal [Namespace] cache to expose it, the purpose is to allow the cache
/// to be reusable.
pub trait HasCachedNamespaces {
    /// Return the cached namespaces.
    fn cached_namespaces(&self) -> Vec<Namespace>;

    /// Empty the namespace cache.
    fn clear_cache(&mut self);
}

impl HasCachedNamespaces for NamespaceResolver {
    fn cached_namespaces(&self) -> Vec<Namespace> {
        self.by_key
            .iter()
            .map(|cell| cell.1.borrow().clone())
            .collect()
    }

    fn clear_cache(&mut self) {
        self.by_val.clear();
        self.by_key.clear();
    }
}

impl From<Vec<Namespace>> for NamespaceResolver {
    fn from(value: Vec<Namespace>) -> Self {
        let mut resolver = NamespaceResolver::new();
        for ns in value {
            resolver.insert(ns);
        }

        resolver
    }
}

/// Allow to batch write operations on [Namespace] taking care of the [NAMESPACE_KEY_INCREMENT], it
/// manages insertions/deletions as well as counting references. It internally use a [NamespaceResolver]
/// as a cache of new/removed/modified namespaces, to finally apply writing to the state when
/// calling [Self::flush].
pub struct NamespaceBatchService {
    ns_resolver: NamespaceResolver,
    ns_key_inc: u128,
    ns_count_diff: i128,
}

impl NamespaceBatchService {
    pub fn new(storage: &dyn Storage) -> StdResult<Self> {
        Ok(Self {
            ns_resolver: NamespaceResolver::new(),
            ns_key_inc: NAMESPACE_KEY_INCREMENT.load(storage)?,
            ns_count_diff: 0,
        })
    }

    /// Resolve a namespace by its value, or allocate a new one if not existing. This is applied to
    /// the in-memory cache only, [Self::flush] must be called to write the changes to the state.
    pub fn resolve_or_allocate(
        &mut self,
        storage: &dyn Storage,
        value: String,
    ) -> StdResult<Namespace> {
        self.ns_resolver
            .resolve_cell_from_val(storage, value.clone())
            .map(|maybe_cell| {
                maybe_cell.map_or_else(|| self.allocate(value), |cell| cell.borrow().clone())
            })
    }

    /// Increment the count of references to this namespace. This is applied to the in-memory cache
    /// only, [Self::flush] must be called to write the changes to the state.
    pub fn count_ref(&mut self, storage: &dyn Storage, key: u128) -> StdResult<Namespace> {
        self.ns_resolver
            .resolve_cell_from_key(storage, key)
            .and_then(|maybe_cell| {
                maybe_cell.map_or_else(
                    || Err(StdError::not_found("Namespace")),
                    |cell| {
                        let mut ns = cell.borrow_mut();
                        ns.counter += 1;
                        Ok(ns.clone())
                    },
                )
            })
    }

    /// Decrement the count of references to this namespace, deleting it if not used anymore.
    /// This is applied to the in-memory cache only, [Self::flush] must be called to write the changes
    /// to the state.
    pub fn free_ref(&mut self, storage: &dyn Storage, key: u128) -> StdResult<Namespace> {
        self.ns_resolver
            .resolve_cell_from_key(storage, key)
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

    /// Same as [Self::free_ref] but identifying a namespace by value.
    pub fn free_ref_by_val(
        &mut self,
        storage: &dyn Storage,
        value: String,
    ) -> StdResult<Namespace> {
        self.ns_resolver
            .resolve_cell_from_val(storage, value)
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

    /// Writes all the cached changes to the state, returning the namespace count diff.
    pub fn flush(&mut self, storage: &mut dyn Storage) -> StdResult<i128> {
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

        let count_diff = self.ns_count_diff;
        self.ns_count_diff = 0;
        self.ns_resolver.clear_cache();

        Ok(count_diff)
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
