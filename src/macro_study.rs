macro_rules! foo {
    () => {};
}
// ↑が基本形
#[macro_export]
macro_rules! bar {
    ($x:expr) => {
        println!("input macro: {:?}", $x);
    };
}
