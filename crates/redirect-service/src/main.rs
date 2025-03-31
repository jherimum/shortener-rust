use std::net::TcpListener;
use cache::Cache;
use app::app;
use failsafe::Config;
use service::Service;
use tokio::select;

mod app;
mod cache;
mod error;
mod service;
mod tracing;

type Result<T> = std::result::Result<T, error::Error>;
type LinksServiceClient = links_service_client::client::Client;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cache = Cache::from_connection_string("redis://localhost:6379")?;
    let client =
        LinksServiceClient::create("http://localhost:8080/api").unwrap();
    let circuit_breaker = Config::new().build();
    let service = Service::new(cache, client, circuit_breaker);
    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();
    let app = app(listener, service)?;

    select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down...");
        }
        _ = tokio::spawn(app) => {
            println!("Server stopped");
        }
    }

    Ok(())
}
