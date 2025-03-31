use std::net::TcpListener;

use app::app;
use failsafe::Config;
use key_generator_service_client::Client;
use key_generator::{remote::RemoteKeyGenerator, KeyGenerator};
use tokio::select;

mod app;
mod key_generator;
mod storage;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8182").unwrap();
    let storage = storage::MemoryStorage::new();
    let client = Client::new("".parse().unwrap());
    let cb = Config::new().build();
    let keys = RemoteKeyGenerator::new(client, cb);
    let app = app(listener, storage, keys).unwrap();

    select! {
        _ = tokio::spawn(app) => {
            println!("Server stopped");
        }

    }
}
