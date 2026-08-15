use serde_json::Value;

use crate::Entity;
#[cfg(feature = "unstable")]
use crate::locals;

pub(crate) static ITEMS_JSON: &str = include_str!("data/items.json");

/// Struct that represents an Item object
#[derive(Debug, Clone)]
pub struct Item {
    /// Item's slugname
    name: String,
    /// Item's additional data
    data: serde_json::Map<String, Value>,
}

impl Item {
    // -----------------------------------------------------------------------------------------
    // METHODS
    // -----------------------------------------------------------------------------------------

    /// Function that used to get a item's price,
    /// if it has one.
    /// 
    /// # Example
    /// ```
    /// use rdotaconstants::{Item, Entity};
    /// let item = Item::get_self("item_blink").unwrap();
    /// assert_eq!(item.get_cost().unwrap(), 2250);
    /// ```
    pub fn get_cost(&self) -> Option<i32> {
        let value = self.get("ItemCost")?;
        if let Value::String(s) = value {
            s.parse().ok()
        } else {
            None
        }
    }

    // -----------------------------------------------------------------------------------------
    // FUNCTIONS
    // -----------------------------------------------------------------------------------------

    /// This function is marked as unstable because
    /// of undefined result for abilities with
    /// same name.
    /// 
    /// There is no known examples of
    /// this behavior, but this function
    /// is still unsafe for future.
    #[cfg(feature = "unstable")]
    pub fn get_by_display_name(display_name: &str) -> Option<Item> {
        let locs = locals();
        let prefix = "DOTA_Tooltip_Ability_";
        for (key, value) in locs.iter() {
            if value == display_name {
                if let Some(codename) = key.strip_prefix(prefix) {
                    if let Some(item) = Self::get_self(codename) {
                        return Some(item);
                    }
                }
            }
        }
        None
    }
}

impl Entity for Item {
    fn name(&self) -> &str {
        &self.name
    }

    fn data(&self) -> &serde_json::Map<String, Value> {
        &self.data
    }

    fn get_self<S: AsRef<str>>(name: S) -> Option<Self> {
        let items = parse_items();
        let raw = items.get_key_value(name.as_ref())?;
        if let Value::Object(o) = raw.1 {
                Some(Self { name: raw.0.clone(), data: o.clone() })
        } else { None }
    }

    fn all() -> Vec<Self> {
        let mut result = Vec::new();
        let items = parse_items();
        for i in items.keys() {
            if let Some(item) = Self::get_self(i) {
                result.push(item);
            }
        }

        result
    }
}
impl crate::private::Sealed for Item {}

#[allow(clippy::expect_used)]
fn parse_items() -> &'static serde_json::Map<String, Value> {
    use std::sync::OnceLock;
    static ONCE: OnceLock<serde_json::Map<String, Value>> = OnceLock::new();
    ONCE.get_or_init(|| {
        let raw: serde_json::Map<String, Value> =
            serde_json::from_str(ITEMS_JSON).expect("failed to parse abilities.json");
        let mut filtered = serde_json::Map::new();
        for (k, v) in raw {
            if v.is_object() {
                filtered.insert(k, v);
            }
        }
        filtered
    })
}
