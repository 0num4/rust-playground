mod gacha;
mod hypertest;
mod tokiotest;

fn main() {
    println!("Hello, world!");
    gacha::gacha();
    // tokiotest::server();
    hypertest::main();
}
