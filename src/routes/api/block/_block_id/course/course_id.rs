use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use diesel::{
    delete, update, AsChangeset, BelongingToDsl, ExpressionMethods, GroupedBy, QueryDsl,
    RunQueryDsl, SelectableHelper,
};
use std::sync::Arc;

use crate::errors::{AppError, AppResult};
use crate::models::{Course, CourseComponent, CourseSubcomponent};
use crate::routes::api::users::me::{GetUserComponent, GetUserCourse};
use crate::schema::course::dsl::course;
use crate::schema::course::id;
use crate::ServerState;
use serde::Deserialize;

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name=crate::schema::course)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCourse {
    pub long_name: Option<String>,
    pub course_code_name: Option<String>,
    pub course_code_number: Option<String>,
}

pub async fn update_course(
    Path((_block_id, _course_id)): Path<(String, String)>,
    Extension(state): Extension<Arc<ServerState>>,
    Json(_update_course): Json<UpdateCourse>,
) -> AppResult<Json<GetUserCourse>> {
    let con = &mut state.get_db_con()?;

    update(course.filter(id.eq(&_course_id)))
        .set(_update_course)
        .execute(con)?;

    return get_course(Path((_block_id, _course_id)), Extension(state)).await;
}

pub async fn delete_course(
    Path((_block_id, _course_id)): Path<(String, String)>,
    Extension(state): Extension<Arc<ServerState>>,
) -> Result<Response, AppError> {
    let con = &mut state.get_db_con()?;
    let result = delete(course.filter(id.eq(_course_id)))
        .execute(con)
        .or_else(|e| AppError::database_ise(e).into())?;

    (result == 1)
        .then(|| StatusCode::OK.into_response())
        .ok_or(AppError::resource_not_found())
}
pub async fn get_course(
    Path((_block_id, _course_id)): Path<(String, String)>,
    Extension(state): Extension<Arc<ServerState>>,
) -> AppResult<Json<GetUserCourse>> {
    let con = &mut state.get_db_con()?;

    let selected_course = course
        .filter(id.eq(_course_id))
        .select(Course::as_select())
        .get_result(con)?;
    let course_components: Vec<CourseComponent> = CourseComponent::belonging_to(&selected_course)
        .select(CourseComponent::as_select())
        .load(con)?;

    Ok(Json(GetUserCourse {
        course: selected_course,
        components: CourseSubcomponent::belonging_to(&course_components)
            .select(CourseSubcomponent::as_select())
            .load(con)?
            .grouped_by(&course_components)
            .into_iter()
            .zip(course_components)
            .map(|(sub, comp)| GetUserComponent {
                component: comp,
                subcomponents: sub,
            })
            .collect::<Vec<GetUserComponent>>(),
    }))
}
