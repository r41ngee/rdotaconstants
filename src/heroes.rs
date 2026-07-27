use std::collections::HashMap;
use serde_json::Value;

use crate::locals;

pub(crate) static HEROES_JSON: &str = include_str!("data/heroes.json");

/// Struct that represents hero object and its data
#[derive(Debug, Clone)]
pub struct Hero {
    /// Hero's slugname
    pub name: String,
    /// Hero's ID
    pub id: i64,
    /// Hero's additional data
    pub data: serde_json::Map<String, Value>,
}

impl Hero {
    /// Method used to get ability display name.
    /// # Example
    /// ```
    /// use rdotaconstants::Hero;
    /// let ability = Hero::get("npc_dota_hero_abyssal_underlord").unwrap();
    /// assert_eq!(ability.display_name(), "Underlord");
    /// ```
    pub fn display_name(&self) -> String {
        let key = format!("{}:n", self.name);
        locals()
            .get(&key)
            .cloned()
            .unwrap_or_default()
    }

    /// Function used to get a [Hero] object by hero's slugname.
    /// 
    /// ⚠️ Slugname should contain `npc_dota_hero_` part.
    pub fn get<T: AsRef<str>>(name: T) -> Option<Hero> {
        let map = parse_heroes();
        let key = name.as_ref();
        let raw = map.get(key)?;
        let id = raw.get("HeroID")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(-1);
        Some(Hero {
            name: key.to_string(),
            id,
            data: raw.clone(),
        })
    }

    /// Function used to get a [Hero] object by hero's
    /// ID, mentioned in `data/heroes.json`.
    /// # Example
    /// ```
    /// // heroes.json:
    /// // {
    /// //     "npc_dota_hero_antimage": {
    /// //         ...
    /// //         "HeroID": "1",
    /// //         ...
    /// //     }
    /// //     ...
    /// // }
    /// 
    /// use rdotaconstants::Hero;
    /// let hero = Hero::get_by_id(1).unwrap();
    /// assert_eq!(hero.display_name(), "Anti-Mage");
    /// ```
    pub fn get_by_id(id: i64) -> Option<Hero> {
        let map = parse_heroes();
        map.iter()
            .find(|(_, v)| {
                v.get("HeroID")
                    .and_then(|val| val.as_str())
                    .and_then(|s| s.parse::<i64>().ok())
                    == Some(id)
            })
            .map(|(k, v)| Hero {
                name: k.clone(),
                id,
                data: v.clone(),
            })
    }

    /// Function used to get a [Hero] object by its
    /// display name.
    pub fn get_by_display_name(display_name: &str) -> Option<Hero> {
        let map = parse_heroes();
        let locs = locals();
        for (name, _) in map.iter() {
            let key = format!("{}:n", name);
            if locs.get(&key).map(|s| s.as_str()) == Some(display_name) {
                return Self::get(name);
            }
        }
        None
    }

    /// Function returns all variants of [Hero].
    pub fn all() -> Vec<Hero> {
        let map = parse_heroes();
        map.iter()
            .map(|(k, v)| {
                let id = v.get("HeroID")
                    .and_then(|val| val.as_str())
                    .and_then(|s| s.parse::<i64>().ok())
                    .unwrap_or(-1);
                Hero {
                    name: k.clone(),
                    id,
                    data: v.clone(),
                }
            })
            .collect()
    }
}

fn parse_heroes() -> &'static HashMap<String, serde_json::Map<String, Value>> {
    use std::sync::OnceLock;
    static ONCE: OnceLock<HashMap<String, serde_json::Map<String, Value>>> = OnceLock::new();
    ONCE.get_or_init(|| {
        serde_json::from_str(HEROES_JSON).expect("failed to parse heroes.json")
    })
}
