use std::collections::HashMap;
use std::sync::{LazyLock};

static LOCALS_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/locals.json"));

/// Global static variable that contains all localization strings for English language.
/// 
/// Check [`Locals`] struct for more information.
pub static LOCALS: LazyLock<Locals> = LazyLock::new(|| {
    Locals { inner: locals() }
});

/// HashMap wrapper that contains all localization strings for English language.
pub struct Locals {
    inner: HashMap<String, String>,
}

impl Locals {
    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.inner.get(key)
    }

    /// Returns a reference to the inner HashMap.
    pub fn inner(&self) -> &HashMap<String, String> {
        &self.inner
    }

    /// Returns a vector of tuples containing references to the key-value pairs in the inner HashMap.
    pub fn as_vec(&self) -> Vec<(&String, &String)> {
        self.inner.iter().collect()
    }

    /// Returns a reference to the value corresponding to the key, ignoring case.
    pub fn get_no_case(&self, key: &str) -> Option<&String> {
        let key_lower = key.to_lowercase();
        self.inner.iter().find_map(|(k, v)| {
            if k.to_lowercase() == key_lower {
                Some(v)
            } else {
                None
            }
        })
    }
}

/// Function that returns a [HashMap]<[String], [String]> with all localization strings for English language.
/// 
/// Panics if the `locals.json` file cannot be parsed. Expected not to panic, as the file is generated at build time and should always be valid.
#[allow(clippy::expect_used)]
fn locals() -> HashMap<String, String> {
    let raw: HashMap<String, serde_json::Value> =
        serde_json::from_str(LOCALS_JSON).expect("failed to parse locals.json");
    raw.into_iter()
        .filter_map(|(k, v)| v.as_str().map(|s| (k, s.to_string())))
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_locals() {
        use crate::*;

        let q = "AbilityDamage";
        let v = "DAMAGE:";
        assert_eq!(LOCALS.get(q).unwrap(), v);
    }
}