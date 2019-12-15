use tokio::net::TcpStream;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    stream.write_all(b"hello world\n").await.unwrap();
    println!("wrote to stream");
}
