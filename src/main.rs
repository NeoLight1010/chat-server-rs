use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let addr = "localhost:8000";

    let listener = TcpListener::bind(addr)
        .await
        .expect("Unable to open TCP listener on localhost:8000.");

    let (tx, _) = broadcast::channel::<(String, SocketAddr)>(8);

    println!("Listening on {addr}.");

    loop {
        let (mut socket, addr) = listener
            .accept()
            .await
            .expect("Error accepting connection.");

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (read_socket_half, mut write_socket_half) = socket.split();

            println!(
                "Opened connection with {} on port {}.",
                addr.ip(),
                addr.port()
            );

            let mut reader = BufReader::new(read_socket_half);
            let mut read_line = String::new();

            loop {
                tokio::select! {
                    read_result = reader.read_line(&mut read_line) => {
                        let bytes_read = read_result.expect("Error reading data.");

                        if bytes_read == 0 {
                            println!("Closed connection with {} on port {}.", addr.ip(), addr.port());
                            break;
                        }

                        tx.send((read_line.clone(), addr)).expect("Error broadcasting data.");

                        read_line.clear();
                    }

                    msg_result = rx.recv() => {
                        let (msg, sender_addr) = msg_result.expect("Error receiving broadcast data.");

                        match sender_addr == addr {
                            true => {}
                            false => {
                                write_socket_half
                                    .write_all(msg.as_bytes())
                                    .await
                                    .expect("Error echoing data.");
                            }
                        }

                    }
                }
            }
        });
    }
}
