use serde::{Deserialize, Serialize};

pub type Pair = Vec<(String, Value)>;

#[derive(Deserialize, Serialize)]
pub struct Map {
    inner: Pair,
}

#[derive(Deserialize, Serialize)]
pub enum Value {
    String(String),
    Map(Map),
}

impl Map {
    pub fn get(&self, q: impl AsRef<str>) -> Option<&Value> {
        self.inner.iter().find(|x| x.0 == q.as_ref()).map(|(_, v)| v)
    }

    pub fn inner(&self) -> &Pair {
        &self.inner
    }

    pub fn keys(&self) -> Vec<&str> {
        self.inner.iter().map(|(k, _)| k.as_str()).collect()
    }

    pub fn values(&self) -> Vec<&Value> {
        self.inner.iter().map(|(_, v)| v).collect()
    }
}
