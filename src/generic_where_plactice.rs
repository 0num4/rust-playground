pub fn main() {
    let s = genericsample(5);
    println!("{:?}", s)
}

fn genericsample<T>(argx: T) -> i32 {
    return 3;
}
