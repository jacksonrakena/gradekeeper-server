use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use diesel::result::Error;
use log::error;
use std::convert::From;

use serde_json::json;

pub type AppResult<R> = Result<R, AppError>;

#[derive(Debug)]
pub struct AppError {
    pub(crate) status_code: StatusCode,
    pub(crate) description: String,
}
impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        match value {
            Error::NotFound => AppError::resource_not_found(),
            _ => AppError::database_ise(value),
        }
    }
}
impl AppError {
    pub fn invalid_redirect_url(redirect_url: String) -> AppError {
        AppError {
            status_code: StatusCode::BAD_REQUEST,
            description: format!("Redirect URL '{}' is not allowed.", redirect_url),
        }
    }
    pub fn resource_access_denied() -> AppError {
        AppError {
            status_code: StatusCode::UNAUTHORIZED,
            description: "You are not allowed to access that resource.".to_string(),
        }
    }
    pub fn database_ise(e: Error) -> AppError {
        error!("Database error: {}", e);
        AppError::unspecified_ise()
    }
    pub fn unspecified_ise() -> AppError {
        AppError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            description: "There was an error. Please try again later.".to_string(),
        }
    }
    pub fn resource_not_found() -> AppError {
        AppError {
            status_code: StatusCode::NOT_FOUND,
            description: "That resource was not found.".to_string(),
        }
    }
    pub fn bad_request<D: ToString>(message: D) -> AppError {
        AppError {
            status_code: StatusCode::BAD_REQUEST,
            description: message.to_string(),
        }
    }
}

impl<T> Into<Result<T, AppError>> for AppError {
    fn into(self) -> Result<T, AppError> {
        Err(self)
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(json![{
                "type": self.status_code.as_u16(),
                "error": self.description
            }]),
        )
            .into_response()
    }
}
