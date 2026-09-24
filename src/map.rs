use bincode_next::{Decode, Encode};

/// Type alias for key-value pair as in `vdf` files
pub type Pair = (String, Value);

#[derive(Debug, Clone)]
#[derive(Decode, Encode)]
/// Map of key-value pairs ([`Pair`])
pub struct Map {
    inner: Vec<Pair>,
}

#[derive(Debug, Clone)]
#[derive(Decode, Encode)]
/// Value of [`Pair`].
/// 
/// Can be [`String`] or child [`Map`].
pub enum Value {
    /// Terminal variant
    String(String),
    /// Recursive variant
    Map(Map),
}

impl Map {
    /// Finds a value by key.
    pub fn get(&self, q: impl AsRef<str>) -> Option<&Value> {
        self.inner.iter().find(|x| x.0 == q.as_ref()).map(|(_, v)| v)
    }

    /// Takes reference to bare representation.
    pub fn inner(&self) -> &Vec<Pair> {
        &self.inner
    }

    /// Takes mutable reference to bare representation.
    pub fn inner_mut(&mut self) -> &mut Vec<Pair> {
        &mut self.inner
    }

    /// Takes bare representation owning back.
    pub fn take(self) -> Vec<Pair> {
        self.inner
    }

    /// Returns list of keys.
    pub fn keys(&self) -> Vec<&str> {
        self.inner.iter().map(|(k, _)| k.as_str()).collect()
    }

    /// Returns list of values.
    pub fn values(&self) -> Vec<&Value> {
        self.inner.iter().map(|(_, v)| v).collect()
    }

    /// Like [`get()`](Self::get()), but returns tuple with queried key.
    pub fn get_key_value<Q: AsRef<str>>(&self, q: Q) -> Option<(Q, &Value)> {
        self.get(&q).map(|v| (q, v))
    }

    /// Returns is map is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Value {
    #[allow(dead_code)]
    pub(crate) fn get_str(&self) -> Option<&str> {
        if let Self::String(s) = self {
            Some(s.as_str())
        } else { None }
    }
}
