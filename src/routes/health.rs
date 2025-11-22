use crate::errors::AppError;
use crate::routes::api::users::{gather_meta_info, ServerMetaInfo};
use crate::ServerState;
use axum::{Extension, Json};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponseDatabase {
    connections: u32,
    idle_connections: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponse {
    status: String,
    database: HealthResponseDatabase,
    meta: ServerMetaInfo,
}

pub async fn health_status(
    Extension(state): Extension<Arc<ServerState>>,
) -> Result<Json<HealthResponse>, AppError> {
    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        database: HealthResponseDatabase {
            connections: state.db_pool.state().connections,
            idle_connections: state.db_pool.state().idle_connections,
        },
        meta: gather_meta_info(),
    }))
}
