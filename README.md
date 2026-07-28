# rdotaconstants

Rust port of [pydotaconstants](https://github.com/r41ngee/pydotaconstants) — local Dota 2 hero, ability, item, and localization data with zero runtime dependencies.

All game data is embedded directly in the binary at compile time via `include_str!`. No file I/O, no network calls.

## Usage

```rust
use rdotaconstants::{Hero, Ability, Item, locals};

// Heroes
let hero = Hero::get("npc_dota_hero_axe").unwrap();
assert_eq!(hero.id, 2);
assert_eq!(hero.display_name(), "Axe");

let hero = Hero::get_by_id(1).unwrap();           // Anti-Mage
let hero = Hero::get_by_display_name("Axe").unwrap();

// Abilities
let ability = Ability::get("antimage_mana_break").unwrap();
assert_eq!(ability.display_name(), "Mana Break");
assert!(!ability.display_description().is_empty());

// Items
let item = Item::get("item_blink").unwrap();
assert_eq!(item.display_name(), "Blink Dagger");

// All entries
Hero::all();       // 128+ heroes
Ability::all();    // 1291+ abilities
Item::all();       // 544+ items

// Localization
locals().get("npc_dota_hero_axe:n"); // Some("Axe")
```

## API Reference

| Method | Hero | Ability | Item |
|---|---|---|---|
| `get(name)` | by codename | by codename | by codename |
| `get_by_id(id)` | by HeroID (i64) | — | — |
| `get_by_display_name(name)` | by display name | by display name | by display name |
| `all()` | all heroes | all abilities | all items |
| `display_name()` | localized name | localized name | localized name |
| `display_description()` | — | localized description | — |

Each struct also exposes a raw `data: serde_json::Map<String, Value>` field with full access to every Valve-defined field (cooldowns, mana costs, damage types, behavior flags, etc.).

## How It Works

- All JSON data is compiled into the binary via `include_str!`
- On first access, data is parsed once with `serde_json` and cached in a `std::sync::OnceLock`
- Lookup by codename, numeric ID, or localized display name

## Project Structure

```
src/
  lib.rs              # Crate root, re-exports, locals(), tests
  heroes.rs           # Hero struct and lookup methods
  abilities.rs        # Ability struct and lookup methods
  items.rs            # Item struct and lookup methods
  data/
    heroes.json       # ~128 hero definitions
    abilities.json    # ~1291 ability definitions
    items.json        # ~544 item definitions
    locals.json       # ~57,800 localization entries
```

## Testing

```bash
cargo test
```

12 unit tests covering all entity types and lookup methods.

## CI/CD

GitHub Actions automatically:
1. Checks [pydotaconstants](https://github.com/r41ngee/pydotaconstants) for upstream changes (every 2 hours)
2. Downloads fresh JSON data
3. Runs tests and validates data completeness
4. Bumps patch version and publishes to crates.io

## Dependencies

Only `serde` + `serde_json`. No dev dependencies.

## License

MIT

**Data source: [dotabuff/d2vpkr](https://github.com/dotabuff/d2vpkr)**
