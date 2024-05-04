use std::str::Chars;

pub fn capitalize_first_letter(vecStr: Vec<String>) {
    let vs: Vec<String> = vecStr
        .iter()
        .map(|s| {
            let mut svec: Chars = s.chars();
            svec.next().unwrap().to_ascii_uppercase().to_string() + &svec.collect();
        })
        .collect();
    println!("{:?}", vs);
}
