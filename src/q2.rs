pub fn q1(v: Vec<i32>) -> i32 {
    let s = v.iter().filter(|x| x.is_positive()).sum();
    if v.is_empty() {
        return 0;
    } else {
        return s;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q1() {
        let v = vec![3; 4];
        assert_eq!(q1(v), 12);
        assert_eq!(q1(Vec::new()), 0);
        let v2 = vec![3, -4, -6, 2];
        assert_eq!(q1(v2), 5)
    }
}
