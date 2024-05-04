use std::collections::HashMap;

fn main() {
    let s = "This is a pen. That is a pencil.";
    let v = s.chars().collect::<Vec<char>>();
    v.iter().map(|x| {
        let mut hashmap: HashMap<char, i32> = HashMap::new();
        let mut prev = hashmap.get(x).unwrap_or(&0);

        hashmap.insert(*x, *prev + 1);
    });
    println!("{:?}", v);
}
