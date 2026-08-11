/*!
 * rdotaconstants - library created to work with Dota constant values
 * 
 * Currently supports [Ability], [Item], [Hero] and [locals] objects.
 */
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod heroes;
pub mod abilities;
pub mod items;
pub mod errors;

use std::collections::HashMap;
use std::sync::OnceLock;

static LOCALS_JSON: &str = include_str!("data/locals.json");

/// Function that returns a [HashMap]<[String], [String]> with all localization strings for English language.
pub fn locals() -> &'static HashMap<String, String> {
    static ONCE: OnceLock<HashMap<String, String>> = OnceLock::new();
    ONCE.get_or_init(|| {
        let raw: HashMap<String, serde_json::Value> =
            serde_json::from_str(LOCALS_JSON).expect("failed to parse locals.json");
        raw.into_iter()
            .filter_map(|(k, v)| v.as_str().map(|s| (k, s.to_string())))
            .collect()
    })
}

pub use heroes::Hero;
pub use abilities::Ability;
pub use items::Item;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hero_get() {
        let hero = Hero::get("npc_dota_hero_axe").unwrap();
        assert_eq!(hero.id, 2);
        assert_eq!(hero.display_name(), "Axe");
    }

    #[test]
    fn test_hero_get_by_id() {
        let hero = Hero::get_by_id(1).unwrap();
        assert_eq!(hero.name, "npc_dota_hero_antimage");
    }

    #[test]
    fn test_hero_get_by_display_name() {
        let hero = Hero::get_by_display_name("Anti-Mage").unwrap();
        assert_eq!(hero.name, "npc_dota_hero_antimage");
    }

    #[test]
    fn test_hero_all() {
        let all = Hero::all();
        assert!(all.len() > 100);
    }

    #[test]
    fn test_ability_get() {
        let ability = Ability::get("antimage_mana_break").unwrap();
        assert_eq!(ability.display_name().unwrap(), "Mana Break");
    }

    #[test]
    fn test_ability_display_description() {
        let ability = Ability::get("antimage_mana_break").unwrap();
        let desc = ability.display_description();
        assert!(!desc.unwrap().is_empty());
    }

    #[test]
    fn test_ability_all() {
        let all = Ability::all();
        assert!(all.len() > 1000);
    }

    #[test]
    fn test_item_get() {
        let item = Item::get("item_blink").unwrap();
        assert_eq!(item.display_name().unwrap(), "Blink Dagger");
    }

    #[test]
    fn test_item_all() {
        let all = Item::all();
        assert!(all.len() > 500);
    }

    #[test]
    fn test_locals() {
        let l = locals();
        assert!(l.len() > 50000);
        assert_eq!(l.get("npc_dota_hero_axe:n").unwrap(), "Axe");
    }
}
