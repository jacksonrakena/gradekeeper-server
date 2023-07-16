use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct GetCourse {

}
#[derive(Deserialize)]
pub struct UpdateCourse{}

pub async fn update_course(Path((block_id, course_id)): Path<(String, String)>, Json(update_course): Json<UpdateCourse>) -> StatusCode {
    StatusCode::NOT_FOUND
}
pub async fn delete_course(Path((block_id, course_id)): Path<(String, String)>) -> StatusCode {
    StatusCode::NOT_FOUND
}
pub async fn get_course(Path((block_id, course_id)): Path<(String, String)>) -> (StatusCode, Json<GetCourse>) {
    (StatusCode::OK, Json(GetCourse{}))
}