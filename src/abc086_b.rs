use proconio::input;

pub fn main() {
    input! {
        x: i32,
        a: i32,
        b: i32
    }
    // let c: i32 = (a + &b).parse().unwrap();
    // println!("{:?}", c);
    print!("{}", (x - a) % b);
}
