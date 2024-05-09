use proconio::input;

pub fn main() {
    input! {
        (n,x,y,z):(i32,i32,i32,i32)
    }
    if x > y {
        if (x > z && z > y) {
            println!("Yes")
        } else {
            println!("No")
        }
    } else {
        if (y > z && z > x) {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
