use cw_storage_plus::Item;

pub const DEFAULT_FOO_VALUE: &str = "foo";
pub const FOO: Item<String> = Item::new("foo");
