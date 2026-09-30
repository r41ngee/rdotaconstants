use std::fs;
use std::path::{PathBuf};

include!("src/map.rs");

impl From<serde_json::Map<String, serde_json::Value>> for Map {
    fn from(val: serde_json::Map<String, serde_json::Value>) -> Self {
        Map {
            inner: val.into_iter().map(|(k, v)| (k, v.into()) ).collect()
        }
    }
}
impl From<serde_json::Value> for Value {
    fn from(val: serde_json::Value) -> Self {
        match val {
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Object(m) => Value::Map(m.into()),
            _ => panic!(),
        }
    }
}

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let bin_dir = out_dir.join(".bin");
    fs::create_dir_all(&bin_dir).unwrap();

    outdir::write!("abilities.bin", &content(include_str!("data/abilities.json"))).unwrap();
    outdir::write!("heroes.bin", &content(include_str!("data/heroes.json"))).unwrap();
    outdir::write!("items.bin", &content(include_str!("data/items.json"))).unwrap();
    outdir::write!("locals.bin", &content(include_str!("data/locals.json"))).unwrap();

    println!("cargo::rerun-if-changed=data/");
}

fn content(from: &str) -> Vec<u8> {
    let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(from).unwrap();
    let map: Map = map.into();
    bincode_next::encode_to_vec(map, bincode_next::config::standard()).unwrap()
}