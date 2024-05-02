mod gacha;
mod tokiotest;

fn main() {
    println!("Hello, world!");
    gacha::gacha();
    tokiotest::server();
    
}
