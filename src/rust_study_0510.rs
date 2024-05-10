use chrono::prelude;
use std::{default, fs::File, io::Read};

pub fn main() {
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

// #[cfg(test)]
// {
//     use super::*;
//     fn main_test(){

//     }
// }

pub fn errsample1() {
    // 参照エラーになるサンプル
    let x = String::new();
    let y = x;
    println!("x is {:?}", x);
}

pub fn oksample1() {
    // これは変数がスタック上に乗ってるのでok
    let x = 5;
    let y = x;
    println!("x is {:?}", x);
}
