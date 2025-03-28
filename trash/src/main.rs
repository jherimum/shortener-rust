use std::net::TcpListener;
use app::App;
use tokio::select;

pub mod app;
pub mod shorten;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:9090")?;
    let app = App::new(listener)?;

    select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Shutting down1");
        }
        _ =  tokio::spawn(app.start()) => {
            println!("Server stopped");
        }
    }

    Ok(())
}
