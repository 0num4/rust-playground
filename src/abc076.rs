use proconio::input;

pub fn b() {
    input! {
        n:i32,
        k: i32,
    }
    let mut l = 1;
    for i in 0..n {
        if l + k < l * 2 {
            l = l + k;
        } else {
            l = l * 2;
        }
    }
    println!("l = {}", l);
}
