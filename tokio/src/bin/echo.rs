use tokio::net::TcpListener;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(socket_res) = incoming.next().await {
            match socket_res {
                Ok(mut socket) => {
                    println!("Accepted connection from {:?}", socket.peer_addr());
                    tokio::spawn(async move {
                        let (mut reader, mut writer) = socket.split();

                        match tokio::io::copy(&mut reader, &mut writer).await {
                            Ok(amt) => println!("wrote {} bytes", amt),
                            Err(err) => eprintln!("IO error {:?}", err),
                        }
                    });
                }
                Err(err) => eprintln!("Accept error = {:?}", err)
            }
        }
    };

    println!("Listening on {}", addr);
    server.await;
}
