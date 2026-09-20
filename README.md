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

## How it works

- Upstream JSON is converted into compact binary blobs during the build step.
- The generated binary data is embedded into the crate via `include_bytes!`.
- The first access initializes the in-memory cache using `std::sync::LazyLock` / `OnceLock`.
- All lookups are done in-memory, without reading files or hitting the network at runtime.

## Runtime dependencies

The crate itself depends on:

- `bincode = "2"`
- `serde = { version = "1", features = ["derive"] }`

This keeps the public API lightweight while avoiding a runtime dependency on external game data files.

## Unstable feature

The crate has an optional `unstable` feature that enables a display-name-based lookup for items:

```rust
#[cfg(feature = "unstable")]
{
    use rdotaconstants::Item;
    let item = Item::get_by_display_name("Aeon Disk").unwrap();
    assert_eq!(item.name(), "item_aeon_disk");
}
```

## License

MIT

**Data source:** [dotabuff/d2vpkr](https://github.com/dotabuff/d2vpkr)
