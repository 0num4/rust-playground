use std::process::exit;

use proconio::input;
pub fn main() {
    b();
}
pub fn a() {
    input! {
        x: String
    }
    if x.ends_with("s") {
        println!("{}", x + "es");
    } else {
        println!("{}", x + "s");
    }
}
pub fn b() {
    input! {
        n: i32,
        d: [[i32;2];n],
    }
    let mut count = 0;
    for di in d {
        if di[0] == di[1] {
            count += 1;
        }
        if count > 2 {
            println!("Yes");
            exit(0);
        }
    }
    println!("No")
}
pub fn c() {}
