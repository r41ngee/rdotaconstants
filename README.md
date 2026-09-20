# rdotaconstants

![Crates.io Version](https://img.shields.io/crates/v/rdotaconstants)
![docs.rs](https://img.shields.io/docsrs/rdotaconstants)
![Deps.rs Crate Dependencies (latest)](https://img.shields.io/deps-rs/rdotaconstants/latest)
![GitHub last commit](https://img.shields.io/github/last-commit/r41ngee/rdotaconstants)
![Crates.io License](https://img.shields.io/crates/l/rdotaconstants)

Rust port of [pydotaconstants](https://github.com/r41ngee/pydotaconstants): local Dota 2 hero, ability, item, and localization data embedded into the binary at build time.

This crate exposes the data as strongly typed lookup objects and keeps runtime access to a cached in-memory map generated from the upstream JSON sources.

## Features

- Hero lookup by codename and numeric ID
- Ability lookup by codename with display name and description
- Item lookup by codename with cost and raw data access
- Full localization table via `LOCALS`
- No file I/O or network calls at runtime
- Works with Rust 1.85.1+

## Installation

```toml
[dependencies]
rdotaconstants = "0.5.0"
```

## Quick start

```rust
use rdotaconstants::{Ability, Entity, Hero, Item, LOCALS};

// Heroes
let hero = Hero::new("npc_dota_hero_axe").unwrap();
assert_eq!(hero.name(), "npc_dota_hero_axe");
assert_eq!(hero.id(), 2);
assert_eq!(hero.display_name(), "Axe");

let anti_mage = Hero::from_id(1).unwrap();
assert_eq!(anti_mage.name(), "npc_dota_hero_antimage");

// Abilities
let ability = Ability::new("antimage_mana_break").unwrap();
assert_eq!(ability.display_name().unwrap(), "Mana Break");
assert!(!ability.display_description().unwrap().is_empty());
assert_eq!(ability.owner().unwrap(), "npc_dota_hero_antimage");

// Items
let item = Item::new("item_blink").unwrap();
assert_eq!(item.display_name().unwrap(), "Blink Dagger");
assert_eq!(item.get_cost().unwrap(), 2250);

// All entries
let heroes = Hero::all();
let abilities = Ability::all();
let items = Item::all();
assert!(!heroes.is_empty());
assert!(!abilities.is_empty());
assert!(!items.is_empty());

// Localization
assert_eq!(LOCALS.get("npc_dota_hero_axe:n").unwrap(), "Axe");
```

## Data model

Each entity implements the `Entity` trait:

```rust
use rdotaconstants::Entity;

// Common API for all entity types
// fn name(&self) -> &str
// fn data(&self) -> &crate::map::Map
// fn new<S: AsRef<str>>(s: S) -> Option<Self>
// fn all() -> Vec<Self>
// fn get<Q: AsRef<str>>(&self, k: Q) -> Option<&crate::map::Value>
```

This gives access to the raw key-value metadata for each object while keeping the convenient lookup helpers on the concrete types.

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
