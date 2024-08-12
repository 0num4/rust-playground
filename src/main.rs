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
use models::Post;
use rust_playground::*;
mod abc362;
mod cargo_toml_parse;
mod generic_where_plactice;
mod rust_study20240812;
mod rust_study20240812_anyhow;

fn main() {
    generic_where_plactice::main();
    // cargo_toml_parse::main();
    panic!("end");
    abc362::main();
    use self::schema::posts::dsl::*;
    let connection = &mut establish_connection();
    let res = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");
    for post in res {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
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
