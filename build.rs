use std::{fs::{self, File}, path::{Path, PathBuf}};

include!("src/map.rs");

impl Into<Map> for serde_json::Map<String, serde_json::Value> {
    fn into(self) -> Map {
        Map {
            inner: self.into_iter().map(|(k, v)| (k, v.into()) ).collect()
        }
    }
}

impl Into<Value> for serde_json::Value {
    fn into(self) -> Value {
        match self {
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

    println!("cargo::rerun-if-changed=data/abilities.json");
    println!("cargo::rerun-if-changed=data/heroes.json");
    println!("cargo::rerun-if-changed=data/locals.json");
    println!("cargo::rerun-if-changed=data/items.json");
}

fn write_into_bin(from: &str, to: &Path) {
    let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(from).unwrap();
    let map: Map = map.into();

    let mut f = File::create(to).unwrap();
    bincode::encode_into_std_write(map, &mut f, bincode::config::standard()).unwrap();
}