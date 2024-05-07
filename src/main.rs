use std::{
    sync::{Arc, Mutex},
    thread,
};

mod ahc;
mod capitalize_first_letter;
mod gacha;
mod hypertest;
mod macro_study;
mod q1;
mod q2;
mod q3;
mod q4;
mod read_json;
mod tokiotest;

fn main() {
    println!("Hello, world!");

    gacha::gacha();
    ahc::main();
    q1::main();
    bar!(1 + 1);

    let l = vec!["as".to_string(), "bf".to_string(), "ccd".to_string()];
    println!("capitalize_first_letter");
    capitalize_first_letter::capitalize_first_letter(l);
    // println!("{:?}", res);
    // tokiotest::server();
    // hypertest::main();
    // q3::main();
    // q4::main();
}
