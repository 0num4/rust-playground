use proconio::input;

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
        $(println!("$x {:?}", $x);)*
    };
}

macro_rules! testmacro2 {
    ($($x:expr),*) => {
        $(println!("$x {:?}", $x);)*
    };
}

// macro_rules! print_vec {
//     ($x:expr) => {
//         match $x {
//             vec![$()] => {

//             }
//         }
//     };
// }

pub fn main() {
    input! {
        (inp1, inp2): (i32,i32)
    }
    if (inp1 * inp2) % 2 == 1 {
        println!("Odd")
    } else {
        println!("Even")
    }
    println!("{:?}", inp1);
    println!("{:?}", inp2);
    new_person!("a".to_string(), 32);
    test!(1 + 1, 3, 5, 3);
    testmacro2!(1 + 1, 3, 5, 3);
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
