use std::process::exit;

// https://atcoder.jp/contests/abs/tasks/practice_1
use proconio::input;
pub fn main() {
    abc081a();
}
pub fn a() {
    input! {
        a: i32,
        (b, c): (i32,i32),
        s: String
    }
    print!("{} {}", a + b + c, s);
    return println!("");
}

pub fn abc081a() {
    input! {
        s: String
    }
    println!("{:?}", s.chars().filter(|&x| x == '1').count())
}

pub fn abc081b() {
    input! {
        n: usize,
        a: [usize;n]
    }
    let min = a.iter().min().unwrap().clone();
    for ai in a {
        if ai % 2 != 0 {
            println!("0");
            exit(0);
        }
    }

    // println!("{}", log2(min));
    // math::a[0]
    // for (i, ai) in a.iter().enumerate() {
    //     if ai % 2 != 0 {
    //         println!("0");
    //         exit(0)
    //     } else {
    //     }
    // }
    // println!("{:?}", s.chars().filter(|&x| x == '1').count())
}

// https://atcoder.jp/contests/ahc032/tasks/ahc032_a
pub fn ahc032_a() {
    input! {
        (n, m, k):(i32,i32,i32),
        a: [[i32;n];n],
        s:[[i32;3];m*3]
    }
    println!()
}
