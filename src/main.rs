use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let addr = "localhost:8000";

    let listener = TcpListener::bind(addr)
        .await
        .expect("Unable to open TCP listener on localhost:8000.");

    println!("Listening on {addr}.");

    let (mut socket, addr) = listener
        .accept()
        .await
        .expect("Error accepting connection.");

    let (read_socket_half, mut write_socket_half) = socket.split();

    println!(
        "Opened connection with {} on port {}.",
        addr.ip(),
        addr.port()
    );

    let mut reader = BufReader::new(read_socket_half);
    let mut line = String::new();

    loop {
        let bytes_read = reader
            .read_line(&mut line)
            .await
            .expect("Error reading data.");

        if bytes_read == 0 {
            println!("Closed connection.");
            break;
        }

        write_socket_half
            .write_all(line.as_bytes())
            .await
            .expect("Error echoing data.");
    }
}
