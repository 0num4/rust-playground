use proconio::input;

pub fn main() {
    input! {
        a: String,
        b: String
    }
    let mut ansvec: Vec<usize> = Vec::new();
    let mut idx: usize = 0;
    let acs: Vec<char> = a.chars().collect();
    for (i, c) in b.chars().enumerate() {
        if c == acs[idx] {
            ansvec.push(i + 1);
            idx += 1;
        }
    }
    // ansvec.sort();
    for num in ansvec.iter() {
        print!("{} ", num);
    }
    println!("");
}

fn b() {
    input! {
        a: String,
        b: String
    }
    let mut ansvec: Vec<i32> = Vec::new();
    let mut idx: i32 = 0;
    for (i, c) in b.chars().enumerate() {
        if c == a.chars().nth(idx.try_into().unwrap()).unwrap() {
            ansvec.push((i + 1).try_into().unwrap());
            idx += 1;
        }
    }
    ansvec.sort();
    for num in ansvec.iter() {
        print!("{} ", num);
    }
    println!("");
}
