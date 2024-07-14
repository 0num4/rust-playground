use proconio::input;

pub fn main() {
    a();
    // println!("nyaaa");
    // b();
    // c();
}

pub fn a() {
    input! {
        (r, g, b): (usize, usize, usize),
        c: String
    }
    println!("{}", c);
    match c.as_str() {
        "Red" => println!("{}", std::cmp::min(g, b)),
        "Green" => println!("{}", std::cmp::min(r, b)),
        "Blue" => println!("{}", std::cmp::min(r, g)),
        _ => panic!("Invalid color"),
    }
}
