pub fn main() {
    let n = N { s: 4 };
    let s = genericsample(n);
    println!("{:?}", s)
}
trait SS {
    fn ss(&self) -> i32;
}
struct N {
    s: i32,
}

impl SS for N {
    fn ss(&self) -> i32 {
        return self.s * self.s;
    }
}

fn genericsample<T: SS>(argx: T) -> i32 {
    return argx.ss();
}
