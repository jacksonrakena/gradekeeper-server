use axum::extract::Path;
use axum::http::StatusCode;

pub(crate) async fn import_block(Path(id): Path<String>) -> (StatusCode) {
    StatusCode::OK
}