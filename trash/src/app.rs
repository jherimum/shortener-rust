use std::net::TcpListener;
use actix_web::{dev::Server, get, post, HttpServer, Responder};
use tokio::io;

pub struct App {
    server: Server,
}

impl App {
    pub fn new(listener: TcpListener) -> io::Result<App> {
        let server = HttpServer::new(move || {
            actix_web::App::new()
                .service(health)
                .service(create_link)
                .service(redirect_to_link)
        })
        .listen(listener)?
        .run();
        Ok(Self { server })
    }

    pub async fn start(self) -> io::Result<()> {
        self.server.await
    }
}

#[get("/health")]
async fn health() -> impl Responder {
    "OK1"
}

#[post("/api/v1/links")]
async fn create_link() -> impl Responder {
    "OK111"
}

#[get("/l/{id}")]
async fn redirect_to_link() -> impl Responder {
    "OK"
}
