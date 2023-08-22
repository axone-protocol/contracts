use cw_storage_plus::{Index, IndexList, IndexedMap, Item, UniqueIndex};
use serde::{Deserialize, Serialize};

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
