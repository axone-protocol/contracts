use cw_storage_plus::{Index, IndexList, IndexedMap, Item, UniqueIndex};
use serde::{Deserialize, Serialize};

pub const NAMESPACE_KEY_INCREMENT: Item<u128> = Item::new("namespace_key");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Namespace {
    pub key: u128,
    pub counter: u128,
}

pub struct NamespaceIndexes<'a> {
    key: UniqueIndex<'a, u128, Namespace, String>,
}

impl IndexList<Namespace> for NamespaceIndexes<'_> {
    fn get_indexes(&self) -> Box<dyn Iterator<Item = &'_ dyn Index<Namespace>> + '_> {
        Box::new(vec![&self.key as &dyn Index<Namespace>].into_iter())
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
