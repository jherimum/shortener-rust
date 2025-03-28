use std::net::TcpListener;
use tokio::select;

mod generator;
mod server;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("addr").unwrap();
    let server = server::server(listener);

    select! {
        _ = tokio::spawn(server) => {
            println!("Server stopped");
        }
    }
}
