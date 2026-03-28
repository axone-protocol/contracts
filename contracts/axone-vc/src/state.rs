use crate::domain::Authority;

use cw_storage_plus::Item;

pub const DEFAULT_FOO_VALUE: &str = "foo";
pub const AUTHORITY: Item<Authority> = Item::new("authority");
pub const FOO: Item<String> = Item::new("foo");
