use std::collections::HashMap;
use serde_json::Value;

use crate::locals;

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
    /// Method that returns item's display name.
    pub fn display_name(&self) -> String {
        let key = format!("DOTA_Tooltip_Ability_{}", self.name);
        locals()
            .get(&key)
            .cloned()
            .unwrap_or_default()
    }

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

fn parse_items() -> &'static HashMap<String, serde_json::Map<String, Value>> {
    use std::sync::OnceLock;
    static ONCE: OnceLock<HashMap<String, serde_json::Map<String, Value>>> = OnceLock::new();
    ONCE.get_or_init(|| {
        serde_json::from_str(ITEMS_JSON).expect("failed to parse items.json")
    })
}
