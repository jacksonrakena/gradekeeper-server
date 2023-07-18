use axum::extract::Path;
use axum::{Extension, Json};
use std::sync::Arc;

use bigdecimal::{BigDecimal, Zero};
use cuid2::cuid;
use diesel::{insert_into, BelongingToDsl, GroupedBy, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::errors::AppError;
use crate::models::{Course, CourseComponent, CourseSubcomponent};
use crate::routes::api::auth::callback::Session;
use serde::Deserialize;

use crate::schema::course::dsl::course;
use crate::schema::course_component::dsl::course_component;
use crate::schema::course_subcomponent::dsl::course_subcomponent;
use crate::ServerState;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportCourse {
    pub share_code: String,
}
pub(crate) async fn import_course(
    Path(block_id): Path<String>,
    Extension(state): Extension<Arc<ServerState>>,
    Extension(_session): Extension<Arc<Session>>,
    Json(course_request): Json<ImportCourse>,
) -> Result<Json<Course>, AppError> {
    let con = &mut state.db_pool.get().unwrap();
    let original_course = course
        .find(&course_request.share_code)
        .select(Course::as_select())
        .first(con)
        .or_else(|_| AppError::resource_not_found().into())?;

    let original_components = CourseComponent::belonging_to(&original_course)
        .select(CourseComponent::as_select())
        .load(con)
        .or_else(|_| AppError::unspecified_ise().into())?;

    let original_subcomponents = CourseSubcomponent::belonging_to(&original_components)
        .select(CourseSubcomponent::as_select())
        .load(con)
        .or_else(|_| AppError::unspecified_ise().into())?
        .grouped_by(&original_components)
        .into_iter()
        .zip(original_components)
        .map(|(subcomponents, component)| (component, subcomponents))
        .collect::<Vec<(CourseComponent, Vec<CourseSubcomponent>)>>();

    let new_course_id = cuid();
    let new_course = Course {
        id: new_course_id.clone(),
        long_name: original_course.long_name.clone(),
        course_code_name: original_course.course_code_name.clone(),
        course_code_number: original_course.course_code_number.clone(),
        block_id,
        color: original_course.color.clone(),
    };

    let mut components: Vec<CourseComponent> = vec![];
    let mut subcomponents: Vec<CourseSubcomponent> = vec![];
    for (c, split_subcomponent) in original_subcomponents {
        let component_id = cuid();
        let component = CourseComponent {
            id: component_id.clone(),
            name: c.name.clone(),
            name_of_subcomponent_singular: c.name_of_subcomponent_singular.clone(),
            number_of_subcomponents_to_drop_lowest: c.number_of_subcomponents_to_drop_lowest,
            course_id: new_course_id.clone(),
            subject_weighting: c.subject_weighting,
        };
        components.push(component);
        for subcomponent in split_subcomponent {
            subcomponents.push(CourseSubcomponent {
                id: cuid(),
                component_id: component_id.clone(),
                grade_value_percentage: BigDecimal::zero(),
                is_completed: false,
                number_in_sequence: subcomponent.number_in_sequence,
                override_name: subcomponent.override_name,
            })
        }
    }

    insert_into(course)
        .values(&new_course)
        .execute(con)
        .unwrap();
    insert_into(course_component)
        .values(components)
        .execute(con)
        .unwrap();
    insert_into(course_subcomponent)
        .values(subcomponents)
        .execute(con)
        .unwrap();

    Ok(Json(new_course))
}
