use axum::{ body::Body, http::{Response, StatusCode}, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct Error {
    pub status_code: u16,
    pub message: String,
    pub data: Option<Value>,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl Error {
    pub fn new(status: StatusCode, message: String, data: Option<Value>) -> (StatusCode, Response<Body>) {
        let error = Response::from(Error {
            status_code: status.as_u16(),
            data,
            message
        }.into_response());
    
        (
            status,
            error
        )
    }
}