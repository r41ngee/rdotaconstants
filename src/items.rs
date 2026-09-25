use std::sync::LazyLock;

use crate::Entity;
use crate::LOCALS;
use crate::map::{Map, Value};

use bincode_next as bincode;

static ITEMS_BIN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/.bin/items.bin"));

/// Struct that represents an Item object
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Item {
    /// Item's slugname
    name: String,
    /// Item's additional data
    data: Map,
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
    /// # use rdotaconstants::{Item, Entity};
    /// let item = Item::new("item_blink").unwrap();
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

    /// Returns display name of item.
    pub fn display_name(&self) -> Option<&str> {
        LOCALS.get(&format!("DOTA_Tooltip_ability_{}", self.name()))
            .or_else(|| LOCALS.get(&format!("DOTA_Tooltip_Ability_{}", self.name())))
            .map(|x| x.as_str())
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
    /// 
    /// ```
    /// # use rdotaconstants::{Entity, Item};
    /// let item = Item::get_by_display_name("Aeon Disk");
    /// assert_eq!(item.name(), "item_aeon_disk")
    /// ```
    #[cfg(feature = "unstable")]
    pub fn get_by_display_name(display_name: &str) -> Option<Item> {
        let prefix = "DOTA_Tooltip_Ability_";
        for (key, value) in LOCALS.as_ref().iter() {
            if value == display_name {
                if let Some(codename) = key.strip_prefix(prefix) {
                    if let Some(item) = Self::new(codename) {
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

    fn data(&self) -> &Map {
        &self.data
    }

    fn new<S: AsRef<str>>(name: S) -> Option<Self> {
        let items = parse_items();
        let (name, data) = items.get_key_value(name.as_ref())?;
        if let Value::Map(m) = data {
            Some(Self { name: name.to_string(), data: m.clone() })
        } else { None }
    }

    fn all() -> Vec<Self> {
        let mut result = Vec::new();
        let items = parse_items();
        for i in items.keys() {
            if let Some(item) = Self::new(i) {
                result.push(item);
            }
        }

        result
    }
}
impl crate::private::Sealed for Item {}

#[allow(clippy::expect_used)]
fn parse_items() -> &'static Map {
    static ONCE: LazyLock<Map> = LazyLock::new(|| {
        bincode::decode_from_slice(ITEMS_BIN, bincode::config::standard())
            .expect("failed to parse abilities.bin")
            .0
    });

    &ONCE
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn get_self() {
        Item::new("item_aeon_disk").unwrap();
    }

    #[test]
    #[should_panic]
    fn get_self_fake() {
        Item::new("item_anjdsdjknasnjska").unwrap();
    }

    #[test]
    fn name_getter() {
        let name = "item_aeon_disk";
        let item = Item::new(name).unwrap();
        assert_eq!(name, item.name())
    }

    #[test]
    fn data_getter() {
        assert!(!Item::new("item_aeon_disk").unwrap().data().is_empty());
    }

    #[test]
    fn get_cost() {
        let item = Item::new("item_aeon_disk").unwrap();
        assert_eq!(item.get_cost().unwrap(), 3000);
    }

    #[test]
    fn get_all() {
        assert!(!Item::all().is_empty())
    }

    #[test]
    fn get_display_name() {
        let item = Item::new("item_aeon_disk").unwrap();
        assert_eq!(item.display_name().unwrap(), "Aeon Disk")
    }

    #[cfg(feature = "unstable")]
    #[test]
    fn get_by_display_name() {
        let item = Item::get_by_display_name("Aeon Disk").unwrap();
        assert_eq!(item.name(), "item_aeon_disk")
    }
}
