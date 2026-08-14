use std::collections::HashMap;
use std::sync::OnceLock;

static LOCALS_JSON: &str = include_str!("data/locals.json");

/// Function that returns a [HashMap]<[String], [String]> with all localization strings for English language.
#[allow(clippy::expect_used)]
pub fn locals() -> &'static HashMap<String, String> {
    static ONCE: OnceLock<HashMap<String, String>> = OnceLock::new();
    ONCE.get_or_init(|| {
        let raw: HashMap<String, serde_json::Value> =
            serde_json::from_str(LOCALS_JSON).expect("failed to parse locals.json");
        raw.into_iter()
            .filter_map(|(k, v)| v.as_str().map(|s| (k, s.to_string())))
            .collect()
    })
}