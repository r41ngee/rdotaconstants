# rdotaconstants

![Crates.io Version](https://img.shields.io/crates/v/rdotaconstants)


![docs.rs](https://img.shields.io/docsrs/rdotaconstants)
![Deps.rs Crate Dependencies (latest)](https://img.shields.io/deps-rs/rdotaconstants/latest)
![GitHub last commit](https://img.shields.io/github/last-commit/r41ngee/rdotaconstants)
![Crates.io License](https://img.shields.io/crates/l/rdotaconstants)



Rust port of [pydotaconstants](https://github.com/r41ngee/pydotaconstants) — local Dota 2 hero, ability, item, and localization data with zero runtime dependencies.

All game data is embedded directly in the binary at compile time via `include_str!`. No file I/O, no network calls.

## Usage

```rust
use rdotaconstants::{Hero, Ability, Item, locals};
use rdotaconstants::Entity;

// Heroes
let hero = Hero::new("npc_dota_hero_axe").unwrap();
assert_eq!(hero.id(), 2);
assert_eq!(hero.display_name(), "Axe");

let hero = Hero::from_id(1).unwrap(); // Anti-Mage

// Abilities
let ability = Ability::new("antimage_mana_break").unwrap();
assert_eq!(ability.display_name().unwrap(), "Mana Break");
assert!(!ability.display_description().unwrap().is_empty());

// Items
let item = Item::new("item_blink").unwrap();
assert_eq!(item.display_name().unwrap(), "Blink Dagger");

// All entries
Hero::all();       // 128+ heroes
Ability::all();    // 1291+ abilities
Item::all();       // 544+ items

// Localization
locals().get("npc_dota_hero_axe:n"); // Some("Axe")
```

## How It Works

- All JSON data is compiled into the binary via `include_str!`
- On first access, data is parsed once with `serde_json` and cached in a `std::sync::OnceLock`
- Lookup by codename, numeric ID, or localized display name

## Project Structure

```text
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
