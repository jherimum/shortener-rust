use std::{io, net::TcpListener};

use actix_web::{dev::Server, App, HttpServer};
use tokio::net::tcp;

pub async fn server(listener: TcpListener) -> Result<Server, io::Error> {
    Ok(HttpServer::new(|| App::new()).listen(listener)?.run())
}
