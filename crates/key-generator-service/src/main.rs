use std::{net::TcpListener, sync::Arc};
use generator::InMemoryKeyGenerator;
use tokio::select;

mod app;
mod generator;
mod key;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("addr").unwrap();
    let generator = Arc::new(InMemoryKeyGenerator::new(7, 100));
    let server = app::app(listener, generator);

    select! {
        _ = tokio::spawn(server) => {
            println!("Server stopped");
        }
    }
}
