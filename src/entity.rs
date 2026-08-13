use serde_json::{Map, Value};

use crate::private::Sealed;

/// Trait that descripts methods
/// that works for both Ability and Item.
/// 
/// Cannot be implemented for custom types.
pub trait Entity: Sealed + Sized {
    /// Method used to get an entity slugname
    fn name(&self) -> &str;
    fn data(&self) -> &Map<String, Value>;
    fn get_self<S: AsRef<str>>(s: S) -> Option<Self>;
    fn all() -> Vec<Self>;
    fn get<Q: AsRef<str>>(&self, k: Q) -> Option<Value> {
        self.data().get(k.as_ref()).cloned()
    }
}

pub trait Executable: Entity {

}
