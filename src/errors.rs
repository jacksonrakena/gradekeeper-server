use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use diesel::result::Error;
use log::error;
use serde::Serialize;
use serde_json::json;

pub struct AppError {
    pub(crate) status_code: StatusCode,
    pub(crate) description: String
}
impl AppError {
    pub fn resource_access_denied() -> AppError {
        AppError { status_code: StatusCode::UNAUTHORIZED, description: "You are not allowed to access that resource.".to_string()}
    }
    pub fn database_ise(e: Error) -> AppError {
        error!("Database error: {}",e);
        AppError::unspecified_ise()
    }
    pub fn unspecified_ise() -> AppError {
        AppError { status_code: StatusCode::INTERNAL_SERVER_ERROR, description: "There was an error. Please try again later.".to_string()}
    }
    pub fn resource_not_found() -> AppError {
        AppError { status_code: StatusCode::NOT_FOUND, description: "That resource was not found.".to_string()}
    }
}

impl<T> Into<Result<T, AppError>> for AppError {
    fn into(self) -> Result<T, AppError> {
        Err(self)
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code, Json(json![{
            "error_type": self.status_code.as_u16(),
            "error_description": self.description
        }])).into_response()
    }
}