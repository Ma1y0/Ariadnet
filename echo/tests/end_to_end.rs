use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};

async fn spawn_server() -> (SocketAddr, oneshot::Sender<()>) {
    let socket = UdpSocket::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind server socket");
    let addr = socket.local_addr().expect("Failed to get socket's address");

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        tokio::select! {
            _ = echo::server::serve(socket) => {
                println!("Server stopped");
            }
            _ = shutdown_rx => {
                println!("Server received shutdown signal");
            }
        }
    });

    (addr, shutdown_tx)
}

#[tokio::test]
async fn test_resolve_query() {
    let (addr, shutdown_tx) = spawn_server().await;

    // Give the server some time to start up
    sleep(Duration::from_millis(100)).await;

    let socket = UdpSocket::bind("127.0.0.0:0")
        .await
        .expect("Failed to bind client socket");

    let message: Vec<u8> = echo::packet::Packet::new(
        1,
        echo::packet::Method::Query,
        echo::packet::Error::None,
        "Hi",
    )
    .into();

    // Send Query
    let _ = socket
        .send_to(&message, addr)
        .await
        .expect("Failed to send message");

    // Receives Answer
    let mut buf = [0u8; 1024];
    let n = socket
        .recv(&mut buf)
        .await
        .expect("Failed to receive message");

    // Parser the packet
    let ans = echo::packet::Packet::try_from(&buf[..n]).expect("Failed to parse received packet");
    let expected = echo::packet::Packet::new(
        1,
        echo::packet::Method::Answer,
        echo::packet::Error::NotFound,
        "",
    );

    assert_eq!(expected, ans);

    // Shutdown the server
    shutdown_tx
        .send(())
        .expect("Failed to send shutdown signal");
}
