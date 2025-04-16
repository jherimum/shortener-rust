use std::net::TcpListener;
use rest::app;
use failsafe::Config;
use key_generator::remote::RemoteKeyGenerator;
use key_generator_service_client::KgsClient;
use tokio::select;

mod key_generator;
mod rest;
mod storage;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8182").unwrap();
    let storage = storage::MemoryStorage::new();
    let client = KgsClient::new().await.unwrap();
    let cb = Config::new().build();
    let keys = RemoteKeyGenerator::new(client, cb);
    let actix_app = app(listener, storage, keys).unwrap();

    select! {
        _ = tokio::spawn(actix_app) => {
            println!("Server stopped");
        }

    }
}
