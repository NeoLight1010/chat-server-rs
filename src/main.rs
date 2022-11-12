use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let addr = "localhost:8000";

    let listener = TcpListener::bind(addr)
        .await
        .expect("Unable to open TCP listener on localhost:8000.");

    println!("Listening on {addr}");

    let (mut socket, addr) = listener
        .accept()
        .await
        .expect("Error accepting connection.");

    println!(
        "Opened connection with {} on port {}.",
        addr.ip(),
        addr.port()
    );

    loop {
        let mut buffer = [0u8; 1024];

        let bytes_read = socket.read(&mut buffer).await.unwrap();

        socket
            .write_all(&buffer[..bytes_read])
            .await
            .expect("Error echoing data.");
    }
}
