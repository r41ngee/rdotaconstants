use std::{collections::HashMap, str::FromStr};
use serde_json::Value;

use crate::{locals, errors};

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
    /// Resolves any string field for [Hero] contained in `heroes.json`.
    /// No depth fields allowed.
    /// 
    /// Does not calls panics.
    /// 
    /// # Example
    /// ```
    /// use rdotaconstants::Hero;
    /// let hero = Hero::get("npc_dota_hero_antimage").unwrap();
    /// 
    /// let v: i32 = hero.resolve_value("CMEnabled").unwrap();
    /// assert_eq!(v, 1);
    /// 
    /// let v2: String = hero.resolve_value("Team").unwrap();
    /// assert_eq!(v2, "Good");
    /// ```
    pub fn resolve_value<T: FromStr, S: Into<String>>(&self, key: S) -> Result<T, errors::ResolveValueError> {
        let key = key.into();
        let v = self.data.get(&key).ok_or_else(|| errors::ResolveValueError::KeyNotFound(key))?;
        let rv = match v {
            Value::String(s) => Ok(s),
            _ => Err(errors::ResolveValueError::DepthQuery),
        }?;
        let parsed = rv.parse::<T>();
        parsed.map_err(|_| errors::ResolveValueError::StringParseFail(rv.to_string()))
    }

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

    /**  Method use to get hero's [PrimaryAttribute]
    * 
    * # Example
    * ```
    * use rdotaconstants::{Hero, heroes::PrimaryAttribute};
    * let hero = Hero::get("npc_dota_hero_axe").unwrap();
    * if let PrimaryAttribute::Strength = hero.get_primary_attribute().unwrap() {} else { panic!() }
    * ```
    * 
    * # Panics
    * 1. `"AttributePrimary"` was not found in `heroes.json`
    * 2. `"AttributePrimary"` is not value in `heroes.json`
    * 3. `"AttributePrimary"` does not matches pattern `"DOTA_ATTRIBUTE_*"`
    */
    pub fn get_primary_attribute(&self) -> Result<PrimaryAttribute, errors::ResolveValueError> {
        const KEY: &'static str = "AttributePrimary";
        let s: String = self.resolve_value(KEY)?;
        Ok(match s.as_str() {
            "DOTA_ATTRIBUTE_STRENGTH" => PrimaryAttribute::Strength,
            "DOTA_ATTRIBUTE_AGILITY" => PrimaryAttribute::Agility,
            "DOTA_ATTRIBUTE_INTELLECT" => PrimaryAttribute::Intelligence,
            "DOTA_ATTRIBUTE_ALL" => PrimaryAttribute::Universal,
            _ => panic!(),
        })
    }

    /// Returns an [AttributeTable] for this hero
    pub fn get_attr_table(&self) -> Result<AttributeTable, errors::ResolveValueError> {
        Ok(AttributeTable {
            strength_base: self.resolve_value("AttributeBaseStrength")?,
            strength_gain: self.resolve_value("AttributeStrengthGain")?,
            agility_base: self.resolve_value("AttributeBaseAgility")?,
            agility_gain: self.resolve_value("AttributeAgilityGain")?,
            intelligence_base: self.resolve_value("AttributeBaseIntelligence")?,
            intelligence_gain: self.resolve_value("AttributeIntelligenceGain")?,
        })
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

/// Enum representing a hero's primary attribute
pub enum PrimaryAttribute {
    Strength,
    Agility,
    Intelligence,
    Universal
}

/// Represents hero's strength, agility, intelligence base
/// and gain.
pub struct AttributeTable {
    /// Strength base value
    pub strength_base: f32,
    /// Strength gain per level
    pub strength_gain: f32,
    /// Agility base value
    pub agility_base: f32,
    /// Agility gain per level
    pub agility_gain: f32,
    /// Intelligence base value
    pub intelligence_base: f32,
    /// Intelligence gain per level
    pub intelligence_gain: f32,
}

fn parse_heroes() -> &'static HashMap<String, serde_json::Map<String, Value>> {
    use std::sync::OnceLock;
    static ONCE: OnceLock<HashMap<String, serde_json::Map<String, Value>>> = OnceLock::new();
    ONCE.get_or_init(|| {
        serde_json::from_str(HEROES_JSON).expect("failed to parse heroes.json")
    })
}
