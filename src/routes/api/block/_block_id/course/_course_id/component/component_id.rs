use axum::extract::Path;
use std::sync::Arc;

use axum::{Extension, Json};
use bigdecimal::{BigDecimal, One, Zero};

use diesel::{update, AsChangeset, BelongingToDsl, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::errors::AppError;
use crate::models::{CourseComponent, CourseSubcomponent};
use diesel::result::Error::QueryBuilderError;
use serde::Deserialize;

use crate::routes::api::users::me::GetUserComponent;
use crate::schema::course_component::dsl::course_component;
use crate::schema::course_component::id;

use crate::schema::course_subcomponent::dsl::course_subcomponent;
use crate::{schema, ServerState};

#[derive(Deserialize, AsChangeset)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name=crate::schema::course_component)]
pub struct UpdateCourseComponentChangeset {
    pub name: Option<String>,
    pub name_of_subcomponent_singular: Option<String>,
    pub subject_weighting: Option<BigDecimal>,
    #[serde(rename = "numberOfSubComponentsToDrop_Lowest")]
    pub number_of_subcomponents_to_drop_lowest: Option<i32>
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCourseComponent {
    #[serde(flatten)]
    pub changeset: UpdateCourseComponentChangeset,
    pub subcomponents: Option<Vec<UpdateCourseSubcomponentChangeset>>,
}

#[derive(Deserialize,AsChangeset)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name=crate::schema::course_subcomponent)]
pub struct UpdateCourseSubcomponentChangeset {
    id: String,
    component_id: String,
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
                for new_subcomponent in new_subcomponents {
                    if new_subcomponent.component_id != _component_id {
                        return Err(AppError::resource_access_denied())
                    }
                    if new_subcomponent.grade_value_percentage.gt(&BigDecimal::one()) {
                        return Err(AppError::bad_request("Can't set a score higher than 100%."))
                    }
                    if new_subcomponent.grade_value_percentage.lt(&BigDecimal::zero()) {
                        return Err(AppError::bad_request("Can't set a score lower than zero."));
                    }
                    update(course_subcomponent.filter(schema::course_subcomponent::id.eq(&new_subcomponent.id))).set(&new_subcomponent).execute(txn)?;
                }
            }
        }

        match update(course_component.filter(id.eq(&_component_id)))
            .set(&_component_data.changeset)
            .execute(txn)
        {
            Ok(_) | Err(QueryBuilderError(_)) => {
                let component = course_component
                    .filter(id.eq(&_component_id))
                    .select(CourseComponent::as_select())
                    .get_result(txn)?;
                let subcomponents = CourseSubcomponent::belonging_to(&component)
                    .select(CourseSubcomponent::as_select())
                    .get_results(txn)?;

                Ok(Json(GetUserComponent {
                    component,
                    subcomponents,
                }))
            }
            Err(e) => Err(AppError::database_ise(e)),
        }
    })
}
