use serde_json::{Map, Value};

use crate::private::Sealed;

/// Trait that descripts methods
/// that works for all entities
/// described in this crate.
/// 
/// Cannot be implemented for custom types.
pub trait Entity: Sealed + Sized {
    /// Returns an entity slugname.
    fn name(&self) -> &str;
    /// Returns an entity data.
    fn data(&self) -> &Map<String, Value>;
    /// Returns an object of implementing class by its
    /// slugname.
    fn get_self<S: AsRef<str>>(s: S) -> Option<Self>;
    /// Returns all possible variants
    /// of this ability.
    fn all() -> Vec<Self>;
    /// Returns data value for this object.
    fn get<Q: AsRef<str>>(&self, k: Q) -> Option<Value> {
        self.data().get(k.as_ref()).cloned()
    }
}
