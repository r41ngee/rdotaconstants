use std::{fs::File, io::Write, path::PathBuf};

fn main() {
    unpretty(include_str!("data/abilities.json"), "abilities.json");
    unpretty(include_str!("data/heroes.json"), "heroes.json");
    unpretty(include_str!("data/locals.json"), "locals.json");
    unpretty(include_str!("data/items.json"), "items.json");

    println!("cargo::rerun-if-changed=data/abilities.json");
    println!("cargo::rerun-if-changed=data/heroes.json");
    println!("cargo::rerun-if-changed=data/locals.json");
    println!("cargo::rerun-if-changed=data/items.json");
}

// DONT USE / IN to ARG
fn unpretty(from: &str, to: &str) {
    let out = std::env::var("OUT_DIR").unwrap();

    let a: serde_json::Value = serde_json::from_str(from).unwrap();
    let ap = PathBuf::from(out + "/" + to);
    let mut f = File::create(ap).unwrap();
    f.write(serde_json::to_string(&a).unwrap().as_bytes()).unwrap();
}