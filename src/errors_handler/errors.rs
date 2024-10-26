use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("Internal Server Error")]
    InternalServerError
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::InternalServerError => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: self.to_string(),
                })
            }
        }
    }
}
