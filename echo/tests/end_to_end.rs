use echo::kv::Store;
use echo::packet::Packet;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::sync::oneshot;

async fn spawn_server(store: Store) -> (SocketAddr, oneshot::Sender<()>) {
    let socket = UdpSocket::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind server socket");
    let addr = socket.local_addr().expect("Failed to get socket's address");

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        tokio::select! {
            _ = echo::server::serve(socket, store) => {
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
async fn test_resolve_query_not_found() {
    let store = Store::in_memory();
    let (addr, shutdown_tx) = spawn_server(store).await;

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

    // Tests
    assert_eq!(expected, ans);

    // Shutdown the server
    shutdown_tx
        .send(())
        .expect("Failed to send shutdown signal");
}

#[tokio::test]
async fn test_resolve_query() {
    let mut store = Store::in_memory();
    store.insert("a", "a");
    let (addr, shutdown_tx) = spawn_server(store).await;

    // Create UDP client
    let socket = UdpSocket::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind the UDP socket");

    // Packet
    let packet: Vec<u8> = Packet::new(
        1,
        echo::packet::Method::Query,
        echo::packet::Error::None,
        "a",
    )
    .into();

    // Send packet
    socket
        .send_to(&packet, addr)
        .await
        .expect("Failed to send query");

    // Read response
    let mut buf = [0u8; 1024];
    let n = socket
        .recv(&mut buf)
        .await
        .expect("Failed to receive answer");

    // Parse the answer
    let answer = Packet::try_from(&buf[..n]).expect("Failed to parse the answer");

    // Tests
    assert_eq!("a", answer.body);

    // Shutdown the server
    shutdown_tx
        .send(())
        .expect("Faile to send shutdown message to the server");
}
