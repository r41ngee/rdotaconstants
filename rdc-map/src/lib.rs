#[cfg(feature = "bincode-next")]
use bincode_next as bincode;

/// Type alias for key-value pair as in `vdf` files
pub type Pair = (String, Value);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bincode-next", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Map of key-value pairs ([`Pair`])
pub struct Map {
    inner: Vec<Pair>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bincode-next", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    pub fn new(inner: Vec<Pair>) -> Self {
        Map { inner }
    }

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

    /// Returns is map is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl IntoIterator for Map {
    type Item = Pair;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl Map {
    /// Transforms map into an iterator of [`Pair`] references.
    pub fn iter(&self) -> impl Iterator<Item = &Pair> {
        self.inner.iter()
    }

    /// Transforms map into an iterator of mutable [`Pair`] references.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Pair> {
        self.inner.iter_mut()
    }
}

impl Value {    
    pub fn get_str(&self) -> Option<&str> {
        if let Self::String(s) = self {
            Some(s.as_str())
        } else { None }
    }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Map<String, serde_json::Value>> for Map {
    fn from(val: serde_json::Map<String, serde_json::Value>) -> Self {
        Map::new(val.into_iter().map(|(k, v)| (k, v.into()) ).collect())
    }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Value> for Value {
    fn from(val: serde_json::Value) -> Self {
        match val {
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Object(m) => Value::Map(m.into()),
            _ => panic!(),
        }
    }
}
