use std::{default, fs::File, io::Read};

pub fn main() {
    let mut s: String = String::new();
    let mut f = File::open("./src/rust_study_0510.log").expect("no such file or dirs");
    f.read_to_string(&mut s).unwrap();
    // println!("{}", s);
    #[derive(Default)]
    struct Ers {
        info_count: i32,
        warn_count: i32,
        err_count: i32,
    }
    let mut ers: Ers = Ers::default();
    for l in s.lines() {
        let lsp: std::str::Split<&str> = l.split(",");
        assert_eq!(lsp.count(), 3);
    }
}

// #[cfg(test)]
// {
//     use super::*;
//     fn main_test(){

//     }
// }
