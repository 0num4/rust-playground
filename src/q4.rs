// struct AsyncProcessor<T> {
//     data: i32,
// }

// impl AsyncProcessor<T> {
//     fn process_data(f) -> Vec<Result<T, E>>{

//     }
// }

use std::{
    sync::{Arc, Mutex},
    thread,
};

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

pub fn q4_pre() {
    for i in 1..10 {
        let t = std::thread::spawn(|| {
            println!("5s待ちます");
            std::thread::sleep(std::time::Duration::from_secs(5));
            println!("5s待ちました");
        });
        // let t_res = t.await;
        // println!("{:?}", t_res)
    }
}
