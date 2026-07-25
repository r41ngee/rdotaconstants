use std::collections::HashMap;
use serde_json::Value;

use crate::locals;

pub(crate) static HEROES_JSON: &str = include_str!("data/heroes.json");

#[derive(Debug, Clone)]
pub struct Hero {
    pub name: String,
    pub id: i64,
    pub data: serde_json::Map<String, Value>,
}

impl Hero {
    pub fn display_name(&self) -> String {
        let key = format!("{}:n", self.name);
        locals()
            .get(&key)
            .cloned()
            .unwrap_or_default()
    }

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
