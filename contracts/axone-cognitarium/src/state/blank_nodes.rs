use cw_storage_plus::Item;

/// A counter serving as blank node unique identifier generator.
pub const BLANK_NODE_IDENTIFIER_COUNTER: Item<u128> = Item::new("blank_node_key");
