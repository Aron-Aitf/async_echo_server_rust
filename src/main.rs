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
    clippy::print_stdout,
    clippy::allow_attributes
)]

use tokio::{
    io::{AsyncReadExt as _, AsyncWriteExt as _},
    net::{TcpListener, TcpStream},
    spawn,
};

const HOST: &str = "localhost";
const PORT: &str = "8000";
const PORT_SEPARATOR: &str = ":";
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let tcp_url = format!("{HOST}{PORT_SEPARATOR}{PORT}");
    let listener = TcpListener::bind(tcp_url).await.unwrap();
    loop {
        #[allow(unused_mut)]
        let (mut socket, _ip) = listener.accept().await.unwrap();
        spawn(handle_connection(socket));
    }
}

async fn handle_connection(mut socket: TcpStream) {
    loop {
        let mut buffer = [0; 1024];
        match socket.read(&mut buffer).await {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }
        println!("5");
        socket.write_all(&buffer).await.unwrap();
        println!("6");
    }
}
