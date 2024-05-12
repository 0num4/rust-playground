use std::process::exit;

use proconio::input;
pub fn main() {
    // a();
    // println!("nyaaa");
    // b();
    c();
}
pub fn a() {
    input! {
        n: usize,
        h: [usize;n]
    }
    // let mut l = h.clone();
    // l.sort();
    // if h[0] < l[l.len()] {}
    // let mut hier_index = 0;
    for (hi, i) in h.iter().enumerate() {
        if *i > h[0] {
            // println!("hi = {}, {}, hier_indexを更新します{}", hi, i, hier_index);
            // hier_index = hi;
            println!("{}", hi + 1);
            exit(0)
        }
    }
    // if h[hier_index] == h[0] {
    println!("-1");
    // } else {
    //     println!("{}", hier_index + 1);
    // }
}
pub fn b() {
    input! {
        (n, k): (usize,usize),
        a: [usize;n]
    }
    let mut aiter = a.iter().enumerate().peekable();
    let mut ni = 1;
    let mut ktmp = 0;

    loop {
        match aiter.next() {
            Some((index, &value)) => {
                ktmp += value;
                // println!(
                //     "Current index: {}, value: {}, ktmp: {}, ni: {}",
                //     index, value, ktmp, ni
                // );
                if ktmp > k {
                    ni += 1;
                    ktmp = value;
                    continue;
                }
            }
            None => break,
        }
    }
    println!("{}", ni)
    // let anya = aiter.next().unwrap();
}

fn c() {
    // f = (x + y) % 10 ^ 8;
    let s = 100000000;
    input! {
        n: usize,
        a: [usize;n]
    }
    let mut sum = 0;
    for i in 0..n {
        for j in i + 1..n {
            sum += (a[i] + a[j]) % s;
            // println!("x+y:{}, f(x,y): {}", a[i] + a[j], (a[i] + a[j]) % s)
        }
    }
    println!("{}", sum);
}
