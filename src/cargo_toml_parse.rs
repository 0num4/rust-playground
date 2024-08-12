use std::{collections::HashMap, fs};

use serde::Deserialize;
use toml;

// TOMLの場合セクションごとにstructを作ってstructをstructで組み合わせる
#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
    edition: String,
    license: String,
    description: String,
}

// #[derive(Deserialize)]
// struct Dependency {
//     version: String,
//     features: Option<Vec<String>>,
// }

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Dependency {
    Simple(String),
    Detailed {
        version: String,
        features: Option<Vec<String>>,
    },
}

#[derive(Deserialize)]
struct Cargo {
    package: Package,
    dependencies: HashMap<String, Dependency>,
}

pub fn main() {
    let cargotomlstring = fs::read_to_string("Cargo.toml").unwrap();
    let cargo_toml: Cargo = toml::from_str(&cargotomlstring).unwrap();
    println!("cargo_toml.package.name {:?}", cargo_toml.package.name);
    println!(
        "diesel version {:?}",
        cargo_toml.dependencies.get("diesel").unwrap()
    );
}

// ```
// thread 'main' panicked at src/cargo_toml_parse.rs:30:62:
// called `Result::unwrap()` on an `Err` value: Error { inner: Error { inner: TomlError { message: "missing field `version`", raw: Some("[package]\nname = \"rust-playground\"\nversion = \"0.1.0\"\nedition = \"2021\"\nlicense = \"MIT\"\ndescription = \"A simple Rust playground\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n\n[dependencies]\nanyhow = \"1.0.86\"\ncargo-insta = \"1.39.0\"\nchrono = \"0.4.38\"\ndiesel = { version = \"2.2.1\", features = [\"postgres\"] }\ndotenvy = \"0.15.7\"\nhttp-body-util = \"0.1.1\"\nhyper = { version = \"1.3.1\", features = [\"full\"] }\nhyper-util = { version = \"0.1.3\", features = [\"full\"] }\nlog = \"0.4.21\"\nproconio = \"0.4.5\"\nrand = \"0.8.5\"\nserde = { version = \"1.0.200\", features = [\"derive\"] }\nserde_json = \"1.0.116\"\ntokio = { version = \"1.37.0\", features = [\"full\"] }\ntoml = \"0.8.19\"\n"), keys: ["dependencies"], span: Some(225..700) } } }
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// ```
