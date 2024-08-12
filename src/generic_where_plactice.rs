pub fn main() {
    let n = N { s: 4 };
    let s = genericsample(n);
    println!("{:?}", s)
}
trait SS<T> {
    fn ss(&self) -> i32;
}
struct N<T> {
    s: T,
}

impl<T> SS<T> for N<T> {
    fn ss(&self) -> i32 {
        return self.s * self.s;
    }
}

// whereは戻り値の後ろにつける
fn genericsample<T, X>(argx: X) -> i32
where
    X: SS<T>,
{
    return argx.ss();
}

fn koukai() {
    // クロージャーは関数の外側にある変数を補足することができる
    let n = 4;
    let l = |x: i32| x * n;
    fn koukai2(f: Fn) {
        let ls = f();
        println!("{}", ls);
    }
    koukai2(l)
}
