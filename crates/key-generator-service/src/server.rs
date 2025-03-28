use std::{io, net::TcpListener, sync::Arc};
use actix_web::{dev::Server, get, post, web::Data, App, HttpResponse, HttpServer};

use crate::generator::KeyGenerator;

#[derive(Clone)]
pub struct AppState {
    pub generator: Arc<dyn KeyGenerator + Send + Sync>,
}

pub async fn server(
    listener: TcpListener,
    state: AppState,
) -> Result<Server, io::Error> {
    Ok(HttpServer::new(move || {
        App::new()
            .service(health)
            .service(create_key)
            .app_data(Data::new(state.clone()))
    })
    .listen(listener)?
    .run())
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/keys")]
async fn create_key() -> HttpResponse {
    HttpResponse::Ok().finish()
}
