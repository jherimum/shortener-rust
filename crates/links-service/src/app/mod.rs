use std::{io, net::TcpListener};
use actix_web::{
    dev::Server,
    web::{self, scope},
    App, HttpServer, Scope,
};
use handler::{create_link, get_link, health};
use model::{ApiError, ApiResponse};
use crate::{key_generator::KeyGenerator, storage::Storage};

mod handler;
mod model;

type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

pub fn app<S, KG>(
    listener: TcpListener,
    storage: S,
    key_generator: KG,
) -> Result<Server, io::Error>
where
    S: Storage + Clone + Send + Sync + 'static,
    KG: KeyGenerator + Clone + Send + Sync + 'static,
{
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(key_generator.clone()))
            .app_data(web::Data::new(storage.clone()))
            .service(api::<S, KG>())
            .service(web::resource("/health").route(web::get().to(health)))
    })
    .listen(listener)?
    .run())
}

fn api<S: Storage + 'static, KG: KeyGenerator + 'static>() -> Scope {
    web::scope("/api").service(
        scope("/v1")
            .service(
                web::resource("/links")
                    .route(web::post().to(create_link::<S, KG>)),
            )
            .service(
                web::resource("/links/{link_id}")
                    .route(web::get().to(get_link::<S>)),
            ),
    )
}
