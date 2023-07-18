use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateCourse {}

pub async fn create_course(
    Json(_course_data): Json<CreateCourse>,
) -> (StatusCode, Json<CreateCourse>) {
    (StatusCode::OK, Json(CreateCourse {}))
}
