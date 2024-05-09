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
mod q5;
// mod q6;
mod abc086_b;
mod abc352_a;
mod q7;
mod q8;
mod read_json;
mod tokiotest;

#[macro_use(macro_study)]
fn main() {
    println!("Hello, world!");

    // read_json::main();
    gacha::gacha();
    // ahc::main();
    // q6::main();
    q7::main();
    q8::main();
    abc352_a::main();
    abc086_b::main();
    // q1::main();
    bar!(1 + 1);
    macro_study::main();
    // new_person!("a".to_string(), 32);

    let l = vec!["as".to_string(), "bf".to_string(), "ccd".to_string()];
    println!("capitalize_first_letter");
    capitalize_first_letter::capitalize_first_letter(l);
    // println!("{:?}", res);
    // tokiotest::server();
    // hypertest::main();
    // q3::main();
    // q4::main();
}
