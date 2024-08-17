use std::collections::HashMap;

use proconio::input;
pub fn main() {
    d();
    // println!("nyaaa");
    // b();
    // c();
}
// pub fn a() {
//     // 入力は全て整数
//     input! {
//         (a, b, c): (i32, i32, i32),
//     }
//     // 24時間表記
//     let bb = b - 24;
//     let cc = c - 24;
//     // b時に寝る
//     // c時に起きる
//     println!("{}", c);
//     // if b <= c && a > c {
//     //     println!("Yes");
//     // }
//     // if c <= b && a > b {
//     //     println!("No");
//     // }
//     if cc > bb {
//         // 通常。朝7時に起きて21時に寝たケース
//         if a >
//         println!("Yes");
//     } else {
//         println!("No");
//     }
// }

// pub fn b() {
//     input! {
//         b: str,
//     }
// }

pub fn c() {
    input! {
        // 長さn
        // 総和がkの倍数
        (n, k): (i32, i32),
        r: [i32; n],
    }
    let mut v: Vec<i32> = vec![];
    for i in 0..n {
        v.push(r[i as usize]);
    }
}

fn solve_lake_rest_area_problem(n: usize, m: i64, a: &[i64]) -> i64 {
    let mut cumsum = vec![0; n + 1];
    for i in 0..n {
        cumsum[i + 1] = cumsum[i] + a[i];
    }

    let total_steps = cumsum[n];
    let mut count = 0;

    for s in 1..=n {
        for t in 1..=n {
            if s != t {
                let steps = if s < t {
                    cumsum[t - 1] - cumsum[s - 1]
                } else {
                    total_steps - (cumsum[s - 1] - cumsum[t - 1])
                };

                if steps % m == 0 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn solve_lake_rest_area_problem2(n: usize, m: i64, a: &[i64]) -> i64 {
    let mut cumsum = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        cumsum[i + 1] = (cumsum[i] + a[i % n]) % m;
    }

    let mut count = vec![0; m as usize];
    let mut result = 0;

    for i in 0..n {
        count.fill(0);
        count[0] = 1; // 初期値を1にセット
        for j in i + 1..i + n + 1 {
            let sum = cumsum[j] as usize;
            result += count[sum];
            count[sum] += 1;
        }
    }

    result
}

fn solve_lake_rest_area_problem3(n: usize, m: i64, a: &[i64]) -> i64 {
    let mut cumsum = vec![0; n + 1];
    for i in 0..n {
        cumsum[i + 1] = (cumsum[i] + a[i]) % m;
    }

    let mut count = 0;
    for s in 0..n {
        for t in s + 1..=n {
            if (cumsum[t] - cumsum[s] + m) % m == 0 {
                count += 1;
            }
        }
    }

    count
}

fn solve_lake_rest_area_problem4(n: usize, m: i64, a: &[i64]) -> i64 {
    let mut cumsum = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        cumsum[i + 1] = (cumsum[i] + a[i % n]) % m;
    }

    let mut result = 0;
    for i in 0..n {
        for j in i + 1..i + n + 1 {
            if (cumsum[j] - cumsum[i] + m) % m == 0 {
                result += 1;
            }
        }
    }

    result
}
fn solve_lake_rest_area_problem5(n: usize, m: i64, a: &[i64]) -> i64 {
    let mut cumsum = 0;
    let mut count_map = HashMap::new();
    count_map.insert(0, 1); // 初期状態で0が1回あるとする
    let mut result = 0;

    for i in 0..2 * n {
        cumsum = (cumsum + a[i % n]) % m;
        // 正規化は必要か再確認
        if cumsum < 0 {
            cumsum += m;
        }
        if let Some(&count) = count_map.get(&cumsum) {
            result += count;
        }
        *count_map.entry(cumsum).or_insert(0) += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_lake_rest_area_problem4() {
        assert_eq!(super::solve_lake_rest_area_problem4(4, 3, &[2, 1, 4, 3]), 4);
        assert_eq!(super::solve_lake_rest_area_problem4(2, 1000000, &[1, 1]), 0);
        // 9 9 8 2 4 4 3 5 3
        assert_eq!(
            super::solve_lake_rest_area_problem4(9, 5, &[9, 9, 8, 2, 4, 4, 3, 5, 3]),
            11
        );
    }
    #[test]
    fn solve_lake_rest_area_problem5() {
        assert_eq!(super::solve_lake_rest_area_problem5(4, 3, &[2, 1, 4, 3]), 4);
        assert_eq!(super::solve_lake_rest_area_problem5(2, 1000000, &[1, 1]), 0);
        // 9 9 8 2 4 4 3 5 3
        assert_eq!(
            super::solve_lake_rest_area_problem5(9, 5, &[9, 9, 8, 2, 4, 4, 3, 5, 3]),
            11
        );
    }
}

pub fn d() {
    input! {
        n: usize,
        m: i64,
        a: [i64; n],
    }
    let answer = solve_lake_rest_area_problem5(n, m, &a);
    println!("{}", answer);
}
