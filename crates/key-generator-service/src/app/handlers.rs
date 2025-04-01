use std::sync::Arc;
use actix_web::{web, HttpResponse};
use crate::generator::KeyGenerator;

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn available_keys(
    generator: web::Data<Arc<dyn KeyGenerator>>,
) -> HttpResponse {

    


    HttpResponse::Ok().finish()
}
