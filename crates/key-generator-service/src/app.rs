use std::{io, net::TcpListener, sync::Arc};
use handlers::{available_keys, health};
use crate::generator::KeyGenerator;
mod handlers;

pub async fn app(
    listener: TcpListener,
    generator: Arc<dyn KeyGenerator + Send + Sync>,
) -> Result<Server, io::Error> {
    Ok(HttpServer::new(move || {
        App::new()
            .service(api())
            .app_data(Data::new(web::Data::new(generator.clone())))
    })
    .listen(listener)?
    .run())
}

fn api() -> Scope {
    Scope::new("")
        .service(web::resource("/health").route(web::get()).to(health))
        .service(Scope::new("/api").service(Scope::new("/v1").service(
            web::resource("/keys").route(web::post().to(available_keys)),
        )))
}
