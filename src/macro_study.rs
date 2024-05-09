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

macro_rules! new_person {
    ($x:expr, $y:expr) => {
        Person { name: $x, age: $y }
    };
}

pub struct Person {
    name: String,
    age: i32,
}

pub fn main() {
    new_person!("a".to_string(), 32);
}
