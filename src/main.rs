use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind 8000 port");

    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    return run(listener)?.await;
}