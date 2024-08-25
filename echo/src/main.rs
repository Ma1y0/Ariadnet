use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:53").await?;

    loop {
        let mut buf = [0; 1024];

        let (len, src) = socket.recv_from(&mut buf).await?;
        println!("{:?}", buf);
        println!("Recived {len} bytes from {src}");

        let a = socket.send_to(&buf[..len], src).await?;
        println!("Send {a} bytes");
    }
}
