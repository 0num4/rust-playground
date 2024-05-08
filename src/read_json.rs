use std::fs::File;

use serde_json::{value, Value};

// {
//     "$schema": "https://docs.renovatebot.com/renovate-schema.json",
//     "extends": [
//       "config:recommended"
//     ]
//   }

#[derive(Debug, serde::Deserialize)]
struct Renovate {
    #[serde(rename = "$schema")]
    schema: String,
    extends: Vec<String>,
}

pub fn main() {
    let mut f = File::open("renovate.json").expect("ファイルが開けませんでした");
    let jsonValue: Value = serde_json::from_reader(f).expect("jsonのパースに失敗しました");
    println!("json value is: {:?}", jsonValue);

    // 構造体に代入
    // let mut json: Renovate = serde_json::from_reader(f).expect("jsonのパースに失敗しました");
    // println!("{:?}", json);
}
