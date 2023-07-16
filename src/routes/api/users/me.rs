use std::collections::HashMap;
use std::sync::Arc;
use axum::{Extension, Json};
use axum::http::{Request, StatusCode};
use diesel::prelude::*;
use serde::Serialize;
use crate::errors::{AppError, AppErrorType};
use crate::models::{StudyBlock, Course, User, CourseComponent, CourseSubcomponent};
use crate::routes::api::auth::callback::Session;
use crate::ServerState;

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct GetUser {
    grade_map: HashMap<String, String>,
    study_blocks: Vec<GetUserStudyBlock>
}
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct GetUserStudyBlock {
    #[serde(flatten)]
    study_block: StudyBlock,
    #[serde(rename = "subjects")]
    courses: Vec<GetUserCourse>
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct GetUserCourse {
    #[serde(flatten)]
    course: Course,

    components: Vec<GetUserComponent>
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct GetUserComponent {
    #[serde(flatten)]
    component: crate::models::CourseComponent,

    subcomponents: Vec<crate::models::CourseSubcomponent>
}
pub async fn get_user<B>(Extension(user_session): Extension<Arc<Session>>, Extension(state): Extension<Arc<ServerState>>, req: axum::http::Request<B>) -> Result<Json<GetUser>, AppError> {
    let con = &mut state.db_pool.get().unwrap();

    let Ok(user) = crate::schema::user::dsl::user
        .find(user_session.id.clone())
        .select(User::as_select())
        .first(con) else { return Err(AppError {
            name: AppErrorType::UnknownServerError,
            status_code: StatusCode::NOT_FOUND,
            description: format!("User not found."),
    }) };

    let study_blocks = StudyBlock::belonging_to(&user)
        .select(StudyBlock::as_select())
        .load(con).unwrap();
    let courses = Course::belonging_to(&study_blocks)
        .select(Course::as_select())
        .load(con)
        .unwrap();
    let components = CourseComponent::belonging_to(&courses)
        .select(CourseComponent::as_select())
        .load(con)
        .unwrap();
    let subcomponents = CourseSubcomponent::belonging_to(&components)
        .select(CourseSubcomponent::as_select())
        .load(con)
        .unwrap();

    Ok(Json(GetUser {
        grade_map: serde_json::from_str(user.grade_map.as_str()).unwrap(),
        study_blocks: study_blocks.into_iter().map(|s|{
            GetUserStudyBlock {
                study_block: s.clone(),
                courses: courses
                    .clone()
                    .into_iter()
                    .filter(|x|x.study_block_id==s.id)
                    .map(|c|{
                        GetUserCourse{
                            course: c.clone(),
                            components: components
                                .clone()
                                .into_iter()
                                .filter(|component|component.subject_id==c.id)
                                .map(|component|GetUserComponent {
                                    component: component.clone(),
                                    subcomponents: subcomponents
                                        .clone()
                                        .into_iter()
                                        .filter(|subc|subc.component_id==component.id)
                                        .collect()
                                })
                                .collect()
                        }
                    })
                    .collect()
            }
        }).collect()
    }))
}