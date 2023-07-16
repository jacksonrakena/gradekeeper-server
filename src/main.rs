mod models;
mod schema;

use std::collections::HashMap;
use std::env;
use std::iter::Map;
use axum::{Json, Router, routing::{get, post, delete}, response::IntoResponse,};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use diesel::{Connection, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use diesel::associations::{BelongsTo, HasTable};
use dotenvy::dotenv;
use log::{info, LevelFilter};
use serde::{Deserialize, Serialize};
use pretty_env_logger::env_logger::{Builder, Target};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use time::OffsetDateTime;
use diesel::prelude::*;
use diesel::query_dsl::methods::GroupByDsl;
use crate::models::{Course, CourseComponent, CourseSubcomponent, StudyBlock, User};
use crate::schema::course::dsl::course;
use crate::schema::course::studyBlockId;
use crate::schema::course_component::dsl::course_component;
use crate::schema::course_component::subjectId;
use crate::schema::course_subcomponent::componentId;
use crate::schema::course_subcomponent::dsl::course_subcomponent;
use crate::schema::study_block::dsl::study_block;

#[tokio::main]
async fn main() {
    Builder::new()
        .filter_module(stringify!(gk-server), LevelFilter::Info)
        .target(Target::Stdout)
        .init();
    info!("Gradekeeper Nova server starting");

    let app = Router::new()
        // Users
        .route("/users/me", get(get_user))
        // Blocks
        .route("/block/create", post(block_create))
        .route("/block/:block_id", get(get_block))
        .route("/block/:block_id", axum::routing::delete(delete_block))
        .route("/block/:block_id/import", post(import_course))

        // Courses
        .route("/block/:block_id/course/create", post(create_course))
        .route("/block/:block_id/course/:course_id", get(get_course))
        .route("/block/:block_id/course/:course_id", axum::routing::delete(delete_course))
        .route("/block/:block_id/course/:course_id", post(update_course))

        // Components
        .route("/block/:block_id/course/:course_id/component/:component_id", post(update_course_component));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
struct GetUser {
    #[serde(flatten)]
    user: User,
    study_blocks: Vec<GetUserStudyBlock>
}
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
struct GetUserStudyBlock {
    #[serde(flatten)]
    study_block: StudyBlock,
    #[serde(rename = "subjects")]
    courses: Vec<GetUserCourse>
}
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
struct GetUserCourse {
    #[serde(flatten)]
    course: crate::models::Course,

    components: Vec<GetUserComponent>
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
struct GetUserComponent {
    #[serde(flatten)]
    component: crate::models::CourseComponent,

    subcomponents: Vec<crate::models::CourseSubcomponent>
}
async fn load_study_blocks(user: &User) -> Result<Vec<GetUserStudyBlock>, String> {
    let con = &mut establish_connection();
    use diesel::prelude::*;

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

    for c in &courses {
        if !study_blocks.clone().into_iter().any(|s|s.id == c.study_block_id) {
            println!("course has no block: {:#?}", c);
        }
    }
    for comp in &components {
        if !courses.clone().into_iter().any(|c|c.id==comp.subject_id) {
            println!("component has no course: {:#?}",comp);
        }
    }
    for subcomp in &subcomponents {
        if !components.clone().into_iter().any(|comp|comp.id==subcomp.component_id) {
            println!("subcomponent has no component: {:#?}",subcomp);
        }
    }
    let grup =
        courses
            .grouped_by(&study_blocks)
            .into_iter()
            .zip(study_blocks)
            .map(|(vcourses, block)| {
                GetUserStudyBlock {
                    study_block: block,
                    courses: components
                        .clone()
                        .grouped_by(&vcourses)
                        .into_iter()
                        .zip(vcourses)
                        .map(|(vcomponents, ncourse)| {
                            println!("course {} has {:#?}", ncourse.id, vcomponents);
                            GetUserCourse {
                                course: ncourse,
                                components: subcomponents
                                    .clone()
                                    .grouped_by(&vcomponents)
                                    .into_iter()
                                    .zip(vcomponents)
                                    .map(move |(subcomponents,ncomponent)| {
                                        GetUserComponent {
                                            component: ncomponent,
                                            subcomponents
                                        }
                                    })
                                    .collect()
                            }
                        })
                        .collect()
                }
            })
            .collect();

    Ok(grup)


}
async fn get_user() -> Result<Json<GetUser>, String> {
    let con = &mut establish_connection();
    use diesel::prelude::*;
    let user = schema::user::dsl::user
        .find("jackson.rakena@gmail.com")
        .select(User::as_select())
        .first(con)
        .unwrap();

    let Ok(blocks) = load_study_blocks(&user).await else { return Err("no".to_string()); };

    println!("{}",user.id);
    Ok(Json(GetUser {
        user,
        study_blocks: blocks,
    }))
}
#[derive(Serialize)]
struct GetCourse {

}
#[derive(Deserialize)]
struct UpdateCourse{}
async fn update_course(Path((block_id, course_id)): Path<(String, String)>, Json(update_course): Json<UpdateCourse>) -> StatusCode {
    StatusCode::NOT_FOUND
}
async fn delete_course(Path((block_id, course_id)): Path<(String, String)>) -> StatusCode {
    StatusCode::NOT_FOUND
}
async fn get_course(Path((block_id, course_id)): Path<(String, String)>) -> (StatusCode, Json<GetCourse>) {
    (StatusCode::OK, Json(GetCourse{}))
}
#[derive(Deserialize, Serialize)]
struct CreateBlock{
}
#[derive(Deserialize)]
struct UpdateCourseComponent{}

async fn update_course_component(Json(component_data): Json<UpdateCourseComponent>) -> (StatusCode) {
StatusCode::OK
}
async fn create_course(Json(course_data): Json<CreateBlock>) -> (StatusCode, Json<CreateBlock>) {
    (StatusCode::OK, Json(CreateBlock{}))
}
async fn import_course(Path(id): Path<String>) -> (StatusCode) {
    StatusCode::OK
}
async fn get_block(Path(id): Path<String>) -> (StatusCode, String) {
    (StatusCode::OK, id)
}
async fn delete_block(Path(id): Path<String>) -> (StatusCode) {
    StatusCode::OK
}
async fn block_create(Json(payload): Json<CreateBlock>) -> (StatusCode, Json<CreateBlock>){
    (StatusCode::CREATED, Json(CreateBlock{}))
}
