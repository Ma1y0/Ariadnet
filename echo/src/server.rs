use crate::packet::Packet;
use tokio::net::UdpSocket;

/// Start the server
pub async fn serve(socket: UdpSocket) -> anyhow::Result<()> {
    loop {
        let mut buf = [0u8; 1024];

        let (len, source) = socket.recv_from(&mut buf).await?;
        println!("{:?}", &buf[..len]);
        let packet = Packet::try_from(&buf[..len])?;
        println!("{:?}", packet);

        let _n = socket.send_to(&[1, 0, 0], source).await?;
    }
}
