use serde_json::Value;

use crate::Entity;

pub(crate) static ABILITIES_JSON: &str = include_str!("data/abilities.json");

#[derive(Debug, Clone)]
/// Represents ability data
pub struct Ability {
    /// Ability slugname
    name: String,
    /// Ability data as [`serde_json::Map`]
    data: serde_json::Map<String, Value>,
}

impl Entity for Ability {
    fn name(&self) -> &str {
        &self.name
    }

    fn data(&self) -> &serde_json::Map<String, Value> {
        &self.data
    }

    fn new<S: AsRef<str>>(name: S) -> Option<Self> {
        let abilities = parse_abilities();
        let raw = abilities.get_key_value(name.as_ref())?;
        if let Value::Object(o) = raw.1 {
                Some(Self { name: raw.0.clone(), data: o.clone() })
        } else { None }
    }

    fn all() -> Vec<Self> {
        let mut result = Vec::new();
        let all_abilities = parse_abilities();
        for k in all_abilities.keys() {
            if let Some(ability) = Self::new(k) {
                result.push(ability);
            }
        }

        result
    }
}
impl crate::private::Sealed for Ability {}

#[allow(clippy::expect_used)]
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
        assert_eq!(data.get("AbilitySound").unwrap(), "Hero_Meepo.Earthbind.Cast");
    }

    #[test]
    fn get_all() {
        let r#abilities = Ability::all();
        assert!(!abilities.is_empty());
    }

    #[test]
    fn entity_get() {
        use serde_json::{Map, Value};

        let ability = Ability {
            name: "test".to_string(),
            data: Map::from_iter([
                ("foo".to_string(), Value::String("bar".to_string())),
            ]),
        };

        assert_eq!(
            ability.get("foo"),
            Some(Value::String("bar".to_string()))
        );

        assert_eq!(ability.get("missing"), None);
    }
}
