use std::{
    fs::File,
    io::{Error, Read},
    path::Path,
};

use rand::{thread_rng, Rng};

pub fn q1_1() {
    let mut v = vec![1; 4];
    v.push(4);
    print!("{:?}", v);
    // print!("{}", v[5]) 存在しない配列にアクセスすると何も表示されなかった
    let mut str_vec = vec!["hello", "world"];
    print!("{:?}", str_vec);
    str_vec.push("nya");
    print!("{:?}", str_vec);
    str_vec.pop(); // 末尾から消す
    print!("{:?}", str_vec);
    str_vec.push("nya");
    str_vec.push("nya");
    str_vec.remove(1);
    print!("{:?}", str_vec);
    for i in str_vec {
        println!("{:?}", i);
    }
}

fn q1_3_option() -> Option<i32> {
    let v = [1, 2, 3, 4, 56];
    let findid = 56;
    let ans = v.get(findid).copied();
    return ans;
}

// https://doc.rust-jp.rs/book-ja/ch06-02-match.html#optiont%E3%81%A8%E3%81%AE%E3%83%9E%E3%83%83%E3%83%81
fn q1_3_match() {
    let a = Some(4);
    match a {
        _ => println!("hoge"),
        None => print!("non"),
    }
}

fn q1_3_expect() {
    let f = Some(4);
    let x: Option<i32> = None; // Noneを入れる場合optionの返り値がわからないので型を明示する必要がある
    let n = x.expect("nyaaa");
    //
    print!("{}", n);
}

fn q1_4_esult(i: i32) -> Result<String, Error> {
    return Ok(i.to_string());
}

/// ファイルを開いてその中の内容を読み取る
/// # Examples
/// ```
/// let m = q1_4_fileopen("memo.md")
/// assert_eq!(m, "nya")
/// ```

fn q1_4_fileopen(path: &Path) -> Result<String, Error> {
    // let dis = path.display();
    let mut a = File::open(path)?;
    let mut buf = String::new();
    a.read_to_string(&mut buf)?;
    Ok(buf)
}

fn q1_4_match() {
    let n = File::create("sofo.txt");
    match n {
        Err(why) => print!("{:?}", why),
        Ok(_) => print!("ok"),
    }
}

pub fn main() {
    q1_1();
    let ans = q1_3_option();
    // println!("ans is {}", ans.unwrap());
    q1_3_match();
    // q1_3_expect();
    println!("q1_3_expect(); end");
    let a = q1_4_esult(32);
    let rspath = "./src/main.rs"; // カレントターミナル(src/以下ではなくプロジェクトのroot)をみる
    let res = q1_4_fileopen(Path::new(rspath)).unwrap();
    println!("{}", res);
    println!("q1_4_fileopen(); end");
    q1_4_match();
    println!("q1_4_match(); end");
    match err_handling() {
        Ok(_) => println!("err_handlingの処理は正常"),
        Err(_) => println!("err_handlingを読んだ際にerrが発生しました!!"),
    }
}

pub fn err_handling() -> Result<i32, i32> {
    let mut r = thread_rng(); //preudo RNG
    let r_num: i32 = r.gen();
    println!("random number is: {}", r_num);
    if r_num > 0 {
        Ok(r_num)
    } else {
        Err(r_num)
    }
}
