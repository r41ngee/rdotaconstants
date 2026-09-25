use crate::map::*;
use crate::{Entity, LOCALS};

use bincode_next as bincode;

static ABILITIES_BIN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/.bin/abilities.bin"));

#[derive(Debug)]
/// Represents ability data
pub struct Ability {
    /// Ability slugname
    name: String,
    /// Ability data as [`serde_json::Map`]
    data: Map,
}

impl Ability {
    /// Returns display name of ability.
    pub fn display_name(&self) -> Option<&str> {
        LOCALS.get_no_case(&format!("DOTA_Tooltip_ability_{}", self.name()))
            .map(|x| x.as_str())
    }

    /// Returns description of ability.
    pub fn display_description(&self) -> Option<&str> {
        LOCALS.get_no_case(&format!("DOTA_Tooltip_ability_{}_Description", self.name()))
            .map(|x| x.as_str())
    }

    /// Returns slugname of hero owning this ability.
    pub fn owner(&self) -> Option<&str> {
        match self.data().get("owner") {
            Some(Value::String(s)) => Some(s.as_str()),
            _ => None,
        }
    }

    fn from_entry(name: &str, data: &Value) -> Option<Self> {
        Some(Self { name: name.to_string(), data: match data {
            Value::Map(v) => v.clone(),
            _ => return None,
        }})
    }
}

impl Entity for Ability {
    fn name(&self) -> &str {
        &self.name
    }

    fn data(&self) -> &Map {
        &self.data
    }

    fn new<S: AsRef<str>>(name: S) -> Option<Self> {
        let abilities = parse_abilities();
        let (name, map) = abilities.get_key_value(name.as_ref())?;
        Some(Self {
            name: name.to_string(),
            data: if let Value::Map(m) = map {
                m.clone()
            } else {
                return None;
            }
        })
    }

    fn all() -> Vec<Self> {
        parse_abilities()
            .inner()
            .iter()
            .filter_map(|(name, value)| Self::from_entry(name, value))
            .collect()
    }
}
impl crate::private::Sealed for Ability {}

mod properties;

#[allow(clippy::expect_used)]
fn parse_abilities() -> &'static Map {
    use std::sync::OnceLock;
    static ONCE: OnceLock<Map> = OnceLock::new();
    ONCE.get_or_init(|| {
        let raw: Map = bincode::decode_from_slice(ABILITIES_BIN, bincode::config::standard())
            .expect("failed to parse abilities.bin")
            .0;
        raw
    })
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn get_self() {
        let ability = Ability::new("meepo_earthbind");

        assert!(ability.is_some());
    }

    #[test]
    fn get_self_fail() {
        let ability = Ability::new("meepo_fucking_shit");

        assert!(ability.is_none());
    }

    #[test]
    fn name_getter() {
        let ab_name = "meepo_earthbind";
        let ab = Ability::new(ab_name).unwrap();
        assert_eq!(ab_name, ab.name())
    }

    #[test]
    fn data_getter() {
        let ability = Ability::new("meepo_earthbind").unwrap();
        assert!(!ability.data().is_empty())
    }

    #[test]
    fn data_getter_truth() {
        let ability = Ability::new("meepo_earthbind").unwrap();
        let data = ability.data();
        assert_eq!(data.get("AbilitySound").unwrap().get_str().unwrap(), "Hero_Meepo.Earthbind.Cast");
    }

    #[test]
    fn get_all() {
        let r#abilities = Ability::all();
        assert!(!abilities.is_empty());
    }

    #[test]
    fn get_owner() {
        let ability = Ability::new("meepo_earthbind").unwrap();
        assert_eq!(ability.owner().unwrap(), "npc_dota_hero_meepo");
    }

    #[test]
    fn get_display_name() {
        let ability = Ability::new("nevermore_requiem").unwrap();
        assert_eq!(ability.display_name().unwrap(), "Requiem of Souls");
    }

    #[test]
    fn get_display_description() {
        let ability = Ability::new("nevermore_requiem").unwrap();
        let desc = "Shadow Fiend gathers up to %max_soul_release% of his captured souls to release them as lines of demonic energy. Units near Shadow Fiend when the souls are released can be damaged by several lines of energy. Any unit damaged by Requiem of Souls will be feared and have its movement speed and magic resistance reduced for %requiem_slow_duration% seconds for each line hit up to a maximum of %requiem_slow_duration_max%. Lines of energy are created for every soul captured through Necromastery. <br><br> Requiem of Souls is automatically cast whenever Shadow Fiend dies, regardless of its cooldown.";
        assert_eq!(ability.display_description().unwrap(), desc);
    }
}
