// struct AsyncProcessor<T> {
//     data: i32,
// }

// impl AsyncProcessor<T> {
//     fn process_data(f) -> Vec<Result<T, E>>{

//     }
// }

use std::sync::{Arc, Mutex};

pub fn main() {
    // let a = AsyncProcessor::<i32>{
    //     data:
    // };
    let mut arcTest = Arc::new(42);
    let arcMutexTest = Arc::new(Mutex::new(43));
    let mut arcMutexTestInner = arcMutexTest.lock().unwrap();
    println!("{}", arcTest);
    println!("{}", arcMutexTest.lock().unwrap());
    // *arcTest = 444;
    *arcMutexTestInner = 444;
    println!("{}", arcTest);
    println!("{}", arcMutexTestInner);
}

pub fn q4_pre() {}