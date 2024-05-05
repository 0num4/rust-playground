use std::collections::HashMap;

pub fn main() {
    let s = "This is a pen. That is a pencil.";
    let v = s.chars().collect::<Vec<char>>();
    let mut hashmap: HashMap<char, i32> = HashMap::new();
    for c in v {
        if c.is_alphabetic() {
            let prev = hashmap.get(&c).unwrap_or(&0);
            hashmap.insert(c, prev + 1);
        }
    }
    let max = hashmap.iter().max_by_key(|x| x.1).unwrap();
    println!("hashmap {:?}", hashmap);
    println!("値が一番大きいもの: {:?}", max.0)
}
