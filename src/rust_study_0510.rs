use chrono::prelude;
use std::{default, fs::File, io::Read};

pub fn mainold() {
    errsample1();
    oksample1();
    //     let mut s: String = String::new();
    //     let mut f = File::open("./src/rust_study_0510.log").expect("no such file or dirs");
    //     f.read_to_string(&mut s).unwrap();
    //     // println!("{}", s);
    //     #[derive(Default)]
    //     struct Ers {
    //         info_count: i32,
    //         warn_count: i32,
    //         err_count: i32,
    //     }
    //     let mut ers: Ers = Ers::default();
    //     for l in s.lines() {
    //         let mut lsp: std::str::Split<&str> = l.split(",");
    //         assert_eq!(lsp.count(), 3);
    //         let lsp2 = lsp.next().unwrap();
    //     }
}

fn main() {
    let strings = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
        String::from("date"),
    ];

    let stats = process_strings(&strings);

    println!("Even strings: {:?}", stats.even_strings);
    println!("Odd strings: {:?}", stats.odd_strings);
}
fn process_strings(v: &Vec<String>) -> StringStats {
    let mut odd: Vec<&str> = Vec::new();
    let mut even: Vec<&str> = Vec::new();
    for vin in v {
        odd.push(vin.as_str());
    }
    return StringStats {
        odd_strings: odd,
        even_strings: even,
    };
}

struct StringStats<'a> {
    odd_strings: Vec<&'a str>,
    even_strings: Vec<&'a str>,
}

// #[cfg(test)]
// {
//     use super::*;
//     fn main_test(){

//     }
// }

pub fn errsample1() {
    // 参照エラーになるサンプル
    let x: Box<i32> = Box::new(5);
    // let y: Box<i32> = x;
    processvec(&x);
    println!("x is {:?}", x);
}

fn processvec(x: &Box<i32>) {
    println!("Destroying box that contains {}", x);
}

pub fn oksample1() {
    // これは変数がスタック上に乗ってるのでok
    let x = 5;
    let y = x;
    println!("x is {:?}", x);
}
