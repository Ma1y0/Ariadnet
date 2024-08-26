use echo::{kv, packet::Packet};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = std::env::var("ECHO_PORT").unwrap_or("53".to_string());
    let mut store = kv::Store::load().await?;

    server(format!("127.0.0.1:{port}").as_str()).await
}

async fn server(addr: &str) -> anyhow::Result<()> {
    let socket = UdpSocket::bind(addr).await?;

    loop {
        let mut buf = [0u8; 1024];

        let (len, _) = socket.recv_from(&mut buf).await?;
        println!("{:?}", &buf[..len]);
        let packet = Packet::try_from(&buf[..len])?;
        println!("{:?}", packet);
    }
}
