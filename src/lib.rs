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
/// Contains storaging system of `vdf` files
pub mod map;

pub use heroes::Hero;
pub use abilities::Ability;
pub use items::Item;
pub use entity::Entity;
pub use locals::LOCALS;

mod private {
    pub trait Sealed { }
}
