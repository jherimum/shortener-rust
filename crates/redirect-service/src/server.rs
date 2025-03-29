use std::net::TcpListener;
use actix_web::{
    dev::Server,
    get,
    http::{self},
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use log::info;
use links_service::{Client, LinkModel};
use tap::TapFallible;
use tracing::instrument;
use crate::{cache::Cache, Result};

#[derive(Clone)]
pub struct AppState {
    pub cache: Cache,
    pub client: Client,
}

pub fn server(listener: TcpListener, app_state: AppState) -> Result<Server> {
    info!(" Starting server at {}", listener.local_addr().unwrap());
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
#[instrument(name = "redirect", skip_all)]
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
            log::error!("Failed to retrieve original url for id {}: {}", id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[instrument(name = "retrieve_link", skip(cache, client))]
async fn retrieve_link(
    id: &str,
    cache: &Cache,
    client: &Client,
) -> Result<Option<String>> {
    if let Some(url) = cache.get_link(id).await? {
        return Ok(Some(url));
    }

    let link = client.find_link(id).await.tap_err(|e| {
        log::error!("Failed to retrieve Link from storage: {e}")
    })?;

    // let link = Some(LinkModel {
    //     id: 1,
    //     short_id: "abcdefg".to_owned(),
    //     original_url: "http://www.terra.com.br".to_owned(),
    // });

    if let Some(link) = link {
        cache
            .store_link(&link.short_id, &link.original_url, 10)
            .await?;
        return Ok(Some(link.original_url.clone()));
    }

    Ok(None)
}
