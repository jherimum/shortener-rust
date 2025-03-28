use std::net::TcpListener;
use actix_web::{
    dev::Server,
    get, http,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use storage_service::Client;
use tap::TapFallible;
use crate::{cache::Cache, Result};

#[derive(Clone)]
pub struct AppState {
    pub cache: Cache,
    pub client: Client,
}

pub fn server(listener: TcpListener, app_state: AppState) -> Result<Server> {
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(health)
            .service(redirect)
    })
    .listen(listener)?
    .run())
}

#[get("/l/{id}")]
async fn redirect(
    id: web::Path<String>,
    state: Data<AppState>,
) -> impl Responder {
    let id = id.into_inner();
    match retrieve_link(&id, &state.cache, &state.client).await {
        Ok(Some(link)) => HttpResponse::Found()
            .append_header((http::header::LOCATION, link.to_string()))
            .finish(),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            tracing::error!(
                "Failed to retrieve original url for id {}: {}",
                id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn retrieve_link(
    id: &str,
    cache: &Cache,
    client: &Client,
) -> Result<Option<String>> {
    if let Some(url) = cache.get_link(id).await? {
        return Ok(Some(url));
    }

    if let Some(link) = client.find_link(id).await.tap_err(|e| {
        tracing::error!("Failed to retrieve Link from storage: {e}")
    })? {
        cache
            .store_link(&link.short_id, &link.original_url, 10)
            .await?;
        return Ok(Some(link.original_url.clone()));
    }

    Ok(None)
}
