use tokio::select;

mod generator;
mod grpc;
mod key;

#[tokio::main]
async fn main() {
    let addr = "[::1]:50051".parse().unwrap();
    let server = grpc::server(addr);

    select! {
        _ = tokio::spawn(server) => {
            println!("Server stopped");
        }
    }
}
