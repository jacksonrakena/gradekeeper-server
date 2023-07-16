use axum::extract::Path;
use axum::http::StatusCode;

pub(crate) async fn import_block(Path(_id): Path<String>) -> StatusCode {
    StatusCode::OK
}