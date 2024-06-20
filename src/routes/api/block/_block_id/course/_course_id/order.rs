use std::collections::HashMap;
use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::Path;
use diesel::{
    Connection, ExpressionMethods, QueryDsl,
    RunQueryDsl, update,
};

use crate::errors::{AppError, AppResult};
use crate::routes::api::block::_block_id::course::course_id::get_course;
use crate::routes::api::users::me::GetUserCourse;
use crate::schema::course_component::{course_id, id, sequence_number};
use crate::schema::course_component::dsl::course_component;
use crate::ServerState;

pub async fn update_course_component_order(
    Path((_block_id, _course_id)): Path<(String, String)>,
    Extension(state): Extension<Arc<ServerState>>,
    Json(_component_data): Json<HashMap<String, i16>>,
) -> AppResult<Json<GetUserCourse>> {
    let con = &mut state.get_db_con()?;

    if _component_data.len() > 100 {
        return AppError::bad_request("Cannot update less than 0 or more than 100 components at once.").into()
    }

    let all_components: i64 = course_component
        .filter(course_id.eq(&_course_id))
        .count()
        .get_result(con)
        .map_err(|e| AppError::database_ise(e))?;

    let mut proposed_sequence: Vec<i16> = _component_data.values().cloned().collect();
    let required_sequence: Vec<i16> = (1..(all_components as i16)+1).collect();
    if proposed_sequence.len() != required_sequence.len() {
        return AppError::bad_request("Must update all components at the same time.").into()
    }
    proposed_sequence.sort();
    if required_sequence != proposed_sequence {
        return AppError::bad_request("Update must contain a valid sequence.").into()
    }

    let update_sequence_transaction = con.transaction(|txn| {
        for (component_id, new_sequence_number) in _component_data {
            match update(course_component)
                .filter(id.eq(&component_id))
                .filter(course_id.eq(&course_id))
                .set(sequence_number.eq(new_sequence_number))
                .execute(txn) {
                Ok(_) => {},
                Err(e) => return Err(e)
            }
        }
        Ok(0)
    });
    if update_sequence_transaction.is_err() {
        return Err(AppError::database_ise(update_sequence_transaction.unwrap_err()))
    }

    get_course(Path((_block_id, _course_id)), Extension(state)).await
}