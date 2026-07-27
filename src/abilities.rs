use serde_json::Value;

use crate::locals;

pub(crate) static ABILITIES_JSON: &str = include_str!("data/abilities.json");

#[derive(Debug, Clone)]
pub struct Ability {
    pub name: String,
    pub data: serde_json::Map<String, Value>,
}

impl Ability {
    /// Method used to get ability display name.
    /// # Example
    /// ```
    /// use rdotaconstants::Ability;
    /// let ability = Ability::get("lion_impale").unwrap();
    /// assert_eq!(ability.display_name(), "Earth Spike");
    /// ```
    pub fn display_name(&self) -> String {
        let key = format!("DOTA_Tooltip_ability_{}", self.name);
        locals()
            .get(&key)
            .cloned()
            .unwrap_or_default()
    }

    /// Method used to get ability UI description.
    pub fn display_description(&self) -> String {
        let key = format!("DOTA_Tooltip_ability_{}_Description", self.name);
        locals()
            .get(&key)
            .cloned()
            .unwrap_or_default()
    }

    /// Method used to get an [Ability] object by its slugname.
    /// Returns [Option]<[Ability]>
    pub fn get<T: AsRef<str>>(name: T) -> Option<Ability> {
        let map = parse_abilities();
        let key = name.as_ref();
        let val = map.get(key)?;
        let data = val.as_object()?.clone();
        Some(Ability {
            name: key.to_string(),
            data,
        })
    }

    #[cfg(feature = "unstable")]
    /// # Use `unstable` feature to use this function
    /// This function us marked as unstable because
    /// of undefined result for abilities with
    /// same name.
    /// 
    /// For example, Lion's **Hex** and Shadow Shaman's **Hex** will return 
    /// undefined result.
    pub fn get_by_display_name(display_name: &str) -> Option<Ability> {
        let locs = locals();
        let prefix = "DOTA_Tooltip_ability_";
        for (key, value) in locs.iter() {
            if value == display_name {
                if let Some(codename) = key.strip_prefix(prefix) {
                    if !codename.ends_with("_Description") {
                        if let Some(ability) = Self::get(codename) {
                            return Some(ability);
                        }
                    }
                }
            }
        }
        None
    }

    /// Returns [Vec] containing all possible variants of [Ability].
    pub fn all() -> Vec<Ability> {
        let map = parse_abilities();
        map.iter()
            .filter_map(|(k, v)| {
                let data = v.as_object()?.clone();
                Some(Ability {
                    name: k.clone(),
                    data,
                })
            })
            .collect()
    }
}

fn parse_abilities() -> &'static serde_json::Map<String, Value> {
    use std::sync::OnceLock;
    static ONCE: OnceLock<serde_json::Map<String, Value>> = OnceLock::new();
    ONCE.get_or_init(|| {
        let raw: serde_json::Map<String, Value> =
            serde_json::from_str(ABILITIES_JSON).expect("failed to parse abilities.json");
        let mut filtered = serde_json::Map::new();
        for (k, v) in raw {
            if v.is_object() {
                filtered.insert(k, v);
            }
        }
        filtered
    })
}
