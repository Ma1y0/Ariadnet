use tokio::{
    io::{self, AsyncReadExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpListener::bind("127.0.0.1:8080").await?;

    let (mut strm, _) = socket.accept().await?;
    let mut buffer = [0u8; 1024];
    let n = strm.read(&mut buffer).await?;
    println!("{:?}", &buffer[..n]);
    println!("{}", String::from_utf8_lossy(&buffer[..n]));
    Ok(())
}
