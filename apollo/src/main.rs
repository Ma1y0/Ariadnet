use aethon::{Method, Request};
use tokio::{
    io::{self, AsyncReadExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8081")
        .await
        .expect("Failed to bind the listener");

    let mut i = 0u32;

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buffer = [0u8; 1204];
        let n = socket.read(&mut buffer).await?;
        Request::try_from(&buffer[..n]).unwrap();
        i += 1;
        println!("{i}");
    }
}
