pub type Pair = Vec<(String, Value)>;

pub struct Map {
    inner: Pair,
}

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
}