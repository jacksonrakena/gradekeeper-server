use std::ops::{Add, Div};
use std::sync::Arc;

use axum::extract::Path;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use bigdecimal::{BigDecimal, Zero};
use cuid2::cuid;
use diesel::{Connection, RunQueryDsl};

use crate::errors::AppError;
use crate::models::{Course, CourseComponent, CourseSubcomponent};
use serde::{Deserialize, Serialize};

use crate::schema::course::dsl::course;
use crate::schema::course_component::dsl::course_component;
use crate::schema::course_subcomponent::dsl::course_subcomponent;

use crate::ServerState;

#[derive(Serialize)]
pub struct CreateCourseResponse {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCourse {
    pub name: String,
    #[serde(rename = "codeName")]
    pub course_code_name: String,
    #[serde(rename = "codeNo")]
    pub course_code_number: String,
    pub color: String,

    pub components: Vec<CreateCourseComponent>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCourseComponent {
    pub drop_lowest: i32,
    pub weighting: BigDecimal,
    pub name: String,
    pub number_of_subcomponents: String,
}

fn validate(course_data: &CreateCourse) -> Result<(), AppError> {
    if course_data.components.iter()
        .map(|d|d.number_of_subcomponents.parse::<i32>().unwrap())
        .reduce(|a,b|a+b)
        .unwrap_or(0) > 100 {
        return Err(AppError::bad_request("Total number of subcomponents must not exceed 100."))
    }

    if course_data.components.iter()
        .map(|d|d.weighting.clone())
        .reduce(|a,b|a.add(b))
        .unwrap_or(BigDecimal::from(0)).ne(&BigDecimal::from(1)) {
        return Err(AppError::bad_request("Course components must add up to 100%."))
    }

    Ok(())
}

pub async fn create_course(
    Path(_block_id): Path<String>,
    Extension(state): Extension<Arc<ServerState>>,
    Json(course_data): Json<CreateCourse>,
) -> Result<Json<CreateCourseResponse>, AppError> {
    let con = &mut state.db_pool.get().unwrap();

    validate(&course_data)?;
    let id = con
        .transaction(|con| {
            let course_id = cuid();
            let new_course = Course {
                id: course_id.clone(),
                long_name: course_data.name,
                course_code_name: course_data.course_code_name,
                course_code_number: course_data.course_code_number,
                block_id: _block_id,
                color: course_data.color,
            };

            let mut new_components: Vec<CourseComponent> = vec![];
            let mut new_subcomponents: Vec<CourseSubcomponent> = vec![];
            for component in course_data.components {
                let new_component_id = cuid();
                let new_component = CourseComponent {
                    id: new_component_id.clone(),
                    name: component.name,
                    course_id: course_id.clone(),
                    subject_weighting: component.weighting,
                    number_of_subcomponents_to_drop_lowest: component.drop_lowest,
                    name_of_subcomponent_singular: "".to_string(),
                };
                let n_subc = component.number_of_subcomponents.parse::<i32>().unwrap();
                for i in 1..(n_subc + 1) {
                    let new_subcomponent = CourseSubcomponent {
                        id: cuid(),
                        component_id: new_component_id.clone(),
                        grade_value_percentage: BigDecimal::zero(),
                        is_completed: false,
                        number_in_sequence: i,
                        override_name: None,
                    };
                    new_subcomponents.push(new_subcomponent);
                }
                new_components.push(new_component);
            }

            diesel::insert_into(course)
                .values(&new_course)
                .execute(con)?;
            diesel::insert_into(course_component)
                .values(&new_components)
                .execute(con)?;
            diesel::insert_into(course_subcomponent)
                .values(&new_subcomponents)
                .execute(con)?;
            diesel::result::QueryResult::Ok(course_id)
        })
        .unwrap();

    Ok(Json(CreateCourseResponse { id }))
}
