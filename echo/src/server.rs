use crate::{
    kv::Store,
    packet::{self, Method, Packet},
};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::UdpSocket;
use tracing::{error, info};

/// Start the server
// pub async fn serve(socket: UdpSocket, store: &Store) -> anyhow::Result<()> {
//     loop {
//         let mut buf = [0u8; 1024];
//
//         let (len, source) = socket.recv_from(&mut buf).await?;
//         info!("Recived {} bytes from {}", len, source);
//         let packet = Packet::try_from(&buf[..len])?;
//         handle(&packet, &socket, &source, store).await;
//     }
// }
pub async fn serve(socket: UdpSocket, store: Store) -> anyhow::Result<()> {
    let socket = Arc::new(socket);
    let store = Arc::new(store);

    loop {
        let mut buf = [0u8; 1024];

        let store = Arc::clone(&store);
        let socket = Arc::clone(&socket);

        let (len, source) = socket.recv_from(&mut buf).await?;
        info!("Received {} bytes from {}", len, source);
        tokio::spawn(async move {
            let packet = match Packet::try_from(&buf[..len]) {
                Ok(packet) => packet,
                Err(e) => {
                    error!("Failed to parse packet: {:?}, error: {e}", &buf[..len]);
                    return;
                }
            };
            handle(&packet, socket.as_ref(), &source, store.as_ref()).await;
        });
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
