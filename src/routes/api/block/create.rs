use axum::http::StatusCode;
use axum::{Extension, Json};
use std::sync::Arc;

use crate::errors::AppError;
use crate::models::StudyBlock;
use crate::routes::api::auth::callback::Session;
use crate::schema::study_block::dsl::study_block;
use crate::ServerState;
use diesel::{insert_into, QueryDsl, RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub async fn create_block(
    Extension(user): Extension<Arc<Session>>,
    Extension(state): Extension<Arc<ServerState>>,
    Json(payload): Json<CreateBlock>,
) -> Result<Json<StudyBlock>, AppError> {
    let con = &mut state.db_pool.get().unwrap();
    let block = StudyBlock {
        end_date: payload.end_date,
        start_date: payload.start_date,
        id: cuid2::create_id(),
        name: payload.name,
        user_id: user.id.clone(),
    };

    insert_into(study_block)
        .values(&block)
        .execute(con)
        .or_else(|e| {
            Err(AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                description: format!("Could not create study block: {}", e),
            })
        })?;

    Ok(Json(block))
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBlock {
    #[serde(with = "time::serde::rfc3339")]
    pub end_date: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub start_date: OffsetDateTime,
    pub name: String,
}
