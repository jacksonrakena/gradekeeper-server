use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct UpdateCourseComponent{}

pub async fn update_course_component(Json(_component_data): Json<UpdateCourseComponent>) -> StatusCode {
    StatusCode::OK
}