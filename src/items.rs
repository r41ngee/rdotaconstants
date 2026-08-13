use std::collections::HashMap;
use serde_json::Value;

use crate::{Entity, errors, locals};

pub(crate) static ITEMS_JSON: &str = include_str!("data/items.json");

/// Struct that represents an Item object
#[derive(Debug, Clone)]
pub struct Item {
    /// Item's slugname
    pub name: String,
    /// Item's additional data
    pub data: serde_json::Map<String, Value>,
}

impl Item {
    // -----------------------------------------------------------------------------------------
    // METHODS
    // -----------------------------------------------------------------------------------------

    /// Returns an a object hidden by
    /// given key.
    /// # Example
    /// ```
    /// use rdotaconstants::Item;
    /// use serde_json::Value;
    /// let item = Item::get("item_abyssal_blade").unwrap();
    /// assert_eq!(item.resolve_value("AbilityBehavior").unwrap(), Value::String("DOTA_ABILITY_BEHAVIOR_UNIT_TARGET".to_string()))
    /// ```
    pub fn resolve_value<S: AsRef<str>>(&self, key: S) -> Result<Value, errors::ResolveValueError> {
        self.data.get(key.as_ref()).ok_or(errors::ResolveValueError::KeyNotFound(key.as_ref().to_string())).cloned()
    }

    /// Method that returns item's display name.
    pub fn display_name(&self) -> Result<String, errors::ResolveValueError> {
        let key = format!("DOTA_Tooltip_Ability_{}", self.name);
        locals()
            .get(&key)
            .cloned()
            .ok_or(errors::ResolveValueError::KeyNotFound(key))
    }

    /// Function that used to get a item's price,
    /// if it has one.
    /// 
    /// # Example
    /// ```
    /// use rdotaconstants::Item;
    /// let item = Item::get("item_blink").unwrap();
    /// assert_eq!(item.get_cost().unwrap(), 2250);
    /// ```
    pub fn get_cost(&self) -> Option<i32> {
        let value = self.resolve_value("ItemCost").ok()?;
        if let Value::String(s) = value {
            s.parse().ok()
        } else {
            None
        }
    }

    // -----------------------------------------------------------------------------------------
    // FUNCTIONS
    // -----------------------------------------------------------------------------------------

    /// Function that used to get an [Item] object
    /// from its slugname.
    pub fn get<T: AsRef<str>>(name: T) -> Option<Item> {
        let map = parse_items();
        let key = name.as_ref();
        let raw = map.get(key)?;
        Some(Item {
            name: key.to_string(),
            data: raw.clone(),
        })
    }

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
                    if let Some(item) = Self::get(codename) {
                        return Some(item);
                    }
                }
            }
        }
        None
    }

    /// Returns [Vec] with all possible variants of [Item].
    pub fn all() -> Vec<Item> {
        let map = parse_items();
        map.iter()
            .map(|(k, v)| Item {
                name: k.clone(),
                data: v.clone(),
            })
            .collect()
    }
}

impl Entity for Item {
    fn name(&self) -> &str {
        &self.name
    }
}
impl crate::private::Sealed for Item {}

#[allow(clippy::expect_used)]
fn parse_items() -> &'static HashMap<String, serde_json::Map<String, Value>> {
    use std::sync::OnceLock;
    static ONCE: OnceLock<HashMap<String, serde_json::Map<String, Value>>> = OnceLock::new();
    ONCE.get_or_init(|| {
        serde_json::from_str(ITEMS_JSON).expect("failed to parse items.json")
    })
}
