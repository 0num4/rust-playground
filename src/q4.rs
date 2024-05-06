// struct AsyncProcessor<T> {
//     data: i32,
// }

// impl AsyncProcessor<T> {
//     fn process_data(f) -> Vec<Result<T, E>>{

//     }
// }

use std::{
    collections::VecDeque,
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
    // q4_pre();
}

// fn q4_pre() {
//     let shared_int = Arc::new(Mutex::new(32));
//     let mut thread_dequeue = VecDeque::new();
//     for _ in 1..10 {
//         let t = std::thread::spawn(move || {
//             let clone_shared_int = Arc::clone(&shared_int);
//             let mut locked_shared_int = clone_shared_int.lock().unwrap();
//             println!("{}", locked_shared_int);
//             for _ in 1..1000 {
//                 *locked_shared_int = *locked_shared_int + 1;
//             }
//             println!("locked_shared_int: {}", *locked_shared_int);
//             println!("5s待ちます");
//             std::thread::sleep(std::time::Duration::from_secs(5));
//             println!("5s待ちました");
//         });
//         thread_dequeue.push_back(t);
//         // let t_res = t.await;
//         // println!("{:?}", t_res)
//     }

//     while thread_dequeue.len() == 0 {
//         let t = thread_dequeue.pop_back();
//         t.unwrap().join();
//     }
// }
