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
    let mut arcMutexTest = Arc::new(Mutex::new(43));
    println!("{}", arcTest);
    println!("{}", arcMutexTest.lock().unwrap());
}
