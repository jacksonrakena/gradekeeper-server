use axum::extract::Path;
use axum::http::StatusCode;

pub async fn get_block(Path(id): Path<String>) -> (StatusCode, String) {
    (StatusCode::OK, id)
}

pub async fn delete_block(Path(_id): Path<String>) -> StatusCode {
    StatusCode::OK
}