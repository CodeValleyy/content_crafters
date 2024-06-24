use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde_json::json;

#[derive(Debug, Display)]
pub enum UploadError {
    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
}

impl ResponseError for UploadError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            UploadError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(json!({ "message": message }))
            }
            UploadError::InternalServerError => HttpResponse::InternalServerError()
                .json(json!({ "message": "Internal Server Error" })),
        }
    }
}
