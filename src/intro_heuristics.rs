use proconio::input;

pub fn main() {
    input! {
        d: usize,
        c: [usize; 26],
        s: [[usize;26]; d],
        i: [usize;d],
        // i1: i32,
        // i2: i32,
        // i3: i32,
        // i4: i32,
        // i5: i32,
    }
    println!("i = {:?}", i);
    let mut calc = 0;
    for num in 0..d {
        calc += s[num][i[num] - 1];
        println!("{}", calc);
        let mut s: usize = 0;
        for satisfy in 0..26 {
            if satisfy == i[num] - 1 {
                continue;
            }
            s += c[satisfy] * (num + 1);
            // println!(
            //     "satisfy {:?}, c[satisfy] = {:?}, d = {:?}, calc-s= {:?}",
            //     satisfy,
            //     c[satisfy],
            //     d,
            //     calc - s,
            // )
        }
        println!("satisfy = {}, s = {}", calc - s, s);
    }
    // println!("");
}
// https://img.atcoder.jp/intro-heuristics/editorial.pdf
