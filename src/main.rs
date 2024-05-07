use std::{
    sync::{Arc, Mutex},
    thread,
};

mod capitalize_first_letter;
mod gacha;
mod hypertest;
mod q1;
mod q2;
mod q3;
mod q4;
mod tokiotest;

fn main() {
    println!("Hello, world!");

    gacha::gacha();
    q1::main();
    let l = vec!["as".to_string(), "bf".to_string(), "ccd".to_string()];
    println!("capitalize_first_letter");
    capitalize_first_letter::capitalize_first_letter(l);
    // println!("{:?}", res);
    // tokiotest::server();
    // hypertest::main();
    // q3::main();
    // q4::main();
}
