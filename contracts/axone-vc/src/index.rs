use cosmwasm_std::{
    from_json, storage_keys::namespace_with_key, Record, StdError, StdResult, Storage,
};
use cw_storage_plus::{Index, IndexPrefix, KeyDeserialize, Map, Prefixer, PrimaryKey};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

pub struct OneToManyIndex<'a, IK, T, PK> {
    index: for<'b> fn(&[u8], &'b T) -> &'b [IK],
    idx_namespace: &'a [u8],
    idx_map: Map<Vec<u8>, u32>,
    pk_namespace: &'a [u8],
    phantom: PhantomData<PK>,
}

impl<'a, IK, T, PK> OneToManyIndex<'a, IK, T, PK>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub const fn new(
        idx_fn: for<'b> fn(&[u8], &'b T) -> &'b [IK],
        pk_namespace: &'a str,
        idx_namespace: &'static str,
    ) -> Self {
        Self {
            index: idx_fn,
            idx_namespace: idx_namespace.as_bytes(),
            idx_map: Map::new(idx_namespace),
            pk_namespace: pk_namespace.as_bytes(),
            phantom: PhantomData,
        }
    }
}

impl<'a, IK, T, PK> Index<T> for OneToManyIndex<'a, IK, T, PK>
where
    T: Serialize + DeserializeOwned + Clone,
    IK: Clone + PrimaryKey<'a>,
{
    fn save(&self, store: &mut dyn Storage, pk: &[u8], data: &T) -> StdResult<()> {
        for idx in (self.index)(pk, data) {
            self.idx_map
                .save(store, idx.clone().joined_extra_key(pk), &(pk.len() as u32))?;
        }

        Ok(())
    }

    fn remove(&self, store: &mut dyn Storage, pk: &[u8], old_data: &T) -> StdResult<()> {
        for idx in (self.index)(pk, old_data) {
            self.idx_map.remove(store, idx.clone().joined_extra_key(pk));
        }

        Ok(())
    }
}

impl<'a, IK, T, PK> OneToManyIndex<'a, IK, T, PK>
where
    PK: PrimaryKey<'a> + KeyDeserialize,
    T: Serialize + DeserializeOwned + Clone,
    IK: PrimaryKey<'a> + Prefixer<'a>,
{
    pub fn prefix(&self, p: IK) -> IndexPrefix<PK, T, PK> {
        IndexPrefix::with_deserialization_functions(
            self.idx_namespace,
            &p.prefix(),
            self.pk_namespace,
            deserialize_one_to_many_kv::<PK, T>,
            deserialize_one_to_many_v,
        )
    }
}

fn deserialize_one_to_many_v<T: DeserializeOwned>(
    store: &dyn Storage,
    pk_namespace: &[u8],
    kv: Record,
) -> StdResult<Record<T>> {
    let (key, pk_len) = kv;
    let pk_len = from_json::<u32>(pk_len.as_slice())?;
    let offset = key.len() - pk_len as usize;
    let pk = &key[offset..];
    let full_key = namespace_with_key(&[pk_namespace], pk);
    let value = store
        .get(&full_key)
        .ok_or_else(|| StdError::generic_err("pk not found"))?;

    Ok((pk.to_vec(), from_json::<T>(&value)?))
}

fn deserialize_one_to_many_kv<K: KeyDeserialize, T: DeserializeOwned>(
    store: &dyn Storage,
    pk_namespace: &[u8],
    kv: Record,
) -> StdResult<(K::Output, T)> {
    let (key, pk_len) = kv;
    let pk_len = from_json::<u32>(pk_len.as_slice())?;
    let offset = key.len() - pk_len as usize;
    let pk = &key[offset..];
    let full_key = namespace_with_key(&[pk_namespace], pk);
    let value = store
        .get(&full_key)
        .ok_or_else(|| StdError::generic_err("pk not found"))?;

    Ok((K::from_slice(pk)?, from_json::<T>(&value)?))
}

#[cfg(test)]
mod tests {
    use super::OneToManyIndex;
    use cosmwasm_std::{testing::MockStorage, Order, StdResult};
    use cw_storage_plus::{Index, IndexList, IndexedMap};
    use serde::{Deserialize, Serialize};

    const ITEMS: IndexedMap<&str, Item, ItemIndexes<'static>> = IndexedMap::new(
        "one_to_many_items",
        ItemIndexes {
            tags: OneToManyIndex::new(item_tags, "one_to_many_items", "one_to_many_items__tags"),
        },
    );

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    struct Item {
        name: String,
        tags: Vec<String>,
    }

    struct ItemIndexes<'a> {
        tags: OneToManyIndex<'a, String, Item, String>,
    }

    impl IndexList<Item> for ItemIndexes<'_> {
        fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Item>> + '_> {
            Box::new(vec![&self.tags as &dyn Index<Item>].into_iter())
        }
    }

    fn item_tags<'a>(_pk: &[u8], item: &'a Item) -> &'a [String] {
        &item.tags
    }

    fn item(name: &str, tags: &[&str]) -> Item {
        Item {
            name: name.to_string(),
            tags: tags.iter().map(|tag| tag.to_string()).collect(),
        }
    }

    #[test]
    fn prefix_returns_items_for_each_indexed_value() {
        let mut storage = MockStorage::new();

        ITEMS
            .save(&mut storage, "alpha", &item("Alpha", &["red", "round"]))
            .expect("alpha should save");
        ITEMS
            .save(&mut storage, "beta", &item("Beta", &["red"]))
            .expect("beta should save");
        ITEMS
            .save(&mut storage, "gamma", &item("Gamma", &["blue"]))
            .expect("gamma should save");

        let red_items = ITEMS
            .idx
            .tags
            .prefix("red".to_string())
            .range(&storage, None, None, Order::Ascending)
            .collect::<StdResult<Vec<_>>>()
            .expect("red items should load");

        assert_eq!(
            red_items,
            vec![
                ("alpha".to_string(), item("Alpha", &["red", "round"])),
                ("beta".to_string(), item("Beta", &["red"])),
            ]
        );
    }

    #[test]
    fn remove_deletes_all_index_entries_for_the_primary_key() {
        let mut storage = MockStorage::new();
        let alpha = item("Alpha", &["red", "round"]);

        ITEMS
            .save(&mut storage, "alpha", &alpha)
            .expect("alpha should save");
        ITEMS
            .remove(&mut storage, "alpha")
            .expect("alpha should remove");

        for tag in ["red", "round"] {
            let items = ITEMS
                .idx
                .tags
                .prefix(tag.to_string())
                .range(&storage, None, None, Order::Ascending)
                .collect::<StdResult<Vec<_>>>()
                .expect("items should load");
            assert!(items.is_empty(), "{tag} index should be empty");
        }
    }

    #[test]
    fn replace_updates_index_entries() {
        let mut storage = MockStorage::new();

        ITEMS
            .save(&mut storage, "alpha", &item("Alpha", &["red"]))
            .expect("alpha should save");
        ITEMS
            .save(&mut storage, "alpha", &item("Alpha", &["blue"]))
            .expect("alpha should update");

        let red_items = ITEMS
            .idx
            .tags
            .prefix("red".to_string())
            .range(&storage, None, None, Order::Ascending)
            .collect::<StdResult<Vec<_>>>()
            .expect("red items should load");
        let blue_items = ITEMS
            .idx
            .tags
            .prefix("blue".to_string())
            .range(&storage, None, None, Order::Ascending)
            .collect::<StdResult<Vec<_>>>()
            .expect("blue items should load");

        assert!(red_items.is_empty());
        assert_eq!(
            blue_items,
            vec![("alpha".to_string(), item("Alpha", &["blue"]))]
        );
    }

    #[test]
    fn empty_index_values_create_no_entries() {
        let mut storage = MockStorage::new();

        ITEMS
            .save(&mut storage, "alpha", &item("Alpha", &[]))
            .expect("alpha should save");

        let indexed_items = ITEMS
            .idx
            .tags
            .prefix("red".to_string())
            .range(&storage, None, None, Order::Ascending)
            .collect::<StdResult<Vec<_>>>()
            .expect("items should load");

        assert!(indexed_items.is_empty());
    }

    #[test]
    fn duplicate_index_values_create_one_entry_per_primary_key() {
        let mut storage = MockStorage::new();

        ITEMS
            .save(&mut storage, "alpha", &item("Alpha", &["red", "red"]))
            .expect("alpha should save");

        let indexed_items = ITEMS
            .idx
            .tags
            .prefix("red".to_string())
            .range(&storage, None, None, Order::Ascending)
            .collect::<StdResult<Vec<_>>>()
            .expect("items should load");

        assert_eq!(
            indexed_items,
            vec![("alpha".to_string(), item("Alpha", &["red", "red"]))]
        );
    }

    #[test]
    fn prefix_respects_bounds_and_order() {
        let mut storage = MockStorage::new();

        for (pk, value) in [
            ("alpha", item("Alpha", &["red"])),
            ("beta", item("Beta", &["red"])),
            ("gamma", item("Gamma", &["red"])),
        ] {
            ITEMS
                .save(&mut storage, pk, &value)
                .expect("item should save");
        }

        let indexed_items = ITEMS
            .idx
            .tags
            .prefix("red".to_string())
            .range(
                &storage,
                Some(cw_storage_plus::Bound::exclusive("alpha".to_string())),
                None,
                Order::Descending,
            )
            .collect::<StdResult<Vec<_>>>()
            .expect("items should load");

        assert_eq!(
            indexed_items,
            vec![
                ("gamma".to_string(), item("Gamma", &["red"])),
                ("beta".to_string(), item("Beta", &["red"])),
            ]
        );
    }

    #[test]
    fn range_raw_returns_primary_keys_and_items() {
        let mut storage = MockStorage::new();

        ITEMS
            .save(&mut storage, "alpha", &item("Alpha", &["red"]))
            .expect("alpha should save");

        let indexed_items = ITEMS
            .idx
            .tags
            .prefix("red".to_string())
            .range_raw(&storage, None, None, Order::Ascending)
            .collect::<StdResult<Vec<_>>>()
            .expect("items should load");

        assert_eq!(
            indexed_items,
            vec![(b"alpha".to_vec(), item("Alpha", &["red"]))]
        );
    }

    #[test]
    fn range_reports_dangling_primary_key() {
        let mut storage = MockStorage::new();

        ITEMS
            .save(&mut storage, "alpha", &item("Alpha", &["red"]))
            .expect("alpha should save");
        ITEMS.key("alpha").remove(&mut storage);

        let err = ITEMS
            .idx
            .tags
            .prefix("red".to_string())
            .range(&storage, None, None, Order::Ascending)
            .next()
            .expect("index entry should remain")
            .expect_err("missing primary key should fail");

        assert_eq!(err.to_string(), "Generic error: pk not found");
    }
}
