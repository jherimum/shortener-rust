use actix_web::web;
use serde::Deserialize;
use crate::{
    key_generator::KeyGenerator,
    storage::{ShortLink, Storage},
};
use super::{
    model::{ApiError, ApiResponse},
    ApiResult,
};

pub async fn health() -> ApiResult<()> {
    ApiResult::Ok(ApiResponse::Ok(None))
}

pub async fn create_link<S: Storage, KG: KeyGenerator>(
    request: web::Json<CreteLinkRequest>,
    storage: web::Data<S>,
    keys: web::Data<KG>,
) -> ApiResult<ShortLink> {
    let key = keys.create().await.unwrap();
    storage
        .create_link(&key, &request.original_url, None)
        .await
        .map_err(ApiError::from)
        .map(|l| Ok(ApiResponse::Ok(Some(l))))?
}

pub async fn get_link<S: Storage>(
    short_link_id: web::Path<String>,
    storage: web::Data<S>,
) -> ApiResult<ShortLink> {
    match storage.get_link(&short_link_id).await.unwrap() {
        Some(l) => Ok(ApiResponse::Ok(Some(l))),
        None => Err(ApiError::NotFound("Link not found".to_owned())),
    }
}

#[derive(Deserialize)]
pub struct CreteLinkRequest {
    pub original_url: String,
}
