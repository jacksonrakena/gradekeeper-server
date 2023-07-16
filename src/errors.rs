use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub enum AppErrorType {
    UnknownServerError,
    ResourceNotFound,
    NotAuthorized,
    NotLoggedIn,
    InvalidRequest
}
pub struct AppError {
    pub(crate) name: AppErrorType,
    pub(crate) status_code: StatusCode,
    pub(crate) description: String
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code, Json(json![{
            "error_type": self.name,
            "error_description": self.description
        }])).into_response()
    }
}