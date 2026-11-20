use std::fs;
use std::path::{PathBuf};
use rdc_map::*;



fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let bin_dir = out_dir.join(".bin");
    fs::create_dir_all(&bin_dir).unwrap();

    outdir::write!("abilities.bin", &content(include_str!("data/abilities.json")));
    outdir::write!("heroes.bin", &content(include_str!("data/heroes.json")));
    outdir::write!("items.bin", &content(include_str!("data/items.json")));
    outdir::write!("locals.bin", &content(include_str!("data/locals.json")));

    println!("cargo::rerun-if-changed=data/");
}

fn content(from: &str) -> Vec<u8> {
    let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(from).unwrap();
    let map: Map = map.into();
    bincode_next::encode_to_vec(map, bincode_next::config::standard()).unwrap()
}