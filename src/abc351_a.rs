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
    // let mut count = 0;
    for (i, _) in d.iter().enumerate() {
        if i > d.len() - 3 {
            println!("No");
            exit(0);
        }
        if d[i][0] == d[i][1] && d[i + 1][0] == d[i + 1][1] && d[i + 2][0] == d[i + 2][1] {
            println!("Yes");
            exit(0);
        }
        // if count > 2 {
        //     println!("Yes");
        //     exit(0);
        // }
    }
    println!("No")
}
pub fn c() {}
