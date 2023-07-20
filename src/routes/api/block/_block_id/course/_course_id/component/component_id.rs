use std::sync::Arc;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use bigdecimal::BigDecimal;
use cuid2::cuid;
use diesel::{AsChangeset, BelongingToDsl, Connection, delete, ExpressionMethods, insert_into, QueryDsl, RunQueryDsl, select, SelectableHelper, update};
use serde::Deserialize;
use crate::errors::AppError;
use crate::models::{CourseComponent, CourseSubcomponent};
use crate::routes::api::block::_block_id::course::create::CreateCourseComponent;
use crate::routes::api::users::me::GetUserComponent;
use crate::schema::course_component::dsl::course_component;
use crate::schema::course_component::{id, name};
use crate::schema::course_subcomponent::component_id;
use crate::schema::course_subcomponent::dsl::course_subcomponent;
use crate::ServerState;

#[derive(Deserialize, AsChangeset)]
#[serde(rename_all="camelCase")]
#[diesel(table_name=crate::schema::course_component)]
pub struct UpdateCourseComponentChangeset {

    pub name: Option<String>,
    pub name_of_subcomponent_singular: Option<String>,
    pub subject_weighting: Option<BigDecimal>,
    #[serde(rename="numberOfSubComponentsToDrop_Lowest")]
    pub number_of_subcomponents_to_drop_lowest: Option<i32>,
}
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct UpdateCourseComponent {
    #[serde(flatten)]
    pub changeset: UpdateCourseComponentChangeset,
    pub subcomponents: Option<Vec<UpdateCourseSubcomponent>>
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct UpdateCourseSubcomponent {
    number_in_sequence: i32,
    override_name: Option<String>,
    is_completed: bool,
    grade_value_percentage: BigDecimal,
}

pub async fn update_course_component(
    Path((_block_id, _course_id, _component_id)): Path<(String, String, String)>,
    Extension(state): Extension<Arc<ServerState>>,
    Json(_component_data): Json<UpdateCourseComponent>,
) -> Result<Json<GetUserComponent>, AppError> {
    let con = &mut state.get_db_con()?;

    con.transaction(|txn| {
        match _component_data.subcomponents {
            None => {}
            Some(new_subcomponents) => {
                delete(course_subcomponent.filter(component_id.eq(&_component_id))).execute(txn)?;

                insert_into(course_subcomponent).values(new_subcomponents.into_iter().map(|new_subcomponent|CourseSubcomponent{
                    id: cuid(),
                    component_id: _component_id.clone(),
                    grade_value_percentage: new_subcomponent.grade_value_percentage,
                    is_completed: new_subcomponent.is_completed,
                    number_in_sequence: new_subcomponent.number_in_sequence,
                    override_name: new_subcomponent.override_name,
                }).collect::<Vec<CourseSubcomponent>>()).execute(txn)?;
            }
        }

        update(course_component.filter(id.eq(&_component_id))).set(&_component_data.changeset).execute(txn)?;
        let component = course_component.filter(id.eq(&_component_id)).select(CourseComponent::as_select()).get_result(txn)?;
        let subcomponents = CourseSubcomponent::belonging_to(&component).select(CourseSubcomponent::as_select()).get_results(txn)?;

        Ok(Json(GetUserComponent{ component, subcomponents }))
    }).map_err(|e|AppError::database_ise(e))
}
