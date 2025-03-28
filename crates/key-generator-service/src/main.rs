use std::{net::TcpListener, sync::Arc};
use generator::postgres::PostgresKeyGenerator;
use server::AppState;
use tokio::select;

mod generator;
mod server;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("addr").unwrap();
    let generator = Arc::new(PostgresKeyGenerator::from_connection_string(""));
    let state = AppState { generator };
    let server = server::server(listener, state);

    select! {
        _ = tokio::spawn(server) => {
            println!("Server stopped");
        }
    }
}
