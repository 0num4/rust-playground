use std::collections::HashMap;

pub fn main() {
    let s = "This is a pen. That is a pencil.";
    let v = s.chars().collect::<Vec<char>>();
    let mut hashmap: HashMap<char, i32> = HashMap::new();
    v.iter()
        .map(|x| {
            let mut prev = hashmap.get(x).unwrap_or(&0);
            if (*x).is_ascii_alphabetic() {
                hashmap.insert(*x, *prev + 1);
            }
        })
        .collect::<Vec<_>>();
    let max = hashmap.iter().max_by_key(|x| x.1).unwrap();
    println!("hashmap {:?}", hashmap);
    println!("値が一番大きいもの: {:?}", max.0)
}
