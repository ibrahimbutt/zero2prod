use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run(bind_port())?.await
}

fn bind_port() -> TcpListener {
    return TcpListener::bind("127.0.0.1:8080").expect("Failed to bind random port");
}
