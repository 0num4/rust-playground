use std::fs::File;

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

    let mut json: Renovate = serde_json::from_reader(f).expect("jsonのパースに失敗しました");
    println!("{:?}", json);
}
