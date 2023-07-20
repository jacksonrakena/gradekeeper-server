use std::sync::Arc;

use axum::{Extension, Json};
use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::OffsetDateTime;

use crate::errors::AppError;
use crate::models::{Course, CourseComponent, CourseSubcomponent, StudyBlock, User};
use crate::routes::api::auth::callback::Session;
use crate::schema::gk_user::dsl::gk_user;
use crate::schema::gk_user::{grade_map, id};
use crate::ServerState;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUser {
    grade_map: serde_json::Value,
    study_blocks: Vec<GetUserStudyBlock>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserStudyBlock {
    #[serde(flatten)]
    study_block: StudyBlock,
    #[serde(rename = "subjects")]
    courses: Vec<GetUserCourse>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserCourse {
    #[serde(flatten)]
    pub course: Course,

    pub components: Vec<GetUserComponent>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserComponent {
    #[serde(flatten)]
    pub component: crate::models::CourseComponent,

    pub subcomponents: Vec<crate::models::CourseSubcomponent>,
}

pub async fn get_user<B>(
    Extension(user_session): Extension<Arc<Session>>,
    Extension(state): Extension<Arc<ServerState>>,
    _req: axum::http::Request<B>,
) -> Result<Json<GetUser>, AppError> {
    let con = &mut state.get_db_con()?;

    match gk_user
        .find(user_session.id.clone())
        .select(User::as_select())
        .first(con)
    {
        Ok(user) => {
            let study_blocks = StudyBlock::belonging_to(&user)
                .select(StudyBlock::as_select())
                .load(con)?;
            let courses = Course::belonging_to(&study_blocks)
                .select(Course::as_select())
                .load(con)?;
            let components = CourseComponent::belonging_to(&courses)
                .select(CourseComponent::as_select())
                .load(con)?;
            let subcomponents = CourseSubcomponent::belonging_to(&components)
                .select(CourseSubcomponent::as_select())
                .load(con)?;

            Ok(Json(GetUser {
                grade_map: user.grade_map,
                study_blocks: study_blocks
                    .into_iter()
                    .map(|s| GetUserStudyBlock {
                        study_block: s.clone(),
                        courses: courses
                            .clone()
                            .into_iter()
                            .filter(|x| x.block_id == s.id)
                            .map(|c| GetUserCourse {
                                course: c.clone(),
                                components: components
                                    .clone()
                                    .into_iter()
                                    .filter(|component| component.course_id == c.id)
                                    .map(|component| GetUserComponent {
                                        component: component.clone(),
                                        subcomponents: subcomponents
                                            .clone()
                                            .into_iter()
                                            .filter(|subc| subc.component_id == component.id)
                                            .collect(),
                                    })
                                    .collect(),
                            })
                            .collect(),
                    })
                    .collect(),
            }))
        }
        Err(diesel::NotFound) => {
            let user = User {
                id: user_session.id.clone(),
                grade_map: json!({
                  "0.4": "D",
                  "0.5": "C-",
                  "0.6": "C+",
                  "0.7": "B",
                  "0.8": "A-",
                  "0.9": "A+",
                  "0.55": "C",
                  "0.65": "B-",
                  "0.75": "B+",
                  "0.85": "A",
                }),
                created_at: OffsetDateTime::now_utc(),
            };
            insert_into(gk_user).values(&user).execute(con)?;
            return Ok(Json(GetUser {
                grade_map: user.grade_map,
                study_blocks: vec![],
            }));
        }
        Err(e) => return AppError::database_ise(e).into(),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    pub grade_map: serde_json::Value,
}

pub async fn update_user(
    Extension(user_session): Extension<Arc<Session>>,
    Extension(state): Extension<Arc<ServerState>>,
    Json(data): Json<UpdateUser>,
) -> Result<StatusCode, AppError> {
    let con = &mut state.get_db_con()?;

    let result = update(gk_user.filter(id.eq(&user_session.id)))
        .set(grade_map.eq(data.grade_map))
        .execute(con)?;

    match result {
        1 => Ok(StatusCode::OK),
        _ => Err(AppError::bad_request("Couldn't find a user to update.")),
    }
}

pub async fn delete_user(
    Extension(user_session): Extension<Arc<Session>>,
    Extension(state): Extension<Arc<ServerState>>,
) -> Result<StatusCode, AppError> {
    let con = &mut state.get_db_con()?;

    let result = delete(gk_user.filter(id.eq(&user_session.id))).execute(con)?;

    match result {
        1 => Ok(StatusCode::OK),
        _ => Err(AppError::bad_request("Couldn't find a user to delete.")),
    }
}
