/*!
 * rdotaconstants - library created to work with Dota constant values
 */
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
/// Contains [`locals()`] function
pub mod locals;

pub use heroes::Hero;
pub use abilities::Ability;
pub use items::Item;
pub use entity::Entity;
pub use locals::locals;

mod private {
    pub trait Sealed { }
}
