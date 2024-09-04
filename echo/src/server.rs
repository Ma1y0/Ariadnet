use crate::{
    kv::Store,
    packet::{self, Method, Packet},
};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tracing::{error, info};

/// Start the server
pub async fn serve(socket: UdpSocket, store: &Store) -> anyhow::Result<()> {
    loop {
        let mut buf = [0u8; 1024];

        let (len, source) = socket.recv_from(&mut buf).await?;
        info!("Recived {} bytes from {}", len, source);
        let packet = Packet::try_from(&buf[..len])?;
        handle(&packet, &socket, &source, store).await;
    }
}

async fn handle(packet: &Packet, socket: &UdpSocket, source: &SocketAddr, store: &Store) {
    let res = store.get(packet.body.as_str());
    let ans: Vec<u8> = match res {
        Some(a) => Packet::new(1, Method::Answer, packet::Error::None, a.to_string()),
        None => Packet::new(1, Method::Answer, packet::Error::NotFound, ""),
    }
    .into();

    let n = socket.send_to(&ans, source).await;
    match n {
        Ok(n) => info!("Send {n} bytes to {source}, packet={:?}", ans),
        Err(e) => error!(
            "Failed to send asnwer to {source}, packet={:?}, error={}",
            ans, e
        ),
    };
}
