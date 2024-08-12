use std::fs;

use serde::Deserialize;
use toml;

// TOMLの場合セクションごとにstructを作ってstructをstructで組み合わせる
#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    edition: String,
    license: String,
    description: String,
}

#[derive(Deserialize)]
struct Dependencies {
    version: String,
    features: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct Cargo {
    package: Package,
    dependencies: Dependencies,
}

pub fn main() {
    let cargotomlstring = fs::read_to_string("Cargo.toml").unwrap();
    let cargo_toml: Cargo = toml::from_str(&cargotomlstring).unwrap();
    println!("cargo_toml.package.name {:?}", cargo_toml.package.name);
}
