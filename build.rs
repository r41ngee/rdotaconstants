use std::{fs::{self, File}, path::{Path, PathBuf}};

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

    write_into_bin(include_str!("data/abilities.json"), &bin_dir.join("abilities.bin"));
    write_into_bin(include_str!("data/heroes.json"), &bin_dir.join("heroes.bin"));
    write_into_bin(include_str!("data/locals.json"), &bin_dir.join("locals.bin"));
    write_into_bin(include_str!("data/items.json"), &bin_dir.join("items.bin"));

    println!("cargo::rerun-if-changed=data/");
}

fn write_into_bin(from: &str, to: &Path) {
    let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(from).unwrap();
    let map: Map = map.into();

    let mut f = File::create(to).unwrap();
    bincode_next::encode_into_std_write(map, &mut f, bincode_next::config::standard()).unwrap();
}