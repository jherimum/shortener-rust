use std::fmt::Display;
use actix_web::{
    body::BoxBody, http::StatusCode, HttpResponse, Responder, ResponseError,
};
use serde::Serialize;
use crate::storage;

pub enum ApiResponse<R> {
    Created(R),
    Ok(Option<R>),
}

impl<R: Serialize> Responder for ApiResponse<R> {
    type Body = BoxBody;

    fn respond_to(
        self,
        _: &actix_web::HttpRequest,
    ) -> HttpResponse<Self::Body> {
        match self {
            ApiResponse::Created(body) => HttpResponse::Created().json(body),
            ApiResponse::Ok(None) => HttpResponse::Ok().finish(),
            ApiResponse::Ok(Some(body)) => HttpResponse::Ok().json(body),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ApiError {
    NotFound(String),
    InternalServerError(String),
}

impl From<storage::Error> for ApiError {
    fn from(value: storage::Error) -> Self {
        match value {
            storage::Error::SqlxError(error) => {
                ApiError::InternalServerError(error.to_string())
            }
            storage::Error::ShortLinkalteradyExists(short_link_id) => {
                ApiError::InternalServerError(short_link_id.to_owned())
            }
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NotFound(s) => write!(f, "NotFound: {}", s),
            ApiError::InternalServerError(s) => {
                write!(f, "InternalServerError: {}", s)
            }
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::InternalServerError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut r = HttpResponse::build(self.status_code());

        match self {
            ApiError::NotFound(s) => r.json(s),
            ApiError::InternalServerError(s) => r.json(s),
        }
    }
}
