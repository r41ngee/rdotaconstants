use std::sync::LazyLock;

use bincode_next as bincode;

use crate::map::{Map, Value};
use crate::{Entity, LOCALS};

static HEROES_BIN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/.bin/heroes.bin"));

/// Represents hero's data
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hero {
    /// Hero's slugname
    name: String,
    /// Hero's ID
    id: i64,
    /// Hero's additional data
    data: Map,
}

impl Hero {
    /// Get hero display name.
    /// # Example
    /// ```
    /// use rdotaconstants::{Hero, Entity};
    /// let hero = Hero::new("npc_dota_hero_abyssal_underlord").unwrap();
    /// assert_eq!(hero.display_name(), "Underlord");
    /// ```
    pub fn display_name(&self) -> String {
        let key = format!("{}:n", self.name);
        LOCALS
            .get(&key)
            .cloned()
            .unwrap_or_default()
    }

    /// Get hero id.
    /// # Example
    /// ```
    /// use rdotaconstants::{Hero, Entity};
    /// let hero = Hero::new("npc_dota_hero_antimage").unwrap();
    /// assert_eq!(hero.id(), 1);
    /// ```
    #[deprecated(
        since = "0.5.3",
        note = "This function will return `u16` in `0.6.0`. Use [`id_v2()`](`Self::id_v2()`) instead"
    )]
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Get hero id([`u16`]).
    /// # Example
    /// ```
    /// use rdotaconstants::{Hero, Entity};
    /// let hero = Hero::new("npc_dota_hero_antimage").unwrap();
    /// assert_eq!(hero.id(), 1);
    /// ```
    pub fn id_v2(&self) -> u16 {
        self.id as u16
    }

    /// Get hero object from its id.
    /// # Example
    /// ```
    /// use rdotaconstants::{Hero, Entity};
    /// let hero = Hero::from_id(1).unwrap();
    /// assert_eq!(hero.name(), "npc_dota_hero_antimage")
    /// ```
    #[deprecated(
        since = "0.5.3",
        note = "This function will take `u16` in `0.6.0`. Use [`from_id_v2()`](`Self::from_id_v2()`) instead"
    )]
    pub fn from_id(id: i64) -> Option<Self> {
        Self::all().into_iter().find(|x| x.id_v2() as i64 == id)
    }

    /// Get hero object from its id([`u16`]).
    /// # Example
    /// ```
    /// use rdotaconstants::{Hero, Entity};
    /// let hero = Hero::from_id(1).unwrap();
    /// assert_eq!(hero.name(), "npc_dota_hero_antimage")
    /// ```
    pub fn from_id_v2(id: u16) -> Option<Self> {
        Self::all().into_iter().find(|x| x.id_v2() == id)
    }
}

impl Entity for Hero {
    fn name(&self) -> &str { &self.name }
    fn data(&self) -> &Map {
        &self.data
    }
    fn new<S: AsRef<str>>(s: S) -> Option<Self> {
        let name = s.as_ref();
        let heroes = parse_heroes();
        if let Value::Map(o) = heroes.get(name)? {
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
            if let Some(h) = Self::new(i) {
                result.push(h);
            }
        }
        result
    }
}
impl crate::private::Sealed for Hero {}

#[allow(clippy::expect_used)]
fn parse_heroes() -> &'static Map {
    static ONCE: LazyLock<Map> = LazyLock::new(|| {
        bincode::decode_from_slice(HEROES_BIN, bincode::config::standard())
            .expect("failed to parse heroes.bin")
            .0
    });

    &ONCE
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn get_self() {
        Hero::new("npc_dota_hero_antimage").unwrap();
    }

    #[test]
    #[should_panic]
    fn get_self_fake() {
        Hero::new("sometypeofshit").unwrap();
    }

    #[test]
    fn name_getter() {
        let name = "npc_dota_hero_antimage";
        let hero = Hero::new(name).unwrap();
        assert_eq!(name, hero.name());
    }

    #[test]
    fn data_getter() {
        let hero = Hero::new("npc_dota_hero_antimage").unwrap();
        assert!(!hero.data().is_empty());
    }

    #[test]
    fn data_getter_truth() {
        let hero = Hero::new("npc_dota_hero_antimage").unwrap();
        assert_eq!(hero.data().get("CMEnabled").unwrap().get_str().unwrap(), "1");
    }

    #[test]
    fn display_name() {
        let hero = Hero::new("npc_dota_hero_antimage").unwrap();
        assert_eq!(hero.display_name(), "Anti-Mage");
    }

    #[test]
    fn id() {
        let hero = Hero::new("npc_dota_hero_antimage").unwrap();
        assert_eq!(hero.id_v2(), 1);
    }

    #[test]
    fn get_all() {
        let heroes = Hero::all();
        assert!(!heroes.is_empty())
    }
}
