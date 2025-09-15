#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::cargo_common_metadata,
    clippy::blanket_clippy_restriction_lints,
    clippy::unwrap_used,
    clippy::single_call_fn,
    clippy::allow_attributes_without_reason,
    clippy::infinite_loop,
    clippy::allow_attributes
)]

use tokio::{
    io::{AsyncReadExt as _, AsyncWriteExt as _},
    net::{TcpListener, TcpStream},
    spawn,
};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let listener = TcpListener::bind("localhost:8000").await.unwrap();
    loop {
        #[allow(unused_mut)]
        let (mut socket, _ip) = listener.accept().await.unwrap();
        spawn(handle_connection(socket));
    }
}

async fn handle_connection(mut socket: TcpStream) {
    loop {
        let mut buffer = String::new();
        match socket.read_to_string(&mut buffer).await {
            Ok(0) | Err(_) => break,
            Ok(n) => n,
        };
        socket.write_all(buffer.as_bytes()).await.unwrap();
    }
}
