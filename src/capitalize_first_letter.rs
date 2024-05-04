use std::str::Chars;

pub fn capitalize_first_letter(vecStr: Vec<String>) {
    let vs: Vec<String> = vecStr
        .iter()
        .map(|s| {
            let mut svec: Chars = s.chars();
            match svec.next() {
                None => String::new(),
                Some(c) => c.to_ascii_uppercase().to_string(),
            }
        })
        .collect();

    println!("{:?}", vs);
}
