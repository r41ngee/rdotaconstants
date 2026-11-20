#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(missing_docs)]

/// Contains [`Hero`] class
pub mod heroes;
/// Contains [`Ability`] class
pub mod abilities;
/// Contains [`Item`] class
pub mod items;
/// Contains [`Entity`] trait
pub mod entity;
/// Contains [`LOCALS`] static
pub mod locals;

/// KV related types
///
/// As [`Map`](map::Map), [`Value`](map::Value) and [`Pair`](map::Pair) types and their logic was
/// moved into the separate [`rdc_map`] crate, to not break compatibility this is module with public
/// usage of these types.
pub mod map {
    pub use rdc_map::*;
}

pub use heroes::Hero;
pub use abilities::Ability;
pub use items::Item;
pub use entity::Entity;
pub use locals::LOCALS;

mod private {
    use crate::Entity;

    pub trait Sealed { }
    impl<T: Entity> Sealed for T {}
}
