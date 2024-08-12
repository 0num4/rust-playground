use std::{fs::File, io::Read};

use anyhow::{Ok, Result};
use diesel::result;
// pub fn q1() {
//     fn process_file(file_name: &str) -> Result<f64> {
//         // ファイルを読み込む
//         // let mut f = File::open(file_name).expect("file not found");
//         // let mut s = String::new();
//         let s2 = std::fs::read_to_string(file_name)?;
//         let nums: f64 = s2
//             .lines()
//             .map(|line| {
//                 return line
//                     .parse::<i32>()
//                     .map_err(anyhow::Error::from)
//                     .map(|num| num as f64);
//             })
//             .collect::<Result<Vec<f64>>>()?
//             .iter()
//             .sum();

//         return Ok(nums as f64 / 2.0);
//     }

//     match process_file("nums.txt") {
//         Ok(result) => println!("result {}", result),
//         Err(e) => panic!("err!!!"),
//     }
// }
pub fn main() {
    // q1();
}
