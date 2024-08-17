use proconio::input;
pub fn main() {
    input! {
        n: i32,
        w: [i32; n]
    }
    // a();
    b(n, &w);
}

fn a() {
    input! {}
}

// 累積和の練習
// https://atcoder.jp/contests/abc129/tasks/abc129_b
fn b(n: i32, w: &[i32]) -> i32 {
    println!("{:?}", w);
    let mut sum = 1000;
    for wi in 0..n {
        let mut prev = 0;
        for wj in 0..wi {
            prev += w[wj as usize];
        }
        let mut end = 0;
        for wk in wi..n {
            end += w[wk as usize];
        }
        println!("{:?}", prev);
        println!("{:?}", end);
        println!("sub {:?}", prev - end);
        if (prev - end).abs() < sum {
            sum = (prev - end).abs();
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn test_b() {
    //     assert_eq!(super::b(3, &[1, 2, 3]), 0);
    //     assert_eq!(super::b(4, &[1, 3, 1, 1]), 2);
    //     // 27 23 76 2 3 5 62 52
    //     assert_eq!(super::b(8, &[27, 23, 76, 2, 3, 5, 62, 52]), 2);
    // }
    #[test]
    fn test_b_1() {
        assert_eq!(super::b(3, &[1, 2, 3]), 0);
    }
    #[test]
    fn test_b_2() {
        assert_eq!(super::b(4, &[1, 3, 1, 1]), 2);
    }
    #[test]
    fn test_b_3() {
        // 27 23 76 2 3 5 62 52
        assert_eq!(super::b(8, &[27, 23, 76, 2, 3, 5, 62, 52]), 2);
    }
}
