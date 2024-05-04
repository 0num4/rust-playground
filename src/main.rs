mod gacha;
mod hypertest;
mod q1;
mod q2;
mod tokiotest;

fn main() {
    println!("Hello, world!");
    gacha::gacha();
    q1::main();
    // tokiotest::server();
    // hypertest::main();
}
