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

macro_rules! test {
    ($($x:expr),*) => {
        $(println!("$x {:?}", $x))*
    };
}

pub fn main() {
    new_person!("a".to_string(), 32);
    test!(1 + 1);
}

// macro_rules! fibonacci {
//     ($x:expr) => {
//         match $expr {
//                                             vec![$expr]
//                                         }
//         if $x > 2 {
//         } else {
//             fibonacci!($x - 1) + fibonacci!($x - 2)
//         }
//     };
// }
