use std::io::Error;

pub fn q1_1() {
    let mut v = vec![1; 4];
    v.push(4);
    print!("{:?}", v);
    // print!("{}", v[5]) 存在しない配列にアクセスすると何も表示されなかった
    let mut strVec = vec!["hello", "world"];
    print!("{:?}", strVec);
    strVec.push("nya");
    print!("{:?}", strVec);
    strVec.pop(); // 末尾から消す
    print!("{:?}", strVec);
    strVec.push("nya");
    strVec.push("nya");
    strVec.remove(1);
    print!("{:?}", strVec);
    for i in strVec {
        println!("{:?}", i);
    }
}

fn q1_3_option() -> Option<i32> {
    let v = vec![1, 2, 3, 4, 56];
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

pub fn main() {
    q1_1();
    let ans = q1_3_option();
    // println!("ans is {}", ans.unwrap());
    q1_3_match();
    q1_3_expect();
    let a = q1_4_esult(32);
}
