use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

pub type Pair = (String, Value);

#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(Decode, Encode)]
pub struct Map {
    inner: Vec<Pair>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(Decode, Encode)]
pub enum Value {
    String(String),
    Map(Map),
}

impl Map {
    pub fn get(&self, q: impl AsRef<str>) -> Option<&Value> {
        self.inner.iter().find(|x| x.0 == q.as_ref()).map(|(_, v)| v)
    }

    pub fn inner(&self) -> &Vec<Pair> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Vec<Pair> {
        &mut self.inner
    }

    pub fn take(self) -> Vec<Pair> {
        self.inner
    }

    pub fn keys(&self) -> Vec<&str> {
        self.inner.iter().map(|(k, _)| k.as_str()).collect()
    }

    pub fn values(&self) -> Vec<&Value> {
        self.inner.iter().map(|(_, v)| v).collect()
    }

    pub fn get_key_value<Q: AsRef<str>>(&self, q: Q) -> Option<(Q, &Value)> {
        self.get(&q).map(|v| (q, v))
    }
}

impl Value {
    pub fn get_str(&self) -> Option<&str> {
        if let Self::String(s) = self {
            Some(s.as_str())
        } else { None }
    }
}
