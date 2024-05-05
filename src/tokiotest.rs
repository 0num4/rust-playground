use std::error::Error;

use tokio::{
    io::AsyncReadExt,
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
#[allow(dead_code)]
pub async fn tcpconnect() -> Result<(), Box<dyn Error>> {
    println!("tcpconnect()");
    let mut t = TcpStream::connect("example.com:80").await?;
    // tはdisplayを持たないのでprintできない
    let request = b"GET HTTP1.1 example.com";
    Ok(())
}
#[tokio::main]
pub async fn server() -> Result<(), Box<dyn Error>> {
    let l = TcpListener::bind("localhost:8658").await?;
    println!("started localhost:8658");
    loop {
        let mut socket = l.accept().await?;
        println!("new request!!");
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let n = socket.0.read(&mut buf).await.unwrap();
                if n == 0 {
                    println!("Client disconnected");
                    break;
                }
                socket.0.write_all(&buf[0..n]).await.unwrap();
            }
        });
    }
    Ok(())
}
