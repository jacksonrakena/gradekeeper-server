use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize,Serialize};


pub async fn create_block(Json(payload): Json<CreateBlock>) -> (StatusCode, Json<CreateBlock>){
    (StatusCode::CREATED, Json(CreateBlock{}))
}

#[derive(Deserialize, Serialize)]
pub struct CreateBlock{
}