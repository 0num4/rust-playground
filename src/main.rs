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
mod abc076;
mod abc086_b;
mod abc351_a;
mod abc352_a;
mod abc353;
mod atcode_beginers_selection;
mod dieselsample;
mod intro_heuristics;
mod q7;
mod q8;
mod read_json;
mod rust_study_0510;
mod tokiotest;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
pub mod models;
pub mod schema;
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}

fn main() {
    // dieselsample::main();
    println!("Hello, world!");
    abc353::main();
    // read_json::main();
    gacha::gacha();
    // ahc::main();
    // q6::main();
    q7::main();
    q8::main();
    // rust_study_0510::main();
    atcode_beginers_selection::main();
    intro_heuristics::main();

    abc076::b();
    abc351_a::main();
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
