use serde_json::Value;

use crate::{Entity, locals};

pub(crate) static HEROES_JSON: &str = include_str!("data/heroes.json");

/// Struct that represents hero object and its data
#[derive(Debug, Clone)]
pub struct Hero {
    /// Hero's slugname
    name: String,
    /// Hero's ID
    id: i64,
    /// Hero's additional data
    data: serde_json::Map<String, Value>,
}

impl Hero {
    /// Method used to get hero display name.
    /// # Example
    /// ```
    /// use rdotaconstants::Hero;
    /// let hero = Hero::get("npc_dota_hero_abyssal_underlord").unwrap();
    /// assert_eq!(hero.display_name(), "Underlord");
    /// ```
    pub fn display_name(&self) -> String {
        let key = format!("{}:n", self.name);
        locals()
            .get(&key)
            .cloned()
            .unwrap_or_default()
    }

    pub fn id(&self) -> i64 {
        self.id
    }
}

impl Entity for Hero {
    fn name(&self) -> &str { &self.name }
    fn data(&self) -> &serde_json::Map<String, Value> {
        &self.data
    }
    fn get_self<S: AsRef<str>>(s: S) -> Option<Self> {
        let name = s.as_ref();
        let heroes = parse_heroes();
        if let Value::Object(o) = heroes.get(name)? {
            Some(Self {
                name: name.to_string(),
                data: o.clone(),
                id: if let Value::String(ids) = o.get("HeroID")? {
                    ids.parse().ok()?
                } else { return None; }
            })
        } else { None }
    }
    fn all() -> Vec<Self> {
        let mut result = Vec::new();
        let parsed = parse_heroes();
        for i in parsed.keys() {
            if let Some(h) = Self::get_self(i) {
                result.push(h);
            }
        }
        result
    }
}
impl crate::private::Sealed for Hero {}

#[allow(clippy::expect_used)]
fn parse_heroes() -> &'static serde_json::Map<String, Value> {
    use std::sync::OnceLock;
    static ONCE: OnceLock<serde_json::Map<String, Value>> = OnceLock::new();
    ONCE.get_or_init(|| {
        serde_json::from_str(HEROES_JSON).expect("failed to parse heroes.json")
    })
}
