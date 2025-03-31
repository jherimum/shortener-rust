use std::net::TcpListener;
use actix_web::{
    dev::Server,
    http::{self},
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use failsafe::futures::CircuitBreaker;
use log::info;
use tracing::instrument;
use crate::{service::Service, Result};

pub fn app<CB: CircuitBreaker + Clone + Send + Sync + 'static>(
    listener: TcpListener,
    service: Service<CB>,
) -> Result<Server> {
    info!(" Starting server at {}", listener.local_addr().unwrap());
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(service.clone()))
            .service(web::resource("/health").route(web::get().to(health)))
            .service(
                web::resource("/l/{id}").route(web::get().to(redirect::<CB>)),
            )
    })
    .listen(listener)?
    .run())
}

#[instrument(name = "redirect", skip_all)]
async fn redirect<CB: CircuitBreaker>(
    id: web::Path<String>,
    service: Data<Service<CB>>,
) -> impl Responder {
    let id = id.into_inner();
    match service.retrieve_link(&id).await {
        Ok(Some(link)) => HttpResponse::Found()
            .append_header((http::header::LOCATION, link.to_string()))
            .finish(),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            log::error!("Failed to retrieve original url for id {}: {}", id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[instrument(name = "health", skip_all)]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
