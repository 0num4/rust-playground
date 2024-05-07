// struct AsyncProcessor<T> {
//     data: i32,
// }

// impl AsyncProcessor<T> {
//     fn process_data(f) -> Vec<Result<T, E>>{

//     }
// }

use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
    sync::{Arc, Mutex},
    thread::{self, spawn},
};

pub fn main() {
    // let a = AsyncProcessor::<i32>{
    //     data:
    // };
    let mut arcTest = Arc::new(42);
    let arcMutexTest = Arc::new(Mutex::new(43));
    let mut arcMutexTestInner = arcMutexTest.lock().unwrap();
    println!("{}", arcTest);
    println!("{}", arcMutexTestInner);
    // *arcTest = 444;
    *arcMutexTestInner = 444;
    println!("{}", arcTest);
    println!("{}", arcMutexTestInner);
    // q4_pre();
    q5_more_thread()
}

fn q4_pre() {
    let shared_int = Arc::new(Mutex::new(32));
    let mut thread_dequeue = VecDeque::new();
    for _ in 1..10 {
        let clone_shared_int = Arc::clone(&shared_int);
        let t = std::thread::spawn(move || {
            let mut locked_shared_int = clone_shared_int.lock().unwrap();
            println!("{}", locked_shared_int);
            for _ in 1..1000 {
                *locked_shared_int = *locked_shared_int + 1;
            }
            println!("locked_shared_int: {}", *locked_shared_int);
            // println!("5s待ちます");
            // std::thread::sleep(std::time::Duration::from_secs(5));
            // println!("5s待ちました");
        });
        thread_dequeue.push_back(t);
        // let t_res = t.await;
        // println!("{:?}", t_res)
    }

    while thread_dequeue.len() == 0 {
        let t = thread_dequeue.pop_front();
        let _ = t.unwrap().join();
    }
}

// 問題3：マルチスレッドでのファイル読み込みと処理

// 大きなテキストファイルを複数のスレッドで分割して読み込む。
// 各スレッドは、割り当てられた部分のファイルを読み込み、単語の出現回数をカウントする。
// 全てのスレッドが終了した後、単語の出現回数を集計し、上位10個の単語を出力する。
// ヒント：std::fs::Fileとstd::io::BufReaderを使用してファイルを読み込む。Arc<Mutex<HashMap<String, usize>>>を使用して、単語の出現回数を安全に集計する。
fn q5_more_thread() {
    let mut str = String::new();
    let mut countMap: Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut h = HashMap::new();
    // h.insert(k, v)
    let mut f = File::open("memo.md").unwrap(); // file::openはファイルハンドラだから100TB読むわけではない
    let f_size = f.metadata().unwrap().len();
    println!("f_size: {}", f_size);
    for i in 0..5 {
        let countMap = Arc::clone(&countMap);
        let f = f.try_clone().unwrap();
        let t = spawn(move || {
            let mut locked_count_map = countMap.lock().unwrap();
            let mut reader = BufReader::new(f);
            let mut b = String::new();
            let seek_start = (f_size / 5) * i;
            println!("{}個目のthreads。{} byte目から読み取ります", i, seek_start);
            reader.seek(SeekFrom::Start(seek_start)).unwrap();
            let l = reader.read_line(&mut b).unwrap();
            println!("buffer; {}", b);
            println!("stream_position; {}", reader.stream_position().unwrap());
            let words = b.split_ascii_whitespace();
            for word in words {
                let v = locked_count_map.get(word).unwrap();
                let e = locked_count_map.entry("a".to_string()).or_insert(3);
            }
        });
        t.join().unwrap();
    }
}
