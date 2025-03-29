use std::net::TcpListener;
use cache::Cache;
use server::{server, AppState};
use links_service::Client;
use tokio::select;

mod cache;
mod error;
mod server;
mod tracing;

type Result<T> = std::result::Result<T, error::Error>;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cache = Cache::from_connection_string("redis://localhost:6379")?;
    let client = Client::create("http://localhost:8080/api").unwrap();
    let app_state = AppState { cache, client };
    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();
    let server = server(listener, app_state)?;

    select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down...");
        }
        _ = tokio::spawn(server) => {
            println!("Server stopped");
        }
    }

    Ok(())
}
