pub fn main() {
    let n = N { s: 4 };
    let s = genericsample(n);
    println!("{:?}", s)
}

struct N {
    s: i32,
}

impl N {
    pub fn ss(&self) -> i32 {
        return self.s * self.s;
    }
}

fn genericsample(argx: N) -> i32 {
    return argx.ss();
}
