use std::net::TcpStream;

mod gacha;

fn main() {
    println!("Hello, world!");
    gacha::gacha();

    TcpStream::connect("example.com:80").await?
}
