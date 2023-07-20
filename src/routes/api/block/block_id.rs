use crate::errors::AppError;
use crate::routes::api::auth::callback::Session;
use crate::schema::study_block::dsl::study_block;
use crate::schema::study_block::{id, user_id};
use crate::ServerState;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Extension;
use diesel::{BoolExpressionMethods, ExpressionMethods, RunQueryDsl};
use std::sync::Arc;

pub async fn delete_block(
    Path(_id): Path<String>,
    Extension(state): Extension<Arc<ServerState>>,
    Extension(session): Extension<Arc<Session>>,
) -> Result<(), AppError> {
    let con = &mut state.get_db_con()?;

    let rows = diesel::delete(study_block)
        .filter(id.eq(_id).and(user_id.eq(session.id.clone())))
        .execute(con)
        .or_else(|_e| {
            Err(AppError {
                status_code: StatusCode::BAD_REQUEST,
                description: "Failed to delete study block.".to_string(),
            })
        })?;

    if rows == 0 {
        return Err(AppError {
            status_code: StatusCode::BAD_REQUEST,
            description: "No study block by that ID exists.".to_string(),
        });
    }

    Ok(())
}
