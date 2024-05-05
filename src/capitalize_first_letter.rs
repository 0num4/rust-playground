use std::str::Chars;

pub fn capitalize_first_letter(vec_str: Vec<String>) {
    let vs: Vec<String> = vec_str
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
