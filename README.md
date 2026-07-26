# rdotaconstants

Rust port of [pydotaconstants](https://github.com/r41ngee/pydotaconstants) — local Dota 2 hero, ability, item, and localization data with zero runtime dependencies.

## Usage

```rust
use dotaconstants::{Hero, Ability, Item, locals};

let hero = Hero::get("npc_dota_hero_axe").unwrap();
assert_eq!(hero.display_name(), "Axe");
assert_eq!(hero.id, 2);

let ability = Ability::get("antimage_mana_break").unwrap();
assert_eq!(ability.display_name(), "Mana Break");
assert!(!ability.display_description().is_empty());

let item = Item::get("item_blink").unwrap();
assert_eq!(item.display_name(), "Blink Dagger");

// All objects
Hero::all();       // 128 heroes
Ability::all();    // 1291 abilities
Item::all();       // 544 items

// Localization
locals().get("npc_dota_hero_axe:n"); // Some("Axe")
```

## API

| Method | Hero | Ability | Item |
|--------|------|---------|------|
| `get(name)` | by codename | by codename | by codename |
| `get_by_id(id)` | by HeroID | — | — |
| `get_by_display_name(name)` | by display name | by display name | by display name |
| `all()` | all heroes | all abilities | all items |
| `display_name()` | localized name | localized name | localized name |
| `display_description()` | — | localized description | — |

## Dependencies

Only `serde` + `serde_json`. All data is embedded in the binary via `include_str!`.

## License

MIT

**Data source: [dotabuff/d2vpkr](https://github.com/dotabuff/d2vpkr)**
