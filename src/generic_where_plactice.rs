pub fn main() {
    let n = N { s: 4 };
    let s = genericsample(n);
    println!("{:?}", s)
}

struct N {
    s: i32,
}

fn genericsample(argx: N) -> i32 {
    return argx.s;
}
